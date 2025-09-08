use crate::frontend::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::DogView,
                h1 { "HotDog!" }
            }
            Link { id: "favorites",
                to: Route::Favorites,
                "\u{2764}\u{FE0F} Favorites \u{2764}\u{FE0F}" } // heart emojis
        }
        Outlet::<Route> {}
    }
}
