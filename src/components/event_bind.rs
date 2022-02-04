use yew::prelude::*;

pub struct EventBind { 
    state: State 
}
struct State { 
    message: String 
}
pub enum Msg { 
    ChangeMessage,
}

impl Component for EventBind { 
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            state: State { 
                message: "Hello".to_string()
            }
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::ChangeMessage => { 
                self.state.message = "Goodbye".into();
                true 
            }
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let State { message } = &self.state;
        //  Binding in the render method 
        let click_handler = ctx.link().callback(|_| Msg::ChangeMessage);
        let handler = |msg: Msg| -> Callback<MouseEvent> { 
            ctx.link().callback(|_| Msg::ChangeMessage)
        };
        html! {
            <>
                <div>
                    <p> {message.clone()}</p>
                    <button onclick={click_handler}>{"Event Binding Button"}</button>
                    <button onclick={Some(ctx.link().callback(|_| Msg::ChangeMessage))}>{"Event Binding Button"}</button>
                    <button onclick={handler(Msg::ChangeMessage)}>{"Event Binding Button"}</button>

                </div>
            </>
        }

    }

}