use yew::prelude::*;
use reqwasm::{http::*, Error};
use yew_interop::script::wasm_bindgen_futures::spawn_local;
use gloo_utils;


pub struct PostList { 
    posts: Vec<String>
}
pub enum Msg { 
    Get
}
impl Msg { 
    async fn get(url: &str) { 
        Request::get(url).send().await.unwrap();

    }
}



impl Component for PostList {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            posts: Vec::new()
        }
    }
    /// Communication with components happens primarily through messages which are handled by the 
    /// update lifecycle method. This allows the component to updat eitself based on what the message was and deterine if ti needs to 
    /// re render itself
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::Get => { 
                let posts = self.posts.clone();
                
                
            }
        }
        true         
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>{"Hello"}</div>
            </>
        }
        
    }
}

