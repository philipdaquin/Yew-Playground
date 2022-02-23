mod app;
mod api;
mod types;
mod service;
pub mod components;
mod hooks;
pub mod router;
pub mod contexts;
pub mod higherordercomp;
pub mod renderprop;
pub mod use_effect;
pub mod usecontext;
pub mod use_reducer;
pub mod fetching_data;
pub mod datafetching;
pub mod use_hooks;
pub mod use_memo;
pub mod use_ref;
pub mod custom_hooks;

use crate::app::App;

use wasm_bindgen::prelude::*;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}