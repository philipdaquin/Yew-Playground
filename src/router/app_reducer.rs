use yew::{prelude::*, function_component, html, Html};
use crate::router;
use crate::use_reducer::{
    counterone::CounterOne, 
    countertwo::CounterTwo,
    counterthree::CounterThree
};

use crate::fetching_data::data_fetching1::DataFetchOne;

use crate::use_reducer::{
    compa::CompA,
    compb::CompB,
    compc::CompC
};

use std::rc::Rc;

pub struct Product { 
    pub id: i32,
    pub name: String,
    pub description: String, 
    pub image: String,
    pub price: f32
}

pub enum Action { 
    AddToCart(i32), 
    Decrement,
    Increment
}
//  Reducer's State
#[derive(Clone, Debug, PartialEq)]
pub struct Subtotal { 
    pub value: i32,
    pub quantity: i32
}


impl Default for Subtotal { 
    fn default() -> Subtotal { 
        Self { 
            value: 0,
            quantity: 0
        }
    }
}
pub type SubtotalContext = UseReducerHandle<Subtotal>;
impl Reducible for Subtotal  {
    /// The action type  
    /// The dispatch function takes one argument of type Action 
    type Action = Action;

    /// When called, the action adn current value are passed to the reducer
    /// function which computes a new state that is returned and the component is re-rendered
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> { 
       let curr = match action { 
            Action::AddToCart(x) => {
               self.value + x;
               self.quantity + 1
            },
            Action::Increment => {self.quantity + 1},
            Action::Decrement => {self.quantity - 1}
       };
       Subtotal  { 
           value: curr,
           quantity: curr
       }.into()
    }
}

#[function_component(AppReducer)]
pub fn reducers() -> Html {
    let subtotal = use_reducer_eq(Subtotal::default);

    let addtocart = { 
        let subtotal =  subtotal.clone();
        Callback::from(move |e: MouseEvent| subtotal.dispatch(Action::AddToCart(10)));
    };

    html! {
        <>
            <div>
                // <CounterOne/>
                // <CounterTwo/>
                // <CounterThree/>
                
                <h1>{"Subtotal: "}{subtotal.value}</h1>
                <h1>{"Total Items: "}{subtotal.quantity}</h1>

                <ContextProvider<SubtotalContext> context={subtotal}>
                    <CompA/>
                    <CompB/>
                    <CompC/>
                </ ContextProvider<SubtotalContext>>


                <DataFetchOne/>


            </div>
        </>
    }
}