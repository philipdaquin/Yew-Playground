pub mod newapp;
pub mod home;
pub mod app_reducer;
use crate::router::{home::Home, newapp::NewApp, app_reducer::AppReducer};
use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute { 
    #[at("/")]
    Home,

    #[at("/about")]
    About,
    
    #[at("/app_reducer")]
    AppReducer,
}
pub fn switch(routes: &AppRoute) -> Html { 
    match routes { 
        AppRoute::Home => html! { <Home/>},
        AppRoute::About => html! { <NewApp/>},
        AppRoute::AppReducer => html! { <AppReducer/>},
    }
}