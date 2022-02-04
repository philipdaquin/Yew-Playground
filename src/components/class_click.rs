use yew::prelude::*;
use gloo_console as console;
pub struct ClassClick;

impl Component for ClassClick { 
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
        let click_handler = Callback::from(|_| { 
            console::log!("Button Clicked")
        });
        html! {
            <>
                <div>
                    <button onclick={click_handler}>{"Click Me"}</button>
                </div>
            </>
        }

    }

}