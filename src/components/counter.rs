use yew::prelude::*;

pub struct Counter { 
    state: State
}
struct State { 
    count: i64
}
pub enum Msg { 
    Increment,
    Decrement,
    Multiply
}


impl Component for Counter { 
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let state = State { 
            count: 0
        };
        
        Self { 
            state
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::Increment => { 
                self.state.count += 1;
                true
            }, 
            Msg::Decrement => { 
                self.state.count -= 1;
                true
            },
            Msg::Multiply => { 
                self.state.count *= 2;
                true
            }
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let State {count } = self.state;
        let increment = ctx.link().callback(|_| Msg::Increment);
        let decrement = ctx.link().callback(|_| Msg::Decrement);
        let batch_call = ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment]);
        let multiply = ctx.link().callback(|_| Msg::Multiply);
        
        
        
        
        html! {
            <>
                <div>
                    <h1>{format!("Counter: {}", count)}</h1>
                    <button onclick={increment}>{"Increment"}</button>
                    <button onclick={decrement}>{"Decrement"}</button>
                    <button onclick={batch_call}>{ "+1, +1" }</button>
                    <button onclick={multiply}>{"Multiply by 2"}</button>

                </div>
            </>
        }

    }

}