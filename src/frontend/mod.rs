mod nav;
mod view;

use dioxus::prelude::*;
pub use view::DogView;
pub use nav::NavBar;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    /* example catch-all for invalid URL;
    // TODO: implement the component
    #[route("/..segments")]
    PageNotFound { segments: Vec<String> },
    */
}
