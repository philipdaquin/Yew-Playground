use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props { 
    pub styles: bool
}

#[function_component(StyleSheets)]
pub fn stylesheet(props: &Props) -> Html {
    let class_name = if props.styles {"primary font-xl"} else {"font-xl"};
    html! { 
        <>
            <div>
                <h1 class={class_name}>{"Hello"}</h1>
            </div>
        </>
    }
}