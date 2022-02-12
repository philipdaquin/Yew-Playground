use yew::{prelude::*, function_component, html, Html};

#[function_component(HookCounter)]
pub fn hookcounter() -> Html {
    let count = use_state(||0 );

    let onclick = {
        let count = count.clone();
        Callback::from(move |_| { 
            count.set(*count + 1)
        })
    };
        

    html! {
        <>
            <div>
                <p>{"Number of Counts"}</p><button {onclick}>{*count}</button>
            </div>
        </>
    }
}