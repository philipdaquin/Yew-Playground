use yew::{prelude::*, function_component, html, Html};
use crate::use_reducer::{
    counterone::CounterOne, 
    countertwo::CounterTwo,
    counterthree::CounterThree
};


#[function_component(AppReducer)]
pub fn reducers() -> Html {
    html! {
        <>
            <div>
                // <CounterOne/>
                // <CounterTwo/>
                <CounterThree/>
            </div>
        </>
    }
}