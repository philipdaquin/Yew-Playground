use yew::prelude::*;

#[function_component(Inline)]
pub fn inline() -> Html { 
    // In line styling in rust 
    let heading = {"
        font-size: 72px; 
        font-weight: 2rem;
        color: blue;"
    };
    html! { 
        <>
            <div>
                <h1 style={heading}>{"Inline"}</h1> 

            </div>
        </>
    }
}