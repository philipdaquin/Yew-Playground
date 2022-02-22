use yew::{prelude::*, function_component, html, Html};

#[function_component(Counter)]
pub fn counter() -> Html {
    let counterone = use_state(|| 0);
    let countertwo = use_state(|| 0);
    
    let set_counterone = {
        let counterone = counterone.clone();
        Callback::from(move |e: MouseEvent| counterone.set(0))
    };

    let set_countertwo = {
        let countertwo = countertwo.clone();
        Callback::from(move |e: MouseEvent| countertwo.set(0))
    };

    let increment_one = { 
         let counterone = counterone.clone();
        Callback::from(move |e: MouseEvent| counterone.set(*counterone + 1))
    };

    let increment_two = { 
        let countertwo = countertwo.clone();
        Callback::from(move |e: MouseEvent| countertwo.set(*countertwo + 1))
    };

    


    let is_even = {
        let (counterone, countertwo) = (counterone.clone(), countertwo.clone());
        
        let mut i = 0;
        // while i < 200000000 { 
        //     i +=1
        // }
        
        if *counterone % 2 == 0 { 
            html! { "counterone is even 😍😍" }
        } else { 
            html! { "counterone is odd ⭐⭐" }
        }
    };


    html! {
        <>
            <div>
                <div>
                    <button 
                        onclick={increment_one}>
                        {"Count One - "}{(*counterone).clone()}
                    </button>
                    <span>{is_even}</span>
                </div>
                <div>
                    <button 
                        onclick={increment_two}>
                        {"Count Two - "}{(*countertwo).clone()}
                    </button>
                </div>
            </div>
        </>
    }
}