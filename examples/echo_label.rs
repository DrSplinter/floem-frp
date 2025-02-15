use floem_frp::{component, Component};
use futures::stream;

pub fn component() -> impl Component {
    let text_input = component::text_input("Hello", stream::pending::<String>());
    let label = component::label(text_input.text());

    component::v_stack((text_input, label))
}

#[tokio::main]
async fn main() {
    component::show(component());
}
