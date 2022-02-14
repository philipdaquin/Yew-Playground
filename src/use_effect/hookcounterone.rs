use web_sys::HtmlInputElement;
use yew::{prelude::*, function_component, html, Html};
use gloo_utils::{self, document};
use gloo_console as console;
use std::rc::Rc;


#[function_component(HookCounterOne)]
pub fn hook_counter() -> Html {
    let count = use_state(|| 0);
    let name = use_state(String::default);
    
    {
        let count = count.clone();
        let count_ = count.clone();
        //  It takes a function which is called after every component's render finishes
        use_effect_with_deps(move |_| { 
            console::log!("use_effect - updating document title");
            /// Make a call to DOM API after component is rendered 
            document().set_title(&format!("You clicked {} times", *count));
                /// Cleanip function, which is called right before starting a new render 
                // || gloo::utils::document().set_title("You clicked 0 times")
                || () 
        },
            count_

        );
    }
    let set_count = { 
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };

    let set_name = { 
        let name = name.clone();
        Callback::from(move |e: InputEvent| { 
            let mut input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*name).clone();
            info = input.value();
            name.set(info)
        })
    };
    

    html! {
        <>
            <div>
                <input type="text" value={(*name).clone()} oninput={set_name} />
                <button onclick={set_count}>{*count}</button>
            </div>
        </>
    }
}
