use yew::prelude::*;
pub struct Conditional { 
    state: State,
    props: Props
}
struct State { 
    is_logged_in: bool,
}
#[derive(PartialEq, Properties, Clone)]
pub struct Props { 

} 

pub enum Msg { 
    SignIn,
    SignOut
}

impl Component for Conditional { 
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            state: State { 
                is_logged_in: false
            },
            props: Props {}, 
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::SignIn => { 
                if !self.state.is_logged_in { 
                    self.state.is_logged_in = true;
                }
                true 
            },
            Msg::SignOut => { 
                if self.state.is_logged_in  { 
                    self.state.is_logged_in = false
                }
                true
            }
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        // let onclick = ctx.link().callback(|_| Msg::SignIn);



        // return html! {
        //     <>
                
        //        {self.state.is_logged_in &&

        //             return html! { 
        //                 <>
        //                     <div>{"asdasdas"}</div>            
        //                 </>
        //             }
        //        }
        //     </>
        // }
        // return html! {
        //     <>
        //        { 
        //             Some(self.state.is_logged_in).filter(|_| false).unwrap_or(
        //                 return html! { 
        //                     <>
        //                         <div>{"Hellloo"}</div>
        //                     </>
        //                 }
        //             )     
        //         }
        //     </>
        // }
    //    return html! {
    //         <>
    //            { 
    //                 self.state.is_logged_in | false          
    //             }
    //         </>
    //     }



        // let message =  match self.state.is_logged_in { 
        //     true => { 
        //         html! {
        //             <>
        //                 <div>{"Welcome Philip Daquin"}</div>
        //                     <p>{"To sign out, press the button below"}</p>
        //                 <button onclick={ctx.link().callback(|_| Msg::SignOut)}>{"Sign Out"}</button>
        //             </>
        //         }
        //     },
        //     false => {
        //         html!{
        //             <>
        //                 <div>{"Welcome Guest"}</div>
        //                     <p>{"To sign in, press the button below"}</p>
        //                 <button onclick={onclick}>{"Sign In"}</button>
        //             </>
    
        //         }
        //     }
        // };
        // match self.state.is_logged_in { 
        //     true => { 
        //         html! {
        //             <>
        //                 <div>{"Welcome Philip Daquin"}</div>
        //                     <p>{"To sign out, press the button below"}</p>
        //                 <button onclick={ctx.link().callback(|_| Msg::SignOut)}>{"Sign Out"}</button>
        //             </>
        //         }
        //     },
        //     false => {
        //         html!{
        //             <>
        //                 <div>{"Welcome Guest"}</div>
        //                     <p>{"To sign in, press the button below"}</p>
        //                 <button onclick={onclick}>{"Sign In"}</button>
        //             </>
        //         }
        //     }
        // }
        // html! {
        //     {message}
        // }
    }
}