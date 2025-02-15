use floem::{reactive::SignalGet, IntoView};
use frp::signal::FrpSignalExt;
use futures_signals::signal::{ReadOnlyMutable, Signal, SignalExt};

use crate::{Component, FloemSignalExt};

pub fn label<S>(text: S) -> Label
where
    S: Signal + Send + 'static,
    S::Item: Into<String>,
{
    Label::new(text.map(Into::into))
}

#[derive(Clone)]
pub struct Label {
    text: ReadOnlyMutable<String>,
}

impl Label {
    pub fn new<S>(text: S) -> Self
    where
        S: Signal<Item = String> + Send + 'static,
    {
        Self {
            text: text.materialize(),
        }
    }

    pub fn text(&self) -> impl Signal<Item = String> {
        self.text.signal_cloned()
    }
}

impl Component for Label {
    fn view(&self) -> floem::AnyView {
        let text = self.text().into_floem_read_signal();

        floem::views::label(move || text.get()).into_any()
    }
}
