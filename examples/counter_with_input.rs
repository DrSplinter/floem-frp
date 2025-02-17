use floem_frp::{component, Component};
use frp::stream;
use futures::StreamExt;
use futures_signals::signal::SignalExt;

pub fn component() -> impl Component {
    let increment_button = component::button("+");
    let decrement_button = component::button("-");

    let counter_states = stream::placeholder::<i32>();
    let counter_text_input =
        component::text_input("0", counter_states.stream().map(|x| x.to_string()));

    let increments = increment_button.clicks().map(|_| 1);
    let decrements = decrement_button.clicks().map(|_| -1);
    let deltas = stream::merge((increments, decrements));
    let counter = counter_text_input
        .text()
        .map(|text| text.parse::<i32>().unwrap_or(0));
    counter_states.fill(
        counter
            .sample_stream_cloned(deltas)
            .map(|(counter, delta)| counter + delta),
    );

    let buttons = component::h_stack((increment_button, decrement_button));

    component::v_stack((counter_text_input, buttons))
}

#[tokio::main]
async fn main() {
    component::show(component());
}
