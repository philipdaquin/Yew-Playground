use yew::{prelude::*, function_component, html, Html};
use crate::components::mousemovement::MouseMovement;
#[function_component(MouseContainer)]
pub fn mousecontainer() -> Html {
    let display = use_state(|| true);
    let set_display = { 
        let display = display.clone();
        Callback::from(move |_| { 
            display.set(false)   
        })
    };
    html! {
        <>
            <div>
                <button onclick={set_display}>{"Toggle Display"}</button>
            </div>
        </>
    }
}