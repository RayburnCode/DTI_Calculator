//Description, Type, Monthly Amount


use dioxus::prelude::*;


#[component]
pub fn Income() -> Element {
    let total_income = use_signal(|| 0.0);
    
    rsx! {
        div {
            p { "Income" }
            input {
                r#type: "text",
                placeholder: "Description",
                class: "m-2 p-2 rounded-md border",
            }
            input {
                r#type: "text",
                placeholder: "Type",
                class: "m-2 p-2 rounded-md border",
            }
            input {
                r#type: "amount",
                placeholder: "Monthly Amount",
                class: "m-2 p-2 rounded-md border",
            }
            button { "Add Income" }
        }

        div {
            p { "Total Income: $" }
            p { "{total_income}" }
        }
    }
}