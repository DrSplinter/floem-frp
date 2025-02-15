use floem_frp::{component, ArcComponent, Component, ComponentTuple};
use frp::stream::{FrpStreamExt, MergeStreamTuple};
use futures::StreamExt;

mod clear_field;
mod counter;
mod echo_label;
mod greetings_label;
mod reversed_label;
mod translate;

fn examples(iter: impl IntoIterator<Item = (&'static str, ArcComponent)>) -> impl Component {
    let (buttons, component_streams): (Vec<_>, Vec<_>) = iter
        .into_iter()
        .map(|(name, component)| {
            let button = component::button(name);
            let component_stream = button.clicks().map(move |_| component.clone());

            (button, component_stream)
        })
        .unzip();
    let selected_component = component::dynamic(
        component_streams
            .merge()
            .to_signal(component::empty().arced()),
    );

    component::v_stack((selected_component, buttons.h_stack()))
}

pub fn component() -> impl Component {
    examples([
        ("Clear field", clear_field::component().arced()),
        ("Echo label", echo_label::component().arced()),
        ("Greetings label", greetings_label::component().arced()),
        ("Reversed label", reversed_label::component().arced()),
        ("Translate", translate::component().arced()),
        ("Counter", counter::component().arced()),
    ])
}

#[tokio::main]
async fn main() {
    component::show(component());
}
