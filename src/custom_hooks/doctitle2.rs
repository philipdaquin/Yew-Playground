use gloo_utils::{self, document};
use yew::{prelude::*, function_component, html, Html};
use super::use_document_title::use_document;


#[function_component(DocumentTwo)]
pub fn use_documenttwo() -> Html {


    let counter = use_document(12);
    let set_count = {
        let counter = counter.clone();
        Callback::from(move |e: MouseEvent| counter.set(*counter + 1))
    };

    html! {
        <> 
            <div>
                <button onclick={set_count}>{"Count - "}{*counter}</button>
            </div>
        </>
    }
}






