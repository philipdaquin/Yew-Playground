use yew::prelude::*;




//  Pure components have no states nor effects
#[function_component(PureComp)]
pub fn purecomp() -> Html { 
    return html! { 
        <>
            <div>{"Pure Component"}</div>
        </>
    }
}