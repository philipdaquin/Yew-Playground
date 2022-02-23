use web_sys::HtmlInputElement;
use yew::{
    events::Event,
    function_component, html, use_mut_ref, use_state,
    Callback, TargetCast,
};

#[function_component(UseMutRef)]
pub fn mut_ref_hook() -> Html {
    let message = use_state(|| "".to_string());
    let message_count = use_mut_ref(|| 0);

    let onclick = Callback::from(move |_| {
        let window = gloo_utils::window();

        // if *message_count.borrow_mut() > 3 {
        //     window.alert_with_message("Message limit reached").unwrap();
        // } else {
        //     *message_count.borrow_mut() += 1;
        //     window.alert_with_message("Message sent").unwrap();
        // }

        


    });

    let onchange = {
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };

    html! {
        <div>
            <input {onchange} value={(*message).clone()} />
            <button {onclick}>{ "Send" }</button>
        </div>
    }
}