use dioxus::prelude::*;
use crate::components::SearchBar;

#[component]
pub fn MainPage() -> Element {
    rsx! {
        div { id: "main-page",

            h1 { "Welcome to the Main Page!" }

            SearchBar {}

            button {
                // Global html attributes
                class: "button",
                "data-style": "outline",
                        // {children}
            }
        }
    }
}



