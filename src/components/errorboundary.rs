use yew::prelude::*;

//  This is the state
pub struct ErrorBoundary { 
    pub props: ErrorProps
}
#[derive(Properties, PartialEq, Clone)]
pub struct ErrorProps  { 
   #[prop_or(false)]
    pub has_error: bool,
    pub children: Children
  
}
pub enum Msg {
}

impl Component for ErrorBoundary {
    type Message = Msg;
    type Properties = ErrorProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            props: ErrorProps { 
                has_error: false,
                children: Children::default()
            } 
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool  {
        false
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        
    }
    fn view(&self, ctx: &Context<Self>) -> Html {

        // let get_derived_from_state_error = ErrorProps { has_error: true, children: Children::new(children)};


        if ctx.props().has_error { 
            return html! { 
                <h1>{"Something Went Wrong"}</h1>
            }
        } else { 
            html! { 
               <div class="list">
                { for ctx.props().children.iter()}
               </div>
            }
        } 
    }
}