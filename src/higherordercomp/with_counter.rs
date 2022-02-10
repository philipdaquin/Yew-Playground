use yew::{prelude::*};


use yew::prelude::*;

pub struct OriginalComponent { 
    props: Props
}
#[derive(Properties, PartialEq, Clone)]
pub struct Props { 
    name: String
}
pub enum Msg {
}

impl Component for OriginalComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            props: Props  {
                name: String::new()
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>{format!("{}", ctx.props().name.clone())}</div>
            </>
        }
    }
}


pub struct Props { 
    component: OriginalComponent
}

#[function_component(NewComponent)]
pub fn newcomponent(component: &Props) -> Html { 
    html! { 
        <>
            
        </>
    }
}