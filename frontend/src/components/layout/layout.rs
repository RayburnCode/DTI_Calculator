use dioxus::prelude::*;
use crate::Route;
use crate::components::layout::{Navbar};

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-900 text-gray-100 flex flex-col",
            // Header
            Navbar {}
            // Main Content Area
            div { class: "pt-20 px-4 sm:px-6 lg:px-12 py-8",
                div { class: "", Outlet::<Route> {} }
            }
        }
    }
} 