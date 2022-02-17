use yew::{prelude::*, function_component, html, Html};
use super::compe::CompE;
#[function_component(CompC)]
pub fn compc() -> Html {
    html! {
        <>
            <div>
                <CompE/>
            </div>
        </>
    }
}