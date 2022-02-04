use yew::prelude::*;
use gloo_console as console;

#[function_component(FunctionClick)]
pub fn function_click() -> Html { 
    let click_handler = Callback::from(|_| {
        console::log!("Button Clicked")
    });
    html! { 
        <div>
            <button onclick={click_handler}>{"Click Me"}</button>

        </div>
    }
}