use yew::prelude::*;
use std::fmt::Debug;


pub struct Message { 
    state: State
}

#[derive(Debug, Clone)]
struct State  { 
    message: String,
}
pub enum Msg { 
    Clicked
}
impl State { 
    pub fn new(message: String) -> Self { 
        Self { 
            message,
        }
    }
}
impl Component for Message { 
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let state = State {
            message: "Welcome Visitor".into()
        };
        Self { 
           state
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::Clicked => { 
               self.state.message = "Thanks for Subscribing".into();
                true
            }
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Clicked);
        html! {
            <>
                <div>
                    <h1>{self.state.message.clone()}</h1>
                    <button onclick={onclick}>{"Subscribe"}</button>
                </div>
            </>
        }
    }
}