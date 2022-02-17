use yew::{prelude::*, function_component, html, Html};
use super::compe::CompE;
#[function_component(CompD)]
pub fn compd() -> Html {
    html! {
        <>
            <div>
                <CompE/>
            </div>
        </>
    }
}