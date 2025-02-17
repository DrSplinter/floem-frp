use floem_frp::{component, ArcComponent, Component, ComponentTuple};
use frp::signal::FrpSignalExt;
use frp::stream;
use frp::stream::FrpStreamExt;
use futures::StreamExt;
use futures_signals::signal::{ReadOnlyMutable, Signal, SignalExt};

#[derive(Clone)]
struct Counter {
    counter: ReadOnlyMutable<i32>,

    component: ArcComponent,
}

impl Counter {
    fn new(init: i32) -> Self {
        let increment_button = component::button("+");
        let decrement_button = component::button("-");

        let increments = increment_button.clicks().map(|_| 1);
        let decrements = decrement_button.clicks().map(|_| -1);
        let deltas = stream::merge((increments, decrements));
        let counter = deltas.accumulate(init, |a, b| a + b).materialize();
        let counter_text = counter
            .signal()
            .map(|counter| format!("Counter is: {counter}"));

        let counter_label = component::label(counter_text);
        let buttons = component::h_stack((increment_button, decrement_button));
        let component = component::v_stack((counter_label, buttons)).arced();

        Self { counter, component }
    }

    pub fn value(&self) -> impl Signal<Item = i32> {
        self.counter.signal()
    }
}

impl Component for Counter {
    fn view(&self) -> floem::AnyView {
        self.component.view()
    }
}

pub fn component() -> impl Component {
    let counter = Counter::new(0);
    let label = component::label(
        counter
            .value()
            .map(|counter| format!("Counter of component is: {counter}")),
    );

    (counter, label).v_stack()
}

#[tokio::main]
async fn main() {
    component::show(component());
}
