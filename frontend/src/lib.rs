use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::routes::{Route, switch};

mod router;
mod paginas;
mod models;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::Renderer::<App>::new().render();
    Ok(())
}
