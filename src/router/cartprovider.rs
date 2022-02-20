use yew::prelude::*;




#[derive(PartialEq, Properties, Clone)]
pub struct CartProviderProps { 
    #[prop_or_default]
    pub children: Children,
 
}
#[function_component(CartProvider)]
pub fn cartprovider(props: &CartProviderProps) -> Html {
    html! {
        <>  
            {props.children.clone()}
        </>
    }
}