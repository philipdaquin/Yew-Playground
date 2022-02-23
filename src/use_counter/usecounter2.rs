use yew::{prelude::*, function_component, html, Html};

#[function_component(Countertwo)]
pub fn countertwo() -> Html {

    let state = use_state(|| 0);
    let increment = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| state.set(*state  + 1))
    };
    let decrement = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| state.set(*state  - 1))
    };
    let reset = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| state.set(0))
    };

    html! {
        <>
            <div>
                <h2>{*state}</h2>
                <button onclick={increment}>{"Increment"}</button>
                <button onclick={decrement}>{"Decrement"}</button>
                <button onclick={reset}>{"Reset"}</button>


            </div>
        </>
    }
}