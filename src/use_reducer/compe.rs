use yew::{prelude::*, function_component, html, Html};
use super::compf::CompF;
#[function_component(CompE)]
pub fn compe() -> Html {
    html! {
        <>
            <div>
                <CompF/>
            </div>
        </>
    }
}