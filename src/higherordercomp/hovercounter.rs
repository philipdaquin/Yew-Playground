use yew::prelude::*;
use crate::higherordercomp::with_counter::NewComponent;



pub struct HoverCount { 
    pub count: i32,
    pub props: Props
}
#[derive(Properties, PartialEq, Clone)]
pub struct Props { 
    #[prop_or(Callback::noop())]
    pub onmouseover_signal: Callback<MouseEvent>,
}

pub enum Msg {
    Increment(MouseEvent)
}

impl Component for HoverCount {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
       
        HoverCount { 
            count: 0,
            props: Props { 
                onmouseover_signal: Callback::default()
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::Increment(mouseevent) => { 
                //self.props.onmouseover_signal.emit(mouseevent);

                self.count = self.count + 1;
                true 
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmouse_over = ctx.link().callback(|input: MouseEvent| Msg::Increment(input.into()));
        let counter = format!("Hovered {} times", self.count.clone()); 
        html! {
            <>
                <div>
                    <h2 onmouseover={onmouse_over}>{counter}</h2>
                </div>
            </>
        }
    }
}