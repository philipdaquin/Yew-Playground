use yew::prelude::*;

pub struct ParentComponent;

#[derive(Clone, PartialEq, Properties)]
pub struct Props { 
    pub prop1: String,
    pub prop2: String
}


impl Component for ParentComponent { 
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { prop1, prop2} = ctx.props();
        html! {
            <> 
                {
                    format!("prop1: {} prop2: {}", prop1, prop2 )
                }
            </>
        }
    }
}

