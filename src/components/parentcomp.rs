use gloo::{
    timers::callback::{Interval, Timeout}, 
    console::{self, Timer}};
use yew::prelude::*;

pub struct ParentComp { 
    name: Vec<String>,
    // standalone: (Interval, Interval),
    interval: Option<Interval>
}
// pub enum Msg { 
//     StartTime,
//     StartInterval,
//     UpdateMsg
// }

impl Component for ParentComp { 
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let standalone_handle = Interval::new(
            2000, || console::debug!("Callback!")
        );
        let message_handle = {
            let link = ctx.link().clone();
            //Interval::new(3, move || link.send_message(Msg::UpdateMsg))
        };
        Self { 
            name: Vec::new(),
            // standalone: (standalone_handle),
            interval: None
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // match msg { 
        //     Msg::StartTime => { 
        //         let link = ctx.link().clone();
        //         true
        //     }
        // }
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>{"Parent Component"}</div>
            </>
        }

    }

}