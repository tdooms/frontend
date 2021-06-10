use seed::{prelude::*, *};
use std::rc::Rc;

pub struct Button<Msg> {
    text: String,
    style: String,
    icon: String,

    large: bool,
    outlined: bool,
    disabled: bool,
    hidden: bool,

    on_click: Rc<dyn Fn() -> Msg>,
}

impl<Msg: 'static> Button<Msg> {
    pub fn create(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("create", "is-success", "fa-plus", on_click)
    }

    pub fn confirm(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("confirm", "is-success", "fa-check", on_click)
    }

    pub fn delete(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("delete", "is-danger", "fa-times", on_click)
    }

    pub fn trash(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("", "is-danger", "fa-trash", on_click)
    }

    pub fn save(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("", "is-success", "fa-save", on_click)
    }

    pub fn play(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("play", "is-success", "fa-play", on_click)
    }

    pub fn stop(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("stop", "is-danger", "fa-square", on_click)
    }

    pub fn pause(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("pause", "is-light", "fa-pause", on_click)
    }

    pub fn resume(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("resume", "is-light", "fa-play", on_click)
    }

    pub fn next(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("next", "is-success", "fa-angle-double-right", on_click)
    }

    pub fn custom(
        text: impl Into<String>,
        style: impl Into<String>,
        icon: impl Into<String>,
        on_click: impl Fn() -> Msg + Clone + 'static,
    ) -> Self {
        Button {
            text: text.into(),
            style: style.into(),
            icon: icon.into(),
            large: false,
            outlined: false,
            disabled: false,
            hidden: false,
            on_click: Rc::new(on_click),
        }
    }

    pub fn view(&self) -> Node<Msg> {
        let func = self.on_click.clone();
        button![
            C![
                "button",
                &self.style,
                IF!(self.large => "is-large"),
                IF![self.outlined => "is-outlined"]
            ],
            ev(Ev::Click, move |_| func()),
            attrs! {At::Disabled => self.disabled.as_at_value()},
            IF!(self.hidden => style! {St::Display => "none"}),
            span![
                C!["icon", IF![!self.large => "is-small"]],
                i![C!["fas", &self.icon]]
            ],
            IF!(!self.text.is_empty() => span![&self.text])
        ]
    }

    pub fn disable(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn hide(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn outline(mut self, outlined: bool) -> Self {
        self.outlined = outlined;
        self
    }

    pub fn large(mut self) -> Self {
        self.large = true;
        self
    }

    pub fn disabled(self) -> Self {
        self.disable(true)
    }

    pub fn hidden(self) -> Self {
        self.hide(true)
    }

    pub fn outlined(self) -> Self {
        self.outline(true)
    }
}
