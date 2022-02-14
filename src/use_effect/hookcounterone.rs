use yew::{prelude::*, function_component, html, Html};
use gloo_utils::{self, document};
use std::rc::Rc;


#[function_component(HookCounterOne)]
pub fn hook_counter() -> Html {
    let count = use_state(|| 0);
    
    
    {
        let count = count.clone();

        //  It takes a function which is called after every component's render finishes
        use_effect(move || { 
            /// Make a call to DOM API after component is rendered 
            document().set_title(&format!("You clicked {} times", *count));
                /// Cleanip function, which is called right before starting a new render 
                || gloo::utils::document().set_title("You clicked 0 times")
        });

    }

    let set_count = { 
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };

    
    html! {
        <>
            <div>
                <button onclick={set_count}>{*count}</button>
            </div>
        </>
    }
}