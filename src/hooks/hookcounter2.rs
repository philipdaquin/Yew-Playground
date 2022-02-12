use yew::{prelude::*, function_component, html, Html};

#[function_component(HookCounterTwo)]
pub fn counter() -> Html {
    let initial_count = 0;
    let count = use_state_eq(|| initial_count);
    let set_count = |val: i32| {val};


    let onset = {
        let count = count.clone();
        Callback::from(move |e: MouseEvent| count.set(initial_count))
    };
    let increment = {
        let count = count.clone();
        Callback::from(move |e: MouseEvent| count.set(*count + 1))
    };
    let decrement = {
        let count = count.clone();
        Callback::from(move |e: MouseEvent| count.set(*count - 1))
    };
    let increment_by_5 ={
        let prev_count = count.clone();
        Callback::from(move |e: MouseEvent| prev_count.set(*prev_count + 5))
    };
    

    html! {
        <>
            <div>
                <h1>{"Count: "}{*count}</h1>
                <button onclick={onset}>{"Reset"}</button>
                <button onclick={increment}>{"Increment"}</button>
                <button onclick={decrement}>{"Decrement"}</button>
                <button onclick={increment_by_5}>{"Increment by 5"}</button>
            </div>
        </>
    }
}
