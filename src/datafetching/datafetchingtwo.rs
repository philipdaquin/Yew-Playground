use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, function_component, html, Html};
use reqwasm::http::*;

#[derive(Debug, Clone, PartialEq)]
pub struct InitialState { 
    pub loading: bool,
    pub error: String,
    pub post: Vec<String>
}

pub enum Action { 
    FetchSuccess,
    FetchError
}

impl Default for InitialState  {
    fn default() -> Self {
        let post: Vec<String> = vec![];
        Self { 
            loading: true,
            error: String::new(),
            post
        }
    }
}

impl Reducible for InitialState { 
    type Action = Action;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let msg = match action { 
            Action::FetchSuccess => { 
                Self { 
                    loading: false, 
                    error: String::new(),
                    post: Vec::new(),
                }
            },
            Action::FetchError => { 
                Self { 
                    loading: false,
                    error: "Something went wrong!".to_string(),
                    post: Vec::new(),
                }
            }
        };
        msg.into()
    }
}


#[function_component(DataFetchingTwo)]
pub fn datafetchingtwo() -> Html {
    let state = use_reducer(InitialState::default);

    use_effect_with_deps(
         spawn_local( async move |_| { 
            let state = state.clone();
            Request::get("http://api.open-notify.org/iss-now.json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            Callback::from(move |_| state.dispatch(Action::FetchSuccess));
        )
                
        }, ());

    html! {
        <>
            <div>

            </div>
        </>
    }
}