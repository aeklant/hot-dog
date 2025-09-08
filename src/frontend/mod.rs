mod favorites;
mod nav;
mod view;

use dioxus::prelude::*;
pub use favorites::Favorites;
pub use nav::NavBar;
pub use view::DogView;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,
    #[route("/favorites")]
    Favorites,
    /* example catch-all for invalid URL;
    // TODO: implement the component
    #[route("/..segments")]
    PageNotFound { segments: Vec<String> },
    */
}
