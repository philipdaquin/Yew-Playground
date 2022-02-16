use yew::{prelude::*, function_component, html, Html};
use std::rc::Rc;


pub enum Action { 
    Increment, 
    Decrement,
    Reset

}
//  Reducer's State
#[derive(Clone, Debug, PartialEq)]
pub struct Counter { 
    pub value: i32,
    pub default: i32
}

impl Default for Counter { 
    fn default() -> Counter { 
        Self { 
            value: 0,
            default: 0
        }
    }
}

impl Reducible for Counter  {
    /// The action type  
    /// The dispatch function takes one argument of type Action 
    type Action = Action;

    /// When called, the action adn current value are passed to the reducer
    /// function which computes a new state that is returned and the component is re-rendered
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> { 
       let curr = match action { 
           Increment => {self.value + 1},
           Decrement => {self.value -1},
           Reset => {self.default}
       };
       Counter  { 
           value: curr,
           default: self.default
       }.into()
    }
}

#[function_component(CounterTwo)]
pub fn countertwo() -> Html {
    
    let counter = use_reducer_eq(Counter::default);

    let increment ={ 
        let counter = counter.clone();
        Callback::from(move |_|  counter.dispatch(Action::Increment))
    };
    let decrement = { 
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(Action::Decrement))
    };
    let reset = { 
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(Action::Reset))
    };

    
    html! {
        <>
            <div>
                <h1>{counter.value}</h1>
                <button onclick={increment}>{"Increment"}</button>
                <button onclick={decrement}>{"Decrement"}</button>
                <button onclick={reset}>{"Reset"}</button>
            </div>
        </>
    }
}