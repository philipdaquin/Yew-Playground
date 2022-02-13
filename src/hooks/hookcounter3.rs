use serde::de::MapAccess;
use yew::{prelude::*, function_component, html, Html};

#[derive(Properties, Clone, PartialEq)]
pub struct Person { 
    #[prop_or_default]
    first_name: String,
    #[prop_or_default]
    last_name: String
}
#[function_component(HookCounterThree)]
pub fn hookcounterthree(Person {..}: &Person) -> Html {

    


    html! {
        <>
        // <form action="">
        //     <input type="text" 
        //         value={name.first_name.clone()} 
        //         onchange={
        //             Callback::from(|e| name.set(name.first_name = e.target.value()))
        //         }/>
        //     <input type="text"/>
        //     <h2>{format!("Your first name {} -", name.first_name)}</h2>
        //     <h2>{format!("Your last name {} -", name.last_name)}</h2>
        // </form>
        </>
    }
}