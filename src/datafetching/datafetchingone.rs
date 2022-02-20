use reqwasm::http::Request;
use serde::Deserialize;
use yew::{prelude::*, html, Callback, Properties},

#[derive(Default, PartialEq, Clone, Deserialize)]
pub struct IntialState { 
    user_id: i32,
    id: i32, 
    title: String,
    body: String
}

#[function_component(DataFetchingOne)]
pub fn datafetchingone() -> Html {
    
    let error = use_state(|| String::new());
    let set_error = {};

    let state: UseStateHandle<Vec<IntialState>> = use_state(|| vec![]);
    let set_post = {};
    
    {
        let state = state.clone();
        use_effect_with_deps(move |_| { 
            let state = state.clone();
            set_loading(false);
            wasm_bindgen_futures::spawn_local(async move { 
                let request: Vec<IntialState> = Request::get("https://jsonplaceholder.typicode.com/posts/1")
                    .send()
                    .await
                    .unwrap()
                    .json() 
                    .await 
                    .unwrap();
                state.set(request);
                
            });  
            || ()  
        }, ());
    }


    html! {
        <>

        </>
    }
}