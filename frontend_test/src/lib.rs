use seed::{prelude::*, *};

use frontend::Form;

#[derive(Form)]
struct Ingredient {
    name: String,
    category: String,
}

struct Model {
    form: IngredientForm,
}

enum M {
    Form(<IngredientForm as Form>::Msg),
}

fn init(_: Url, orders: &mut impl Orders<M>) -> Model {
    Model {
        form: Ingredient::form(),
    }
}

fn update(msg: M, model: &mut Model, orders: &mut impl Orders<M>) {
    match msg {
        M::Form(msg) => {
            model.form.update(msg, &mut orders.proxy(M::Form));
        }
    };
}

// `view` describes what to display.
fn view(model: &Model) -> Node<M> {
    section![
        C!["section"],
        div![C!["container"], model.form.view().map_msg(M::Form),]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
