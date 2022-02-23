use yew::prelude::*;

use crate::use_form::use_forms::UseForm;

pub struct Home;
impl Component for Home { 
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
                <UseForm />
            </>
        }
    }
}