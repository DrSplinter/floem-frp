use floem::reactive::{ReadSignal, RwSignal, SignalGet, SignalUpdate};
use futures::{FutureExt, Stream, StreamExt};
use futures_signals::signal::{Signal, SignalExt};

pub(crate) trait FloemSignalExt: Signal {
    fn into_floem_read_signal(self) -> ReadSignal<Self::Item>
    where
        Self: Sized + Send + 'static,
    {
        into_floem_read_signal(self)
    }

    fn into_floem_rw_signal(self) -> (RwSignal<Self::Item>, impl Stream<Item = Self::Item>)
    where
        Self: Sized + Send + 'static,
        Self::Item: Clone + Send,
    {
        into_floem_rw_signal(self)
    }
}

impl<S: Signal> FloemSignalExt for S {}

fn into_floem_read_signal<S>(signal: S) -> ReadSignal<S::Item>
where
    S: Signal + Send + 'static,
{
    let (first, rest) = signal
        .to_stream()
        .boxed()
        .into_future()
        .now_or_never()
        .unwrap();

    floem::ext_event::create_signal_from_stream(first.unwrap(), rest)
}

fn into_floem_rw_signal<S>(input_signal: S) -> (RwSignal<S::Item>, impl Stream<Item = S::Item>)
where
    S: Signal + Send + 'static,
    S::Item: Clone + Send,
{
    let (first, mut rest) = input_signal
        .to_stream()
        .boxed()
        .into_future()
        .now_or_never()
        .unwrap();
    let cx = floem::reactive::Scope::new();
    let trigger = cx.create_trigger();

    let channel_closed = cx.create_rw_signal(false);
    let rw_signal = cx.create_rw_signal(first.unwrap());
    let data = std::sync::Arc::new(std::sync::Mutex::new(std::collections::VecDeque::new()));
    let (output_sender, output_stream) = futures::channel::mpsc::unbounded();

    {
        let data = data.clone();
        cx.create_effect(move |_| {
            trigger.track();
            let mut data = data.lock().unwrap();

            if let Some(value) = data.pop_back() {
                rw_signal.set(value);
            }
            data.clear();

            if channel_closed.get() {
                cx.dispose();
            }
        });
    }

    floem::reactive::create_updater(
        move || rw_signal.get(),
        move |value| {
            let _ = output_sender.unbounded_send(value);
        },
    );

    let send = floem::ext_event::create_ext_action(cx, move |_| {
        channel_closed.set(true);
    });

    tokio::task::spawn(async move {
        while let Some(event) = rest.next().await {
            data.lock().unwrap().push_back(event);
            floem::ext_event::register_ext_trigger(trigger);
        }
        send(());
    });

    (rw_signal, output_stream)
}
