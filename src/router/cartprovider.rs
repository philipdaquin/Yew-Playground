use yew::{prelude::*, function_component, html, Html};

 #[derive(Debug, PartialEq, Clone)]
pub struct Product { 
    id: i32,
    name: String,
    description: String,
    image: String,
    price: f64,
}
//  State of Cart 
#[derive(Debug, PartialEq, Clone)]
pub struct CartProduct {
    pub products: Product,
    pub quantity: i32
}
#[derive(Debug, PartialEq, Clone)]
pub struct CartState { 
    pub cart_products: Vec<CartProduct>
} 
pub enum Action   {
    AddToCart(Product)
}

impl Product { 
    fn new(item: &Self) -> Self { 
        Self {
            id: item.id, 
            name: item.name, 
            description: item.description,
            image: item.image, 
            price: item.price
        }
    }
}



impl Reducible for CartState {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        
        // let msg = match action { 
        //     Action::AddToCart => { 
        //         // let cart = self.cart_products.clone().push(CartProduct { 
        //         //     products
        //         //     quantity: 1
        //         // });
        //         // cart
        //     }
        // };
        Self { 
            cart_products: self.cart_products.clone()
        }.into()
    }
}




pub type CartStateContext = UseReducerHandle<CartState>;
pub type CartDispatchContext = UseReducerDispatcher<CartProduct>;

#[derive(Clone, PartialEq, Properties)]
pub struct CartProviderProp { 
    pub children: Children
}

#[function_component(CartProvider)]
pub fn cartprovider(props: &CartProviderProp) -> Html {
    let state = use_reducer( || CartState { 
        cart_products: Vec::<CartProduct>::new()
    });
    // let dispatch = { 
    //     let state = state.clone();
    //     Callback::from(move |_| state.dispatch(Action::AddToCart(Product::new(self))))
    // };
    html! {
        <>
            // <ContextProvider<CartDispatchContext> context={dispatch}>
            //     <ContextProvider<CartStateContext> context={state}>
            //         {props.children.clone()}
            //     </ContextProvider<CartStateContext>>
            // </ContextProvider<CartDispatchContext>>
        </>
    }
}