use yew::{prelude::*, function_component, html, Html};
use rand::Rng; // 0.8.0



#[derive(Clone, Debug, Default)]
pub struct HookCounter { 
    pub id: i32, 
    pub value: i32
}


#[function_component(HookCounterFour)]
pub fn counterhook4() -> Html {
    
    let item = use_state(|| HookCounter { 
        id: 0,
        value: 0
    });
    // let set_items = { 
    //     let item = item.clone();
    //     Callback::from(move |(_)|  {
    //         let mut info = (*item).clone();
    //         info.id = 0i32;
    //         info.value = 0i32
    //     })
    // };
    let add_item = { 
        let item = item.clone();
        Callback::from(move |_| { 
            let mut info = (*item).clone();
            info.id = info.id + 1;
            info.value = rand::thread_rng().gen_range(0..10)
        })
    };



    html! {
        <>
        <div>
            <button onclick={add_item}>{"Add A Number"}</button>
            <ul>
            
                    // for item.map(|item| { 
                    //     html! { 

                    //     }
                    // }).iter()
               
            </ul>
        </div>
        </>
    }
}