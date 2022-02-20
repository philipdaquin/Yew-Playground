use yew::{prelude::*, function_component, html, Html};

// use crate::usecontext::compc::CompC;

#[derive(Debug, Clone, PartialEq)]
pub struct UserContext { 
    pub value: String
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelContext { 
    pub company: String
}


#[function_component(NewApp)]
pub fn newapp() -> Html {
    let ctx = use_state(|| UserContext { 
        value: "Philip".to_owned()
    });
    let ctx_channel = use_state(|| ChannelContext { 
        company: "".to_owned()
    });

    html! {
        <>
            <div>
                // <ContextProvider<UserContext> context={(*ctx).clone()}>
                //     <ContextProvider<ChannelContext> context={(*ctx_channel).clone()}>
                //         // <CompC/>
                //     </ ContextProvider<ChannelContext>>
                // </ContextProvider<UserContext>>
            </div>
        </>
    }
}