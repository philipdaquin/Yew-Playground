use yew::{prelude::*, function_component, html, Html};
use std::rc::Rc;



pub enum Action { 
    Increment, 
    Decrement,
    Reset

}
//  Reducer's State
#[derive(Clone, Debug, PartialEq)]
pub struct Counter1 { 
    pub value: i32,
    pub default: i32
}

impl Default for Counter1 { 
    fn default() -> Counter1 { 
        Self { 
            value: 0,
            default: 0
        }
    }
}
/////////////////////////////////////////////////////////////
//  Reducer's State
#[derive(Clone, Debug, PartialEq)]
pub struct Counter2 { 
    pub value: i32,
    pub default: i32
}

impl Default for Counter2 { 
    fn default() -> Counter2 { 
        Self { 
            value: 0,
            default: 0
        }
    }
}


impl Reducible for Counter2  {
    /// The action type  
    /// The dispatch function takes one argument of type Action 
    type Action = Action;
    /// When called, the action adn current value are passed to the reducer
    /// function which computes a new state that is returned and the component is re-rendered
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> { 
       let curr = match action { 
           Action::Increment => {self.value + 1},
           Action::Decrement => {self.value -1},
           Action::Reset => {self.default}
       };
       Counter2  { 
           value: curr,
           default: self.default
       }.into()
    }
}

////////////////////////////////////////////////////////////

impl Reducible for Counter1  {
    /// The action type  
    /// The dispatch function takes one argument of type Action 
    type Action = Action;

    /// When called, the action adn current value are passed to the reducer
    /// function which computes a new state that is returned and the component is re-rendered
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> { 
       let curr = match action { 
           Action::Increment => {self.value + 1},
           Action::Decrement => {self.value -1},
           Action::Reset => {self.default}
       };
       Counter1  { 
           value: curr,
           default: self.default
       }.into()
    }
}

#[function_component(CounterThree)]
pub fn counterone() -> Html {
    
    let counter = use_reducer_eq(Counter1::default);
    let count2 = use_reducer_eq(Counter2::default);

    let increment ={ 
        let counter = counter.clone();
        Callback::from(move |e: MouseEvent|  counter.dispatch(Action::Increment))
    };
    let decrement = { 
        let counter = counter.clone();
        Callback::from(move |e: MouseEvent| counter.dispatch(Action::Decrement))
    };
    let reset = { 
        let counter = counter.clone();
        Callback::from(move |e: MouseEvent| counter.dispatch(Action::Reset))
    };




    let increment2 ={ 
        let count2 = count2.clone();
        Callback::from(move |e: MouseEvent|  count2.dispatch(Action::Increment))
    };
    let decrement2 = { 
        let count2 = count2.clone();
        Callback::from(move |e: MouseEvent| count2.dispatch(Action::Decrement))
    };
    let reset2 = { 
        let count2 = count2.clone();
        Callback::from(move |e: MouseEvent| count2.dispatch(Action::Reset))
    };

    
    html! {
        <>
            <div>
                <h1>{"Count 1 --"}{counter.value}</h1>
                <button onclick={increment}>{"Increment"}</button>
                <button onclick={decrement}>{"Decrement"}</button>
                <button onclick={reset}>{"Reset"}</button>
            </div>
            <div>
                <h1>{"Count 2 --"}{count2.value}</h1>
                <button onclick={increment2}>{"Increment"}</button>
                <button onclick={decrement2}>{"Decrement"}</button>
                <button onclick={reset2}>{"Reset"}</button>
            </div>
        </>
    }
}