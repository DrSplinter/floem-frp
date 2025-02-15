use floem::{prelude::SignalGet, IntoView};
use frp::signal::FrpSignalExt;
use futures_signals::signal::{ReadOnlyMutable, Signal, SignalExt};

use crate::{ArcComponent, Component, FloemSignalExt};

pub fn dynamic<S>(component: S) -> Dynamic
where
    S: Signal + Send + 'static,
    S::Item: Component + Send + Sync + 'static,
{
    Dynamic::new(component.map(|c| c.arced()))
}

#[derive(Clone)]
pub struct Dynamic {
    component: ReadOnlyMutable<ArcComponent>,
}

impl Dynamic {
    pub fn new<S>(component: S) -> Self
    where
        S: Signal<Item = ArcComponent> + Send + 'static,
    {
        Self {
            component: component.materialize(),
        }
    }
}

impl Component for Dynamic {
    fn view(&self) -> floem::AnyView {
        let component = self.component.signal_cloned().into_floem_read_signal();

        floem::views::dyn_view(move || component.get().view()).into_any()
    }
}
