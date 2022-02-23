use yew::{prelude::*, function_component, html, Html};





use yew_hooks::use_counter;
#[function_component(Counter)]
fn counter() -> Html {
    let counter = use_counter(0);
    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };
    let ondecrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease())
    };
    let onincreaseby = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase_by(10))
    };
    let ondecreaseby = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease_by(10))
    };
    let onset = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(100))
    };
    let onreset = {
        let counter = counter.clone();
        Callback::from(move |_| counter.reset())
    };
    
    html! {
        <div>
            <button onclick={onincrease}>{ "Increase" }</button>
            <button onclick={ondecrease}>{ "Decrease" }</button>
            <button onclick={onincreaseby}>{ "Increase by 10" }</button>
            <button onclick={ondecreaseby}>{ "Decrease by 10" }</button>
            <button onclick={onset}>{ "Set to 100" }</button>
            <button onclick={onreset}>{ "Reset" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}