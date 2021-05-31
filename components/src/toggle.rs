use crate::field::Field;
use seed::{prelude::*, *};

pub struct ToggleField {
    label: String,
    value: bool,
    initial: bool,
}

impl ToggleField {
    pub fn new(label: impl Into<String>, initial: bool) -> Self {
        Self {
            label: label.into(),
            value: initial,
            initial,
        }
    }
}

impl Field for ToggleField {
    type Msg = ();
    type Value = bool;

    fn update(&mut self, _: Self::Msg, _: &mut impl Orders<Self::Msg>) -> bool {
        self.value = !self.value;
        true
    }

    fn reset(&mut self) {
        self.value = self.initial
    }

    fn value(&self) -> Self::Value {
        self.value
    }

    fn has_changed(&self) -> bool {
        self.value != self.initial
    }

    fn view(&self, _: bool) -> Node<Self::Msg> {
        label![
            C!["label"],
            div![
                C!["toggle"],
                input![
                    C!["toggle-state"],
                    attrs!(At::Type => "checkbox"),
                    input_ev(Ev::Input, |_| ())
                ],
                div![C!["toggle-inner"], div![C!["indicator"]]],
                div![C!["active-bg"]],
            ],
            div![C!["label-text"], &self.label]
        ]
    }
}
