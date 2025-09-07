use dioxus::prelude::*;
use crate::backend::server::save_dog;
use crate::frontend::dog_api::DogApi;

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
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = save_dog(current).await;
                }, 
                "save" 
            }
        }
    }
}
