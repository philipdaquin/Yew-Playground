use gloo::events::EventListener;
use yew::{prelude::*, function_component, html, Html};
use gloo_console::{self as console, 

    


};
use gloo_utils::{window};
#[function_component(HookMouse)]
pub fn hook_mouse() -> Html {
    
    let x = use_state(|| 0);
    let y = use_state(|| 0);

    let set_x = {
        let x = x.clone();
    };
    let set_y = {
        let y = y.clone();

    };

   

    {
        let log_mouse_movement = Callback::from( move |e: MouseEvent| { 
            console::log!("Mouse Movement Initiated!");
            x.set(e.client_x());
            y.set(e.client_y());
        });  
        
        // use_effect_with_deps(
        //     move |_| { 
        //         console::log!("Use Effect called!");
        //             let window = windows();
        //             let onmove = EventListener::new(&window, "mousemove", move |e: MouseEvent| { 
        //                 let event = e.dyn_ref().unwrap_thow();
                        
        //             });

        //     }, || ())
    }

    html! {
        <>
            <div> 
                // {*x}{*y}
            </div>
        </>
    }
}