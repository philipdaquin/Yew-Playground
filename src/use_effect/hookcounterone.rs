use yew::{prelude::*, function_component, html, Html};

#[function_component(HookCounterOne)]
pub fn hook_counter() -> Html {
    
    let count = use_state(|| 0);
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