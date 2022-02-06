use web_sys::{Text, HtmlInputElement};
use yew::{prelude::*, NodeRef,};




pub struct FormInput { 
    value: Option<String>,
    //  NodeRef can be used instead of querying the event 
    //  given to a Callback.

    /// The REf keyword can be used inside of any HTML element or component to get the DOM 
    /// element that the item is attached to
    /// THis can be used to make changes to ther DOM outside of the view lifecycle method 
    input_ref: NodeRef
}
   
#[derive(Properties, PartialEq, Clone)]
pub struct TextInputProps { 
    #[prop_or_default]
    pub name: String,

    #[prop_or_default]
    pub value: String,

    
    #[prop_or_default]
    pub topics: String,

    #[prop_or_default]
    pub onchange: Callback<String>,

    #[prop_or_default]
    pub oninput: Callback<String>
}


// Signal the input element has changed
pub enum TextMsg { 
    Changed(String),
    Input(String),
    Submit 
}
/// TEXTINPUT COmponent
impl Component for FormInput { 
    type Message = TextMsg;
    type Properties = TextInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        let value = ctx.props().value.clone();
        Self { 
            value: None,
            input_ref: NodeRef::default()
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            TextMsg::Changed(input) => { 
                self.value = Some(input.clone());
                ctx.props().onchange.emit(input);
            },
            TextMsg::Input(input) => { 
                ctx.props().oninput.emit(input);
                if let Some(val) = self.extract_input() { 
                    self.value = Some(val.clone());
                    ctx.props().onchange.emit(val)
                }
            }
            TextMsg::Submit => { 
                if let Some(val) = self.extract_input() { 
                    self.value = Some(val.clone())
                }
            }
        }
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
       let value = self.value.clone();
       let input_ref = self.input_ref.clone();
       let onchange = ctx.link().batch_callback(move |_| { 
           input_ref.cast::<HtmlInputElement>().map(|input| TextMsg::Changed(input.value()))
       });
       let oninput = ctx.link().callback(|input: InputEvent| TextMsg::Input(input.data().unwrap_or_default()));
       let submit = ctx.link().callback(|_| TextMsg::Submit);


        html! {
            <>
                <form onsubmit={submit}>
                <div>
                    <label>{"Form Component"}</label>
                    <input 
                        type="text" 
                        value={value.clone()} 
                        onchange={onchange.clone()}    
                        oninput={oninput}
                    />
                   

                    </div>
                    <div>
                        <label>
                            <select value={ctx.props().topics.clone()} onchange={onchange}>
                                <option value={"React"}>{"React"}</option>
                                <option value={"Rust"}>{"Rust"}</option>
                                <option value={"Substrate"}>{"Substrate"}</option>
                            </select>
                        </label>
                    </div>
                    <button type="submit" >{"Submit"}</button>
                </form>
            </>
        }
    }
}
/// using NodeRef, you can ignore the event and use the NodeRef::Cast method to 
/// an Option<HtmlInputElement> 
impl FormInput  {
    fn extract_input(&self) -> Option<String> { 
        self.input_ref
        .cast::<HtmlInputElement>()
        .map(|input| input.value())
    }
    fn value(&self, ctx: &Context<Self>) -> String { 
        self
        .value
        .clone()
        .unwrap_or_else(|| ctx.props().value.clone())
    }
}



