use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::routes::{Route, switch}; // Importa la función switch y el enum Route

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

//fn main() {
//    yew::start_app::<App>();
//}