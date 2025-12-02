//Description, Type, Monthly Amount


use dioxus::prelude::*;


#[component]
pub fn HousingPayment() -> Element {
    let mut principal_interest = use_signal(|| 0.0);
    let mut taxes = use_signal(|| 0.0);
    let mut insurance = use_signal(|| 0.0);
    let mut hoa = use_signal(|| 0.0);
    let mut total_housing_payment = use_signal(|| 0.0);
    
    // Calculate total whenever any value changes
    use_effect(move || {
        let total = principal_interest() + taxes() + insurance() + hoa();
        total_housing_payment.set(total);
    });
    
    rsx! {
        div {
            p { "Housing Payment" }
            div {
                label { r#for: "principal_interest", "Principal & Interest" }
                input {
                    r#type: "number",
                    id: "principal_interest",
                    name: "principal_interest",
                    placeholder: "Principal & Interest",
                    class: "m-2 p-2 rounded-md border",
                    oninput: move |evt| {
                        if let Ok(value) = evt.value().parse::<f64>() {
                            principal_interest.set(value);
                        } else {
                            principal_interest.set(0.0);
                        }
                    },
                }
            }
            div {
                label { r#for: "taxes", "Taxes" }
                input {
                    r#type: "number",
                    id: "taxes",
                    name: "taxes",
                    placeholder: "Taxes",
                    class: "m-2 p-2 rounded-md border",
                    oninput: move |evt| {
                        if let Ok(value) = evt.value().parse::<f64>() {
                            taxes.set(value);
                        } else {
                            taxes.set(0.0);
                        }
                    },
                }
            }
            div {
                label { r#for: "insurance", "Insurance" }
                input {
                    r#type: "number",
                    id: "insurance",
                    name: "insurance",
                    placeholder: "Insurance",
                    class: "m-2 p-2 rounded-md border",
                    oninput: move |evt| {
                        if let Ok(value) = evt.value().parse::<f64>() {
                            insurance.set(value);
                        } else {
                            insurance.set(0.0);
                        }
                    },
                }
            }
            div {
                label { r#for: "hoa", "HOA" }
                input {
                    r#type: "number",
                    id: "hoa",
                    name: "hoa",
                    placeholder: "HOA",
                    class: "m-2 p-2 rounded-md border",
                    oninput: move |evt| {
                        if let Ok(value) = evt.value().parse::<f64>() {
                            hoa.set(value);
                        } else {
                            hoa.set(0.0);
                        }
                    },
                }
            }
        }

        div {
            p { "Total Housing Payment: ${total_housing_payment:.2}" }
        }
    }
}