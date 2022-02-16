use yew::{prelude::*, function_component, html, Html};
use crate::use_reducer::counterone::CounterOne;


#[function_component(AppReducer)]
pub fn reducers() -> Html {
    html! {
        <>
            <div>
                <CounterOne/>
            </div>
        </>
    }
}