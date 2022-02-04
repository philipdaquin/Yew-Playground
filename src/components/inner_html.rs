use yew::{prelude::*, Html as File};
use gloo_utils::document;
use gloo_console;
const HTML_FILE: &str = include_str!("inner.html");
use web_sys::console;
pub struct Html { 
    val: i64
}

impl Component for Html { 
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            val: 0
        }
    }
    fn view(&self, ctx: &Context<Self>) -> yew::virtual_dom::VNode {
        let div = document()
            .create_element("div")
            .unwrap();
        div.set_inner_html(HTML_FILE);
        gloo_console::log!(&div);

        File::VRef(div.into())
    }

}