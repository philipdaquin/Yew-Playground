use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo_utils;
pub struct Ref { 
    my_input: NodeRef
}

pub enum Msg { 
    InputValue(String)
}

impl Component for Ref { 
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            my_input: NodeRef::default()
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        match msg { 
            Msg::InputValue(value) => { 
                let window = gloo_utils::window();
                window.alert_with_message(&value).unwrap();
        }
        }
  
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let my_input_ref = self.my_input.clone();

        let oninput = ctx.link().batch_callback(move |_| { 
            let input = my_input_ref.cast::<HtmlInputElement>();
            input.map(|input| Msg::InputValue(input.value()))

        });

        html! {
            <>
                <div>
                    <input type="text" ref={self.my_input.clone()} oninput={oninput}/>
                    <button oninput={onclick}>{"Click"}</button>
                </div>
            </>
        }

    }

}