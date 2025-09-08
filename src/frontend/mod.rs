mod view;

use dioxus::prelude::*;
pub use view::DogView;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    DogView,

    /* example catch-all for invalid URL;
    // TODO: implement the component
    #[route("/..segments")]
    PageNotFound { segments: Vec<String> },
    */
}
