use yew::prelude::*;
use yew_router::prelude::*;

pub mod home;
use home::Home;

pub mod about;
use about::About;

pub mod name;
use name::Name;


/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/name")]
    Name,



}

/// Switch app routes
pub fn switch(routes: AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::Name => html! { <Name name="name" /> },
        AppRoute::PageNotFound => html! { "Page not found" },

    }
}
