use yew::{prelude::*, function_component, html, Html};

#[function_component(FocusInput)]
pub fn foucsinput() -> Html {
    
    // let input = use_ref(|| None);
    // use_effect_with_deps(
    //     move |_| { 
    //         let input = input.clone();
    //         input.target_unchecked_into()
    //     }, || vec![]);

    html! {
        <>
            <div>
                <input type="text"
                    placeholder="Insert Text Here"
                />
            </div>
        </>
    }
}