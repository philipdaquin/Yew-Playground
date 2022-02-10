use yew::{prelude::*, function_component, html, Html};

#[derive(Properties, PartialEq, Clone)]
pub struct Props { 
    pub name: String
}

#[function_component(Hero)]
pub fn hero(props: &Props) -> Html {
    if props.name == "Joker"  { 
        panic!("Crash and Burn!")
    }
    html! {
        <>
            {props.name.clone()}
        </>
    }
}