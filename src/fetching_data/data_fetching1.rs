use reqwasm::http::Request;
use serde::Deserialize;
use yew::{prelude::*, function_component, html, Html};

#[derive(Clone, PartialEq, Deserialize)]
pub struct Video {
    pub id: usize,
    pub title: String, 
    pub speaker: String, 
    pub url: String
}

#[derive(Clone, PartialEq, Properties)]
pub struct VideoProps { 
    #[prop_or_default]
    pub videos: Vec<Video>
}


#[function_component(DataFetchOne)]
pub fn datafetchone(VideoProps { videos  }: &VideoProps) -> Html {
    let videos = vec![
    Video {
        id: 1,
        title: "Building and breaking things".to_string(),
        speaker: "John Doe".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 2,
        title: "The development process".to_string(),
        speaker: "Jane Smith".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 3,
        title: "The Web 7.0".to_string(),
        speaker: "Matt Miller".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 4,
        title: "Mouseless development".to_string(),
        speaker: "Tom Jerry".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
];
    let videos_use_effect = use_state(|| Vec::<Video>::new());

    {
        let videos_use_effect = videos_use_effect.clone();
        use_effect_with_deps(move |_| { 
            wasm_bindgen_futures::spawn_local(async move{ 
                let fetched_videos: Vec<Video> = 
                    Request::get("https://yew.rs/tutorial/data.json")
                    .send().await.unwrap().json().await.unwrap();
                videos_use_effect.set(fetched_videos);
            }); 
            || ()
        }, ())
    }   


    html! {
        <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
            <p>{ "John Doe: Building and breaking things" }</p>
            <p>{ "Jane Smith: The development process" }</p>
            <p>{ "Matt Miller: The Web 7.0" }</p>
            <p>{ "Tom Jerry: Mouseless development" }</p>
        </div>
        <div>
            <h3>{ "John Doe: Building and breaking things" }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>


        <div>
            <h3>{"Videos To Watch!"}</h3>
            <DataFetchOne videos={(*videos_use_effect).clone()}/>
        </div>



    </>
    }
}