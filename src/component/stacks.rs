use floem::IntoView;

use crate::Component;

pub fn v_stack<A: ComponentTuple>(components: A) -> VerticalStack<A> {
    VerticalStack::new(components)
}

#[derive(Clone)]
pub struct VerticalStack<Cs> {
    components: Cs,
}

impl<Cs: ComponentTuple> VerticalStack<Cs> {
    pub fn new(components: Cs) -> Self {
        Self { components }
    }
}

pub fn h_stack<A: ComponentTuple>(components: A) -> HorizontalStack<A> {
    HorizontalStack::new(components)
}

#[derive(Clone)]
pub struct HorizontalStack<Cs> {
    components: Cs,
}

impl<Cs: ComponentTuple> HorizontalStack<Cs> {
    pub fn new(components: Cs) -> Self {
        Self { components }
    }
}

pub trait ComponentTuple {
    fn v_stack(self) -> VerticalStack<Self>
    where
        Self: Sized,
    {
        VerticalStack::new(self)
    }

    fn h_stack(self) -> HorizontalStack<Self>
    where
        Self: Sized,
    {
        HorizontalStack::new(self)
    }
}

impl<A: Component> Component for VerticalStack<Vec<A>> {
    fn view(&self) -> floem::AnyView {
        let components = self.components.iter().map(|c| c.view());

        floem::views::v_stack_from_iter(components).into_any()
    }
}
impl<A: Component> Component for HorizontalStack<Vec<A>> {
    fn view(&self) -> floem::AnyView {
        let components = self.components.iter().map(|c| c.view());

        floem::views::h_stack_from_iter(components).into_any()
    }
}
impl<A: Component> ComponentTuple for Vec<A> {}

impl<A: Component, const N: usize> Component for VerticalStack<[A; N]> {
    fn view(&self) -> floem::AnyView {
        let components = self.components.iter().map(|c| c.view());

        floem::views::v_stack_from_iter(components).into_any()
    }
}
impl<A: Component, const N: usize> Component for HorizontalStack<[A; N]> {
    fn view(&self) -> floem::AnyView {
        let components = self.components.iter().map(|c| c.view());

        floem::views::h_stack_from_iter(components).into_any()
    }
}
impl<A: Component, const N: usize> ComponentTuple for [A; N] {}

macro_rules! impl_stack {
    ($($name:ident),+) => {
        impl<$($name: Component),+> ComponentTuple for ($($name,)+) {}

        impl<$($name: Component + Clone + 'static),+> Component for VerticalStack<($($name,)+)> {
            fn view(&self) -> floem::AnyView {
                #[allow(non_snake_case)]
                let ( $($name,)+ ) = self.components.clone();

                floem::views::v_stack(($($name.view(),)+)).into_any()
            }
        }

        impl<$($name: Component + Clone + 'static),+> Component for HorizontalStack<($($name,)+)> {
            fn view(&self) -> floem::AnyView {
                #[allow(non_snake_case)]
                let ( $($name,)+ ) = self.components.clone();

                floem::views::h_stack(($($name.view(),)+)).into_any()
            }
        }
    }
}

impl_stack!(A);
impl_stack!(A, B);
impl_stack!(A, B, C);
impl_stack!(A, B, C, D);
impl_stack!(A, B, C, D, E);
impl_stack!(A, B, C, D, E, F);
impl_stack!(A, B, C, D, E, F, G);
impl_stack!(A, B, C, D, E, F, G, H);
impl_stack!(A, B, C, D, E, F, G, H, I);
impl_stack!(A, B, C, D, E, F, G, H, I, J);
impl_stack!(A, B, C, D, E, F, G, H, I, J, K);
impl_stack!(A, B, C, D, E, F, G, H, I, J, K, L);
