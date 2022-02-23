use yew::{prelude::*, function_component, html, Html};
use yew_hooks::use_interval;
#[function_component(HookTimer)]
pub fn hooktimer() -> Html {
    let interval = use_state(|| 0);

    {
        let interval = interval.clone();
        use_interval(move || { 
            interval.set(*interval + 1);
        }, 200)
    }
    let reset = { 
        let interval = interval.clone();
        Callback::from(move |e: MouseEvent| interval.set(0))
    };
    
    html! {
        <>
            <div>
            <button onclick={reset}>{"Reset Interval"}</button>
               <h1>
                    {*interval}
               </h1> 
            </div>
        </>
    }
}