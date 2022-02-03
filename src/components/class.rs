use yew::prelude::*;
use crate::components::hello::Props;

#[derive(PartialEq, Properties, Clone)]
pub struct Class;

impl Component for Class { 
    type Message = ();
    type Properties = Props;

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
        let Props {name, num, children} = ctx.props();
        //  let State {}
        html! {
            <>
                <div>
                <h4>                
                    {"Hello"}
                    // no need to use self.ctx => instead ctx.props...
                    {name}
                    {num}
                    {for children.iter()}
                </h4>
                </div>
            </>
        }

    }

}