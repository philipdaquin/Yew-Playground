use yew::prelude::*;
use crate::custom_hooks::{document_title::DocumentTitle, doctitle2::DocumentTwo};
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
                <div>
                    // <UseAsync/>
                    // <Counter/>
                    // <FocusInput />
                    // <ClassTimer/>
                    // <HookTimer/>
                    <DocumentTitle/>
                    <DocumentTwo/>

                </div>
            </>
        }
    }
}