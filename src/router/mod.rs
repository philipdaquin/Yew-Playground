pub mod newapp;
pub mod home;
pub mod app_reducer;
pub mod cartprovider;
use crate::router::{home::Home, /* newapp::NewApp, app_reducer::AppReducer, cartprovider::CartProvider*/};
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

    #[at("/Addtocart")]
    AddToCart
}
pub fn switch(routes: &AppRoute) -> Html { 
    match routes { 
        AppRoute::Home => html! { <Home/>},
        AppRoute::About => todo!(),
        AppRoute::AppReducer => todo!(),
        AppRoute::AddToCart => todo!(),
        // AppRoute::About => html! { <NewApp/>},
        // AppRoute::AppReducer => html! { <AppReducer/>},
        // AppRoute::AddToCart => html! { <CartProvider/>},

    }
}