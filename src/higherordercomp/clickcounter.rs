use yew::prelude::*;
use crate::higherordercomp::with_counter::{OriginalComponent, Props, Msg};

pub struct ClickCounter { 
    pub props: Props
}

impl Component for ClickCounter {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            props: ctx.props().clone()
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let Props { count, increment_count }  = self.props.clone();

        html! {
            <>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}>
                    { format!("Click {} times", count) } 
                </button>
            </>
        }
    }
}