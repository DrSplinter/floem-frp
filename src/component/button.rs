use std::sync::{Arc, Mutex};

use floem::IntoView;
use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::Stream;

use crate::Component;

pub fn button<A: Component + 'static>(component: A) -> Button<A> {
    Button::new(component)
}

#[derive(Clone)]
pub struct Button<A> {
    listeners: Arc<Mutex<Vec<UnboundedSender<()>>>>,
    component: A,
}

impl<A: Component + 'static> Button<A> {
    pub fn new(component: A) -> Self {
        let listeners = Arc::new(Mutex::new(Vec::new()));

        Self {
            listeners,
            component,
        }
    }

    pub fn clicks(&self) -> impl Stream<Item = ()> {
        let (sender, receiver) = unbounded();
        self.listeners.lock().unwrap().push(sender);

        receiver
    }
}

impl<A: Component + 'static> Component for Button<A> {
    fn view(&self) -> floem::AnyView {
        floem::views::button(self.component.view())
            .action({
                let listeners = self.listeners.clone();
                move || {
                    listeners
                        .lock()
                        .unwrap()
                        .retain(|listener| listener.unbounded_send(()).is_ok());
                }
            })
            .into_any()
    }
}
