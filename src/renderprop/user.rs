use yew::prelude::*;

pub struct User;

pub enum Msg {
}


fn name_prop() -> String { 
    "Hello World".to_string()
}


#[derive(Properties, PartialEq, Clone)]
pub struct Props { 
    #[prop_or_else(name_prop)]
    pub name: String
}

impl Component for User {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>{ctx.props().name.clone()}</div>
        }
    }
}