use seed::prelude::{Node, Orders};

pub trait Form {
    type Msg: 'static;
    type Value;

    fn update(&mut self, msg: Self::Msg, orders: &mut impl Orders<Self::Msg>) -> bool;
    fn reset(&mut self);

    fn value(&self) -> Self::Value;
    fn view(&self) -> Node<Self::Msg>;

    fn has_changed(&self) -> bool;
}
