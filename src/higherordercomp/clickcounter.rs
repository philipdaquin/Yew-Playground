use yew::prelude::*;
use crate::higherordercomp::with_counter::NewComponent;





pub struct ClickCounter { 
    pub count: i32
}

pub enum Msg {
    Increment
}

impl Component for ClickCounter {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            count: 0
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::Increment => { 
                self.count = self.count + 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        html! {
            <>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}>
                    { format!("Click {} times", self.count.clone()) } 
                </button>
            </>
        }
    }
}