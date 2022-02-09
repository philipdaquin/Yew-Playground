use yew::{prelude::*, function_component, html, Html, create_portal};
use gloo_utils::document;


#[function_component(PortalDemo)]
pub fn portaldemo() -> Html {
    let id_element = document().get_element_by_id("portal-root").expect("Head Element to be present");
    return create_portal(html! {
        
        <h1>{"Portals Demo"}</h1>
        
    }, id_element.into())
}