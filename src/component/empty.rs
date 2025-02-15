use floem::IntoView;

use crate::Component;

pub fn empty() -> Empty {
    Empty
}

pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self
    }
}

impl Component for Empty {
    fn view(&self) -> floem::AnyView {
        floem::views::empty().into_any()
    }
}
