use dioxus::prelude::*;

use crate::views::{Blog, Home, MainPage};
use crate::components::layout::Navbar;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/main")]
        MainPage {},
        #[route("/blog/:id")]
        Blog { id: i32 },
}