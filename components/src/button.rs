use seed::{prelude::*, *};
use std::marker::PhantomData;
use std::rc::Rc;

pub struct Button<Msg> {
    text: &'static str,
    style: &'static str,
    icon: &'static str,

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
        Self::custom("next", "is-success", "fa-play", on_click)
    }

    pub fn custom(
        text: &'static str,
        style: &'static str,
        icon: &'static str,
        on_click: impl Fn() -> Msg + Clone + 'static,
    ) -> Self {
        Button {
            text,
            style,
            icon,
            on_click: Rc::new(on_click),
        }
    }

    pub fn view(&self, disabled: bool, hidden: bool) -> Node<Msg> {
        let func = self.on_click.clone();
        button![
            C!["button", self.style],
            ev(Ev::Click, move |_| func()),
            attrs! {At::Disabled => disabled.as_at_value()},
            IF!(hidden => style! {St::Display => "none"}),
            span![C!["icon", "is-small"], i![C!["fas", self.icon]]],
            IF!(!self.text.is_empty() => span![self.text])
        ]
    }

    pub fn show(&self) -> Node<Msg> {
        self.view(false, false)
    }

    pub fn disabled(&self) -> Node<Msg> {
        self.view(true, false)
    }

    pub fn hidden(&self) -> Node<Msg> {
        self.view(true, true)
    }
}
