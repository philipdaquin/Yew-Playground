use yew::prelude::*;

pub struct ErrorBoundary;

pub enum Msg {
}

#[derive(PartialEq, Properties, Clone)]
pub struct ErrorProps {
}
impl Component for ErrorBoundary {
    type Message = Msg;
    type Properties = ErrorProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool  {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="">
                
                </div>
            </>
        }
    }
}