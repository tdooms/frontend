use crate::field::{Field, EMPTY};
use seed::{prelude::*, *};
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum State<T> {
    Empty,
    Value(T),
    Error(String),
}

pub type Validator<T> = Box<dyn Fn(&str) -> Result<T, String>>;

pub enum Msg {
    Blur,
    Value(String),
}

pub struct InputField<T: ToString + Clone + PartialEq + Debug> {
    label: String,
    placeholder: String,

    value: String,
    initial: Option<T>,

    validator: Validator<T>,

    untouched: bool,
    optional: bool,
}

impl<T: ToString + Clone + PartialEq + Debug> InputField<T> {
    pub fn new(
        label: impl Into<String>,
        validator: impl Fn(&str) -> Result<T, String> + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            placeholder: String::new(),
            value: String::new(),
            initial: None,
            validator: Box::new(validator),
            untouched: true,
            optional: false,
        }
    }

    pub fn initial(mut self, initial: T) -> Self {
        self.value = initial.to_string();
        self.initial = Some(initial);
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl InputField<String> {
    pub fn string(label: impl Into<String>) -> Self {
        Self::new(label, |x| Ok(x.to_owned()))
    }
}

impl InputField<f64> {
    pub fn f64(label: impl Into<String>) -> Self {
        Self::new(label, |x| {
            x.parse::<f64>()
                .map_err(|_| "Must be valid number.".to_owned())
        })
    }
}

impl<T: ToString + Clone + PartialEq + Debug> Field for InputField<T> {
    type Msg = Msg;
    type Value = Option<T>;

    fn update(&mut self, msg: Self::Msg, _: &mut impl Orders<Self::Msg>) -> bool {
        self.untouched = false;

        self.value = match msg {
            Msg::Value(str) => str,
            Msg::Blur => return false,
        };
        true
    }

    fn reset(&mut self) {
        self.value = match &self.initial {
            Some(x) => x.to_string(),
            None => String::new(),
        }
    }

    fn value(&self) -> Self::Value {
        (!self.value.is_empty())
            .then(|| ())
            .and_then(|_| (self.validator)(&self.value).ok())
    }

    fn has_changed(&self) -> bool {
        self.value
            != match &self.initial {
                Some(x) => x.to_string(),
                None => String::new(),
            }
    }

    fn view(&self, disabled: bool) -> Node<Self::Msg> {
        let (danger, error) = match (self.untouched, self.optional, self.value.is_empty()) {
            (true, _, _) | (_, true, true) => (false, String::new()),
            (false, false, true) => (true, EMPTY.to_owned()),
            (false, _, false) => match (self.validator)(&self.value) {
                Ok(_) => (false, String::new()),
                Err(err) => (true, err),
            },
        };

        div![
            C!["field"],
            label![
                C!["label"],
                &self.label,
                IF!(self.optional => i![C!["has-text-grey"], " - Optional"])
            ],
            div![
                C!["control"],
                IF!(danger => C!["has-icons-right"]),
                IF!(!danger => C!["mb-2"]), // use 5 for minimal error drift
                input![
                    C!["input", IF!(danger => "is-danger")],
                    input_ev(Ev::Input, |str| Msg::Value(str)),
                    ev(Ev::Blur, |_| Msg::Blur),
                    attrs! {At::Placeholder => &self.placeholder},
                    IF!(disabled => attrs! {At::Disabled => ""}),
                ],
                IF![danger => span![C!["icon is-small is-right"], i![C!["fas", "fa-exclamation-triangle"]]]],
            ],
            p![C!["help", "is-danger"], error],
        ]
    }
}
