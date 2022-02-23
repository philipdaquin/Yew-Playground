use web_sys::HtmlInputElement;
use yew::{prelude::*, function_component, html, Html};

#[function_component(UseForm)]
pub fn useform() -> Html {
    let firstname = use_state(|| "".to_string());
    let lastname = use_state(|| "".to_string());

    let on_firstname = {
        let firstname = firstname.clone();
        Callback::from(move |e: Event| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*firstname).clone();
            info =  input.value();
            firstname.set(info)
        })
    };

    let on_lastname = {
        let lastname = lastname.clone();
        Callback::from(move |e: Event| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*lastname).clone();
            info =  input.value();
            lastname.set(info)
        })
    };

    let submit = {
        let lastname = lastname.clone();
        let firstname = firstname.clone();
        Callback::from(move |e: MouseEvent| { 
            e.prevent_default();
            let window = gloo_utils::window();
            window.alert_with_message(
                &format!(
                    " Hello Your First Name is {} and Last Name is {}", 
                    *lastname, *firstname
            )
            ).unwrap()
        })
    };
    
    
    html! {
        
        <>
            <div>
                <form>
                    <div>
                        <label>{"First Name"}</label>
                        <input type="text" 
                            value={(*firstname).clone()} onchange ={on_firstname}/>
                    </div>
                    <div>
                        <label>{"Last Name"}</label>
                        <input type="text" 
                            value={(*lastname).clone()} onchange ={on_lastname}/>
                    </div>
                    <button onclick={submit}>{"Submit"}</button>
                </form>
            </div>
        </>
    }
}