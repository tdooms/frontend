use crate::field::Field;
use seed::{prelude::*, *};

pub struct SliderField {
    label: String,

    mapper: Box<dyn Fn(f64) -> f64>,

    value: f64,
    initial: f64,
}

impl SliderField {
    pub fn new(
        label: impl Into<String>,
        initial: f64,
        mapper: impl Fn(f64) -> f64 + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            mapper: Box::new(mapper),
            value: initial,
            initial,
        }
    }

    pub fn linear(label: impl Into<String>, initial: f64) -> Self {
        Self::new(label, initial, |x| x)
    }
}

impl Field for SliderField {
    type Msg = f64;
    type Value = f64;

    fn update(&mut self, msg: Self::Msg, _: &mut impl Orders<Self::Msg>) -> bool {
        // Only returns true if a new value has been assigned
        std::mem::replace(&mut self.value, msg) != msg
    }

    fn reset(&mut self) {
        self.value = self.initial
    }

    fn value(&self) -> Self::Value {
        (self.mapper)(self.value)
    }

    fn has_changed(&self) -> bool {
        self.value != self.initial
    }

    fn view(&self, _: bool) -> Node<Self::Msg> {
        div![
            C!["field"],
            label![C!["label"], &self.label],
            div![
                C!["control pb-4"],
                input![
                    C!["slider"],
                    attrs! { At::Min => 0, At::Max => 1, At::Step => 0.01 },
                    attrs! { At::Type => "range", At::Value => self.value },
                    input_ev(Ev::Input, |str| str.parse::<f64>().ok()),
                ],
                p![
                    style! {
                    St::Position => "absolute",
                    St::TextAlign => "center",
                    St::Left => format!("calc({}% - ({}px))", 100. * self.value, self.value * 25.)},
                    format!("{:.2}", (self.mapper)(self.value))
                ]
            ]
        ]
    }
}
