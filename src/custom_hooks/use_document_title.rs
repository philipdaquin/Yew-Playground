
use yew::{prelude::*, function_component, html, Html};
use gloo_utils::{self, document};

pub fn use_document(count: i32) -> UseStateHandle<i32>  {
    let counter = use_state(|| count );
    // let set_count = {
    //     let counter = counter.clone();
    //     Callback::from(move |e: MouseEvent| counter.set(*counter + 1))
    // };
   
    { 
        let counter = counter.clone();
        let count_ = counter.clone();
        use_effect_with_deps(move |_| {
            document().set_title(&format!("{}", *counter));
            || ()
        },  count_);
    
    }

    counter
}