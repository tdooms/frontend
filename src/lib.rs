use fields::{Field, InputField, SelectField, SliderField, ToggleField};
use seed::{prelude::*, *};

struct Model {
    name: InputField<String>,
    age: InputField<f64>,
    gender: SelectField,
    income: SliderField,
    toggle: ToggleField,
}

enum Msg {
    Name(<InputField<String> as Field>::Msg),
    Age(<InputField<f64> as Field>::Msg),
    Gender(<SelectField as Field>::Msg),
    Income(<SliderField as Field>::Msg),
    Toggle(<ToggleField as Field>::Msg),
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        name: InputField::string("name"),
        age: InputField::f64("age").optional(),
        gender: SelectField::new(
            "gender",
            vec![(1, "male".to_owned()), (2, "femali".to_owned())],
        )
        .default(0, "enter gender", false),
        income: SliderField::linear("income", 0.5),
        toggle: ToggleField::new("toggle", false),
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let _ = match msg {
        Msg::Name(msg) => model.name.update(msg, &mut orders.proxy(Msg::Name)),
        Msg::Age(msg) => model.age.update(msg, &mut orders.proxy(Msg::Age)),
        Msg::Gender(msg) => model.gender.update(msg, &mut orders.proxy(Msg::Gender)),
        Msg::Income(msg) => model.income.update(msg, &mut orders.proxy(Msg::Income)),
        Msg::Toggle(msg) => model.toggle.update(msg, &mut orders.proxy(Msg::Toggle)),
    };
    log!(
        model.name.value(),
        model.age.value(),
        model.gender.value(),
        model.income.value(),
        model.toggle.value()
    )
}

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    section![
        C!["section"],
        div![
            C!["container"],
            model.name.view(false).map_msg(Msg::Name),
            model.age.view(false).map_msg(Msg::Age),
            model.gender.view(false).map_msg(Msg::Gender),
            model.income.view(false).map_msg(Msg::Income),
            model.toggle.view(false).map_msg(Msg::Toggle),
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
