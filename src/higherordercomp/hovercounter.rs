use yew::prelude::*;
use crate::higherordercomp::with_counter::{OriginalComponent, Msg, Props};



pub struct HoverCount;

impl Component for HoverCount {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self 
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { count, increment_count }  = ctx.props().clone();

        let onmouse_over = ctx.link().callback(|_| Msg::Increment);
        let counter = format!("Hovered {} times", count); 

        html! {
            <>
                <div>
                    <h2 onmouseover={onmouse_over}>{counter}</h2>
                </div>
            </>
        }
    }
}