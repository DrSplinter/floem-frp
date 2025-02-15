use floem_frp::{component, Component};
use futures::stream;
use futures_signals::signal::SignalExt;

fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
}

pub fn component() -> impl Component {
    let text_input = component::text_input("Hello", stream::pending::<String>());

    let reversed_text = text_input.text().map(reverse_string);

    let label = component::label(reversed_text);

    component::v_stack((text_input, label))
}

#[tokio::main]
async fn main() {
    component().show();
}
