use std::sync::Arc;

use floem::IntoView;

mod button;
mod dynamic;
mod empty;
mod label;
mod stacks;
mod text_input;

pub use button::*;
pub use dynamic::*;
pub use empty::*;
pub use label::*;
pub use stacks::*;
pub use text_input::*;

pub type ArcComponent = Arc<dyn Component + Send + Sync + 'static>;

pub fn show(component: impl Component + 'static) {
    floem::launch(move || component.view());
}

pub trait Component {
    fn view(&self) -> floem::AnyView;

    fn arced(self) -> ArcComponent
    where
        Self: Sized + Send + Sync + 'static,
    {
        Arc::new(self)
    }

    fn show(self)
    where
        Self: Sized + 'static,
    {
        show(self);
    }
}

impl<'a> Component for &'a str {
    fn view(&self) -> floem::AnyView {
        floem::IntoView::into_view(*self).into_any()
    }
}

impl Component for String {
    fn view(&self) -> floem::AnyView {
        floem::IntoView::into_view(self.clone()).into_any()
    }
}

impl Component for ArcComponent {
    fn view(&self) -> floem::AnyView {
        self.as_ref().view()
    }
}
