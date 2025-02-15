use floem_frp::{component, Component};
use frp::stream::{FrpStreamExt, MergeStreamTuple};
use futures::StreamExt;

pub fn component() -> impl Component {
    let thank_you_button = component::button("Thank You");
    let danke_button = component::button("Danke");

    let thank_yous = thank_you_button.clicks().map(|_| "Thank You");
    let dankes = danke_button.clicks().map(|_| "Danke");
    let text = (thank_yous, dankes).merge().to_signal("");

    let label = component::label(text);
    let buttons = component::h_stack((thank_you_button, danke_button));

    component::v_stack((label, buttons))
}

#[tokio::main]
async fn main() {
    component().show();
}
