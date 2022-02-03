use yew::prelude::*;

pub struct About;

impl Component for About { 
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
        html! {
            <>
                <img src="https://cms.qz.com/wp-content/uploads/2015/12/ap_651188993449.jpg?quality=75&strip=all&w=1200&h=900&crop=1" />
            </>
        }

    }

}