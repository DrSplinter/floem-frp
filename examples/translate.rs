use floem_frp::{component, Component};
use frp::stream::FrpStreamExt;
use futures::{stream, StreamExt};
use futures_signals::signal::SignalExt;

fn translate(s: String) -> String {
    s.split_whitespace()
        .map(|s| s.to_string() + "us")
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn component() -> impl Component {
    let text_input = component::text_input("", stream::pending::<String>());
    let translate_button = component::button("Translate to latin");

    let translated = text_input
        .text()
        .sample_stream_cloned(translate_button.clicks())
        .map(|(text, ())| translate(text));
    let translated_text = translated.to_signal("".to_string());

    let translation_label = component::label(translated_text);

    component::v_stack((text_input, translate_button, translation_label))
}

#[tokio::main]
async fn main() {
    component::show(component());
}
