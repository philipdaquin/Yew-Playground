use yew::{prelude::*, function_component, html, Html};
use std::rc::Rc;


pub enum Action { 
    Increment, 
    IncrementBy(i32),
    Decrement,
    DecrementBy(i32),
    Reset,

    Increment2, 
    IncrementBy2(i32),
    Decrement2,
    DecrementBy2(i32),
    Reset2,

}
//  Reducer's State
#[derive(Clone, Debug, PartialEq)]
pub struct Counter { 
    pub value: i32,
    pub second_counter: i32,
    pub default: i32, 
}

impl Default for Counter { 
    fn default() -> Counter { 
        Self { 
            value: 0,
            second_counter: 0,
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
            Action::Increment => { self.value + 1},
            Action::Decrement => {self.value -1},
            Action::Reset => {self.default},
            Action::IncrementBy(x)  => {self.value + x},
            Action::DecrementBy(x) => {self.value - x},
            Action::Increment2 => {self.second_counter + 1 },
            Action::Decrement2 => {self.second_counter -1},
            Action::Reset2 => {self.default},
            Action::IncrementBy2(x)  => {self.second_counter + x},
            Action::DecrementBy2(x) => {self.second_counter - x}
           
       };
       Counter  { 
           value: curr,
           second_counter: curr,
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
    let decrement_by = { 
        let counter = counter.clone();
        Callback::from(move |e: MouseEvent| counter.dispatch(Action::DecrementBy(5)))
    };
    let increment_by = { 
        let counter = counter.clone();
        Callback::from(move |e: MouseEvent| counter.dispatch(Action::IncrementBy(5)))
    };


    let increment2 ={ 
        let counter = counter.clone();
        Callback::from(move |_|  counter.dispatch(Action::Increment2))
    };
    let decrement2 = { 
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(Action::Decrement2))
    };
    let reset2 = { 
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(Action::Reset2))
    };






   
    
    html! {
        <>
            <div>
                <h1>{"First Counter: "}{counter.value}{"Second Counter: "}{counter.second_counter}</h1>
                <button onclick={increment}>{"Increment"}</button>
                <button onclick={decrement}>{"Decrement"}</button>
                <button onclick={reset}>{"Reset"}</button>
                <button onclick={decrement_by}>{"Decrement by 5"}</button>
                <button onclick={increment_by}>{"Increment by 5"}</button>



                <button onclick={increment2}>{"Increment2"}</button>
                <button onclick={decrement2}>{"Decrement"}</button>
                <button onclick={reset2}>{"Reset"}</button>
            </div>
        </>
    }
}