use gloo_console::Timer;
use yew::{prelude::*, function_component, html, Html};
use gloo::{console::{self},
    timers::callback::{Interval, Timeout}
};



#[function_component(IntervalHookCounter)]
pub fn intervalhook() -> Html {
    let count = use_state(|| 0);
    let set_count = { 
        let count = count.clone();
        Callback::from( move |e: i32| { 
            count.set(*count + 1)
        })
    };

    // {
    //     use_effect_with_deps(move |_| { 
    //         let interval = Interval::new(1, move || set_count);
    //     } ||  ()
    //     )
    // }


    html! {
        <>
            <div>
              //  {*count}    
            </div>
        </>
    }
}