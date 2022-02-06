use yew::prelude::*;

pub struct LifeCycleA { 
    name: &'static str,
    props: Props
}
#[derive(Properties, PartialEq, Clone)]
pub struct Props { 

}

impl Component for LifeCycleA { 
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            name: "Philip Daquin",
            props: Props {},
        }
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
                <div>{self.name}</div>
            </>
        }

    }

}