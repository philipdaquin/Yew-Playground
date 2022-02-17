use yew::{prelude::*, function_component, html, Html};
use super::compe::CompE;
use crate::router::app_reducer::{SubtotalContext, Action};



#[function_component(CompD)]
pub fn compa() -> Html {
    let subtotal = use_context::<SubtotalContext>().expect("Try again Lol");
    let add_to_cart = {
        let subtotal = subtotal.clone();
        Callback::from(move |_| subtotal.dispatch(Action::AddToCart(1)))};
    let increment = {
        let subtotal = subtotal.clone();        
        Callback::from(move |_| subtotal.dispatch(Action::Increment))
    };
    let decrement = { 
        let subtotal = subtotal.clone();
        Callback::from(move |_| subtotal.dispatch(Action::Decrement))
    };
    html! {
        <>
            <div>
                {"Component D"}
                <button onclick={add_to_cart}>{"Add to Cart"}</button>
                <button onclick={decrement}>{"Decrement"}</button>
                <button onclick={increment}>{"Increment"}</button>
            </div>
        </>
    }
}