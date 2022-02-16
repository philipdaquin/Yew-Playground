use yew::{prelude::*, function_component, html, Html};
use crate::router::newapp::UserContext;

#[function_component(CompF)]
pub fn compf() -> Html {
    let context = use_context::<UserContext>()
        .expect("NO CTX FOUND");
    html! {
        <>
            <div>{context.value}</div>

        </>
    }
}