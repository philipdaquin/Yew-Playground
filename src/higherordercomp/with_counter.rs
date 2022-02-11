use yew::prelude::*;
use crate::app::App;


use yew::prelude::*;

use yew::prelude::*;

pub struct OriginalComponent { 
    pub props: Props
}
#[derive(Properties, Clone, PartialEq)]
pub struct Props { 
    #[prop_or_default]
    pub count: i32,
    #[prop_or_default]
    pub increment_count: i32
}
pub enum Msg {
    Increment
}
impl Component for OriginalComponent{
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: Props { 
                count: 0,
                increment_count: 0
            }
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::Increment => { 
                self.props.count = self.props.increment_count.clone() + 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let count = ctx.props().count;
        let increment_count = ctx.props().increment_count.clone();


        html! {
            <>
                <OriginalComponent count={count}
                    increment_count={increment_count}
                />
            </>        
        }
    }
}


