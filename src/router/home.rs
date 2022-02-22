use yew::prelude::*;
use crate::use_hooks::use_async::UseAsync;
use crate::use_memo::counter::Counter;
pub struct Home;

impl Component for Home { 
    type Message = ();
    type Properties = ();
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

        html! {
            <>
                <div>
                    // <UseAsync/>
                    <Counter/>
                </div>
            </>
        }
    }
}