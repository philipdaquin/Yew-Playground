use yew::{prelude::*, function_component, html, Html};
use super::compd::CompD;

#[function_component(CompB)]
pub fn compb() -> Html {
    html! {
        <>
            <div>
                <CompD/>
            </div>
        </>
    }
}