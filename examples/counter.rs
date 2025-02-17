use floem_frp::{component, Component};
use frp::stream;
use frp::stream::FrpStreamExt;
use futures::StreamExt;
use futures_signals::signal::SignalExt;

pub fn component() -> impl Component {
    let increment_button = component::button("+");
    let decrement_button = component::button("-");

    let increments = increment_button.clicks().map(|_| 1);
    let decrements = decrement_button.clicks().map(|_| -1);
    let deltas = stream::merge((increments, decrements));
    let counter = deltas.accumulate(0, |a, b| a + b);
    let counter_text = counter.map(|counter| format!("Counter is: {counter}"));

    let counter_label = component::label(counter_text);
    let buttons = component::h_stack((increment_button, decrement_button));

    component::v_stack((counter_label, buttons))
}

#[tokio::main]
async fn main() {
    component::show(component());
}
