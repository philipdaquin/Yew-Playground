use yew::{prelude::*, function_component, html, Html};

use crate::usecontext::compc::CompC;

#[derive(Debug, Clone, PartialEq)]
pub struct UserContext { 
    pub value: String
}

#[function_component(NewApp)]
pub fn newapp() -> Html {
    let ctx = use_state(|| UserContext { 
        value: "Philip".to_owned()
    });

    html! {
        <>
            <div>
                <ContextProvider<UserContext> context={(*ctx).clone()}>
                    <CompC/>
                </ContextProvider<UserContext>>
            </div>
        </>
    }
}