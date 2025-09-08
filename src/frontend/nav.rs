use crate::frontend::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::DogView,
                h1 { "Dogger!" }
            }
        }
        Outlet::<Route> {}
    }
}
