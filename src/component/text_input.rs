use floem::reactive::SignalUpdate;
use floem::IntoView;
use frp::signal::FrpSignalExt;
use frp::stream::FrpStreamExt;
use futures::{Stream, StreamExt};
use futures_signals::signal::{Mutable, ReadOnlyMutable, Signal};

use crate::{Component, FloemSignalExt};

pub fn text_input<S, Str>(init: Str, changes: S) -> TextInput
where
    S: Stream + Send + 'static,
    S::Item: Into<String>,
    Str: Into<String>,
{
    TextInput::new(init.into(), changes.map(Into::into))
}

#[derive(Clone)]
pub struct TextInput {
    input_text: ReadOnlyMutable<String>,
    output_text: Mutable<String>,
}

impl TextInput {
    pub fn new<S>(init: String, changes: S) -> Self
    where
        S: Stream<Item = String> + Send + 'static,
    {
        let input_text = changes.to_signal(init.clone()).materialize();
        let output_text = Mutable::new(init.clone());

        Self {
            input_text,
            output_text,
        }
    }

    pub fn text(&self) -> impl Signal<Item = String> {
        self.output_text.signal_cloned()
    }
}

impl Component for TextInput {
    fn view(&self) -> floem::AnyView {
        let (text, changes) = self.input_text.signal_cloned().into_floem_rw_signal();
        let output_text = self.output_text.clone();

        text.set(output_text.get_cloned());

        tokio::spawn(changes.for_each(move |text| {
            output_text.set(text);
            futures::future::ready(())
        }));

        floem::views::text_input(text).into_any()
    }
}
