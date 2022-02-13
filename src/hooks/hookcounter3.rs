use web_sys::HtmlInputElement;
use yew::{prelude::*, function_component, html, Html};


#[derive(Clone, Debug, Default)]
pub struct PersonInfo { 
    pub first_name: String, 
    pub last_name: String
}


#[function_component(HookCounterThree)]
pub fn hookthree() -> Html {
    
    let person_info = use_state(PersonInfo::default);
    let oninput_first_name = { 
        let person_info = person_info.clone();
        Callback::from(move |e: InputEvent| {
            let mut input: HtmlInputElement = e.target_unchecked_into(); 
            let mut info = (*person_info).clone();
            info.first_name = input.value();
            person_info.set(info)
        })
    };
    let oninput_last_name = { 
        let person_info = person_info.clone();
        Callback::from(move |e: InputEvent| {
            let mut input: HtmlInputElement = e.target_unchecked_into(); 
            let mut info = (*person_info).clone();
            info.last_name = input.value();
            person_info.set(info)
        })
    };
    
    
    html! {
        <>
        <form>
            <input type="text"
                placeholder ="First Name"
                value={person_info.first_name.clone()}
                oninput={oninput_first_name}
            />
            <input type="text"
                placeholder ="Last Name"
                value={person_info.last_name.clone()}
                oninput={oninput_last_name}
            />
            <h2>{format!("Your First Name is: {} ", person_info.first_name)}</h2>
            <h2>{format!("You Last Name is: {} ", person_info.last_name)}</h2>
        </form>
        </>
    }
}