use yew::{prelude::*, function_component, html, Html};
use std::fmt;
#[derive(Properties, PartialEq, Clone)]
pub struct Props { 
    pub name: String
}

#[derive(Debug)]
pub enum ErrorResponse { 
    SomethingGoneWrong
}
impl fmt::Display for ErrorResponse { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        match self { 
            ErrorResponse::SomethingGoneWrong => write!(f, "This is not write lmaoooooooooooo!")
        }
    }
}

#[function_component(Hero)]
pub fn hero(props: &Props) -> Html {
    match props.name == "Joker"  { 
        true => { 
            return  html! { <h1> {ErrorResponse::SomethingGoneWrong} </h1>}
        }
        _ =>  return html! {
            <>
                {props.name.clone()}
            </>
        }
    }
}