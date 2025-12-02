use dioxus::prelude::*;

use crate::components::dti::{Income, HousingPayment};

#[component]
pub fn MainPage() -> Element {
    rsx! {
        div { id: "main-page",

            h1 { "Welcome to the Main Page!" }
            Income {}
        }

        HousingPayment {}
    }
}



