use seed::{prelude::*, *};
use std::rc::Rc;

pub struct Button<Msg> {
    text: String,
    style: String,
    icon: String,

    large: bool,
    outlined: bool,
    light: bool,

    on_click: Rc<dyn Fn() -> Msg>,
}

impl<Msg: 'static> Button<Msg> {
    pub fn create(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("create", "is-success", "fa-plus", on_click).light()
    }

    pub fn confirm(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("confirm", "is-success", "fa-check", on_click).light()
    }

    pub fn delete(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("delete", "is-danger", "fa-times", on_click).light()
    }

    pub fn trash(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("", "is-danger", "fa-trash", on_click).light()
    }

    pub fn save(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("", "is-success", "fa-save", on_click).light()
    }

    pub fn play(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("play", "is-success", "fa-play", on_click).light()
    }

    pub fn stop(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("stop", "is-danger", "fa-square", on_click).light()
    }

    pub fn pause(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("pause", "", "fa-pause", on_click).light()
    }

    pub fn resume(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("resume", "", "fa-play", on_click).light()
    }

    pub fn next(on_click: impl Fn() -> Msg + Clone + 'static) -> Self {
        Self::custom("next", "is-success", "fa-angle-double-right", on_click).light()
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
            light: false,
            on_click: Rc::new(on_click),
        }
    }

    pub fn outlined(mut self) -> Self {
        self.outlined = true;
        self
    }

    pub fn large(mut self) -> Self {
        self.large = true;
        self
    }

    pub fn light(mut self) -> Self {
        self.light = true;
        self
    }

    fn view(&self, hidden: bool, disabled: bool) -> Node<Msg> {
        let func = self.on_click.clone();
        button![
            C![
                "button",
                &self.style,
                IF!(self.large => "is-large"),
                IF![self.outlined => "is-outlined"]
            ],
            ev(Ev::Click, move |_| func()),
            attrs! {At::Disabled => disabled.as_at_value()},
            IF!(hidden => style! {St::Display => "none"}),
            span![
                C!["icon", IF![!self.large => "is-small"]],
                i![C!["fas", &self.icon]]
            ],
            IF!(!self.text.is_empty() => span![&self.text])
        ]
    }

    pub fn disabled(&self, disabled: bool) -> Node<Msg> {
        self.view(false, disabled)
    }

    pub fn hidden(&self, hidden: bool) -> Node<Msg> {
        self.view(hidden, false)
    }

    pub fn shown(&self) -> Node<Msg> {
        self.view(false, false)
    }
}
