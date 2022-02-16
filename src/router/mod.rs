pub mod newapp;
pub mod home;
use crate::router::{home::Home, newapp::NewApp};
use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute { 
    #[at("/")]
    Home,

    #[at("/about")]
    About,

    
}

pub fn switch(routes: &AppRoute) -> Html { 
    match routes { 
        AppRoute::Home => html! { <Home/>},
        AppRoute::About => html! { <NewApp/>},
    }
}