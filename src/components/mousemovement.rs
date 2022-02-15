use yew::prelude::*;
use gloo::events::EventListener;
use gloo_utils;
use gloo_console;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
pub struct MouseMovement { 
    pub x: i32,
    pub y: i32,
    pub mousemove: Option<EventListener>
}

pub enum Msg {
    MouseMove(MouseEvent)
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
}
impl Component for MouseMovement {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            x: 0, 
            y: 0,
            mousemove: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::MouseMove(e) => {
                self.x = e.client_x();
                self.y = e.client_y();
                true
            }
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render { 
            return;
        }

        let window = gloo_utils::window();
        let mousemove = ctx.link().callback(|e| Msg::MouseMove(e));

        let event_listener = EventListener::new(
            &window, 
            "mousemove", 
            move |e| { 
                let event = e.dyn_ref::<MouseEvent>().unwrap_throw();
                mousemove.emit(event.clone())
            }
        );
        self.mousemove = Some(event_listener);
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>
                    {format!("X -{} Y -{}", self.x, self.y)}
                </div>
            </>
        }
    }
}