use yew::{prelude::*, NodeRef, Html};
use web_sys::{Text, HtmlInputElement};

pub struct FormArea { 
    pub value: String, 
    pub input_ref: NodeRef
}
#[derive(Properties, PartialEq, Clone)]
pub struct TextAreaProps { 
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
    type Properties = TextAreaProps;

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
                ctx.props().onchange.emit(input);
                false
            },
            TextMsg::Input(input) => { 
                ctx.props().oninput.emit(input);
                if let Some(val) = self.extract_value() { 
                    self.value = val.clone();
                    ctx.props().onchange.emit(val);
                }
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let input_ref = self.input_ref.clone();
        
        let oninput = ctx
            .link()
            .callback(|input: InputEvent| TextMsg::Input(input.data().unwrap_or_default()));
       let onchange = ctx.link().batch_callback(move |_| { 
           input_ref.cast::<HtmlInputElement>().map(|input| TextMsg::Changed(input.value()))
       });
        
        html! {
            <>
                <textarea
                ref={self.input_ref.clone()}
                name={ctx.props().name.clone()}
                value={ctx.props().value.clone()}
                onchange={onchange}
                oninput={oninput}
                ></textarea>
            </>
        }
    }
}
impl FormArea { 
    fn extract_value(&self) -> Option<String> { 
        self.input_ref
        .cast::<HtmlInputElement>()
        .map(|input| input.value()) 
    }
    
}