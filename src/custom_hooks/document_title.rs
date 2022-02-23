use gloo_utils::{self, document};
use super::use_document_title::use_document;
use yew::{prelude::*, function_component, html, Html};

#[function_component(DocumentTitle)]
pub fn use_documentsssss() -> Html {
    // let count = use_state(|| 0 );
   
    // { 
    //     let count = count.clone();
    //     let count_ = count.clone();
    //     use_effect_with_deps(move |_| {
            
    //     document().set_title(&format!("{}", *count));
    //     || ()
    // },  count_);}
    let count = use_document(2);
    let set_count = {
        let count = count.clone();
        Callback::from(move |e: MouseEvent| count.set(*count + 1))
    };
   

    html! {
        <> 
            <div>
                <button onclick={set_count}>{"Count - "}{(*count).clone()}</button>
            </div>
        </>
    }
}