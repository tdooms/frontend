use seed::{prelude::*, *};

pub struct Graph {
    id: String,
}

pub enum Msg {
    Resize,
}

impl Graph {
    pub fn new(id: impl Into<String>, orders: &mut impl Orders<Msg>) -> Self {
        orders.stream(streams::window_event(Ev::Resize, |_| Msg::Resize));

        Self { id: id.into() }
    }

    pub fn redraw() {}

    pub fn update(msg: Msg, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Resize => {}
        }
    }

    pub fn view(&self) -> Node<Msg> {
        canvas![attrs! {At::Id => &self.id}]
    }
}
