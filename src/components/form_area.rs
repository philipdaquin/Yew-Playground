use yew::{prelude::*, NodeRef};

pub struct FormArea { 
    pub value: String, 
    pub input_ref: NodeRef
}
#[derive(Properties, PartialEq, Clone)]
pub struct TextAReaProps { 
    #[prop_or_default]
    pub name: String,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub onchange: Callback<String>,

    #[prop_or_default]
    pub oninput: Callback<String>
}


// Signal the input element has changed
pub enum TextMsg { 
    Changed(String),
    Input(String)
}

impl Component for FormArea { 
    type Message = TextMsg;
    type Properties = TextAReaProps;

    fn create(ctx: &Context<Self>) -> Self {
        let value = ctx.props().value.clone();
        Self { 
            value,
            input_ref: NodeRef::default()
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            TextMsg::Changed(input) => { 
                self.value = input.clone();
                ctx.props().onchange.emit(input)
            },
            TextMsg::Input(input) => { 

            }
        }
        true 
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div></div>
            </>
        }

    }

}