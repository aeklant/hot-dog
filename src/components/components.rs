use dioxus::prelude::*;
use super::dog_api::DogApi;

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { id: "image",
                src: img_src.cloned().unwrap_or_default(),
                max_height: "380px",
            }
        }
        div {id: "buttons",
            button { id: "skip", 
                onclick: move |_| img_src.restart(), 
                "skip" 
            }
            button { id: "save", 
                "save" 
            }
        }
    }
}
