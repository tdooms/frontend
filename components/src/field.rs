use seed::prelude::{Node, Orders};

pub static EMPTY: &str = "This field is required.";

pub trait Field {
    type Msg: 'static;
    type Value;

    fn update(&mut self, msg: Self::Msg, orders: &mut impl Orders<Self::Msg>) -> bool;
    fn reset(&mut self);

    fn value(&self) -> Self::Value;
    fn has_changed(&self) -> bool;

    fn view(&self, disabled: bool) -> Node<Self::Msg>;
}
