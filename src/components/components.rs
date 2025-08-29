use dioxus::prelude::*;
use super::dog_api::DogApi;

static IMG_SRC: GlobalSignal<String> = Signal::global(|| "".to_string());

#[component]
pub fn DogView() -> Element {
    rsx! {
        div { id: "dogview",
            img { id: "image",
                src: "{ IMG_SRC }",
                max_height: "380px",
            }
        }
    }
}

#[component]
pub fn Buttons() -> Element {
    let fetch_new = move |_: Event<MouseData>| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();

        *IMG_SRC.write() = response.message;
    };

    rsx! {
        div {id: "buttons",
            button { id: "skip", 
                onclick: fetch_new, 
                "skip" }
            button { id: "save", "save" }
        }
    }
}
