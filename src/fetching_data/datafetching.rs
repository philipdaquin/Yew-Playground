use serde::Deserialize;
use yew::{prelude::*, function_component, html, Html};
use reqwasm::http::*;
#[derive(Default, PartialEq, Clone, Deserialize)]
pub struct User { 
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String
}

#[derive(Properties, PartialEq, Clone)]
pub struct UserProps { 
    #[prop_or_default]
    pub props: Vec<User>
}

#[function_component(DataFetching)]
pub fn datafetch(UserProps { props }: &UserProps) -> Html {

    let loading = use_state(|| false);
    let user_holder = use_state(||  vec![]);

    { 
        let user_holder = user_holder.clone();
        use_effect_with_deps(move |_| { 
            wasm_bindgen_futures::spawn_local(async move { 
                let fetched_info = Request::get("https://jsonplaceholder.typicode.com/posts")
                .send()
                .await
                .unwrap()
                .json()
                .await 
                .unwrap();
                user_holder.set(fetched_info)
            });
            || ()
        }, ())

    }

    let render: Vec<Html> = user_holder.iter().map(|x: &User| return html! { 
        <>
            <ul>
                <li>{format!("{}", x.user_id)}</li>
                <li>{format!("{}", x.id)}</li>
                <li>{format!("{}", x.title)}</li>
                <li>{format!("{}", x.body)}</li>


            </ul>
        </>
    }).collect();

    html! {
        <>
            <div>
                {render}

            </div>
        </>
    }
}