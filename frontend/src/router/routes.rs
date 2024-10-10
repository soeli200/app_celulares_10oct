use yew::prelude::*;
use yew_router::prelude::*;
use crate::paginas::folder_registro::pagina_registro_usuario::PaginaRegistroUsuario;
use crate::paginas::home::home::Home;
use crate::paginas::not_found::not_found::NotFound;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Registro,
    #[at("/registro")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Registro => html! { <PaginaRegistroUsuario /> },
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
    }
}