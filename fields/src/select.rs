use seed::{prelude::*, *};

use crate::Field;

pub enum Msg {
    Value(i64),
    Blur,
}

pub struct SelectField {
    label: String,
    default: Option<(i64, String)>,

    initial: Option<i64>,
    value: Option<i64>,

    suggestions: Vec<(i64, String)>,

    optional: bool,
    untouched: bool,
}

impl SelectField {
    pub fn new(label: impl Into<String>, suggestions: Vec<(i64, String)>) -> Self {
        Self {
            label: label.into(),
            default: None,
            initial: None,
            value: suggestions.first().map(|x| x.0),
            suggestions,
            optional: false,
            untouched: true,
        }
    }

    pub fn initial(mut self, initial: i64) -> Self {
        self.initial = Some(initial);
        self.value = Some(initial);
        self
    }

    pub fn default(mut self, id: i64, name: impl Into<String>, optional: bool) -> Self {
        self.default = Some((id, name.into()));
        self.optional = optional;
        self
    }

    fn view_option(&self, suggestion: &(i64, String)) -> Node<<Self as Field>::Msg> {
        let selected = self.initial == Some(suggestion.0);
        option![
            attrs! {At::Value => suggestion.0, At::Selected => selected.as_at_value()},
            &suggestion.1
        ]
    }

    fn view_options(&self) -> Vec<Node<<Self as Field>::Msg>> {
        self.default
            .iter()
            .chain(self.suggestions.iter())
            .map(|x| self.view_option(x))
            .collect()
    }
}

impl Field for SelectField {
    type Msg = Msg;
    type Value = Option<i64>;

    fn update(&mut self, msg: Self::Msg, _: &mut impl Orders<Self::Msg>) -> bool {
        self.untouched = false;

        self.value = match msg {
            Msg::Value(value) => self.suggestions.iter().map(|x| x.0).find(|&x| x == value),
            Msg::Blur => return false,
        };
        true
    }

    fn reset(&mut self) {
        self.value = self
            .default
            .as_ref()
            .map(|x| x.0)
            .or(self.suggestions.first().map(|x| x.0))
    }

    fn value(&self) -> Self::Value {
        self.value
    }

    fn has_changed(&self) -> bool {
        self.initial != self.value
    }

    fn view(&self, readonly: bool) -> Node<Self::Msg> {
        div![
            C!["field"],
            label![C!["label"], &self.label],
            div![
                C!["control"],
                div![
                    C![
                        "select",
                        IF!(self.value.is_none() && !self.optional => "is-danger")
                    ],
                    select![
                        IF!(readonly => attrs! {At::Disabled => ""}),
                        input_ev(Ev::Input, |str| str.parse::<i64>().ok().map(Msg::Value)),
                        ev(Ev::Blur, |_| Msg::Blur),
                        self.view_options()
                    ]
                ]
            ]
        ]
    }
}
