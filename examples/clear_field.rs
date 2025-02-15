use floem_frp::{component, Component};
use futures::StreamExt;

pub fn component() -> impl Component {
    let button = component::button("Clear");
    let clears = button.clicks().map(|_| "");
    let text_input = component::text_input("Hello World", clears);

    component::v_stack((text_input, button))
}

#[tokio::main]
async fn main() {
    component::show(component());
}
