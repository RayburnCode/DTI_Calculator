use dioxus::prelude::*;

use crate::components::dti::{Income, Debts, HousingPayment};

#[component]
pub fn Home() -> Element {
    let mut dti_result = use_signal(|| None::<f64>);
    // clear result on global reset
    let reset = use_context::<Signal<usize>>();
    use_effect(move || {
        if reset() > 0 {
            dti_result.set(None);
        }
    });

    rsx! {
        div { id: "main-page", class: "max-w-6xl mx-auto p-6 space-y-8",
            // Header
            div { class: "text-center mb-8",
                h1 { class: "text-3xl font-bold mb-2", "Debt-to-Income Calculator" }
                p { class: "text-gray-800",
                    "Enter your income and debts above to calculate your DTI ratio"
                }
            }

            // Income Section
            div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                h2 { class: "text-2xl font-semibold mb-4 text-gray-800 border-b pb-2",
                    "Income"
                }
                Income {}
            }

            // Housing Payment Section
            div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                h2 { class: "text-2xl font-semibold mb-4 text-gray-800 border-b pb-2",
                    "Housing Payment"
                }
                HousingPayment {}
            }

            // Debts Section
            div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                h2 { class: "text-2xl font-semibold mb-4 text-gray-800 border-b pb-2",
                    "Debts"
                }
                Debts {}
            }

            // Results Section (placeholder for future DTI calculation)
            div { class: "bg-blue-50 rounded-lg shadow-md p-6 border border-blue-200",
                h2 { class: "text-2xl font-semibold mb-4 text-blue-800 border-b border-blue-300 pb-2",
                    "Your Debt-to-Income Ratio"
                }
                if let Some(dti) = dti_result.read().as_ref() {
                    p { class: "text-gray-800 text-xl font-bold", "{dti}%" }
                } else {
                    p { class: "text-gray-800", "Enter your income and debts above to calculate your DTI ratio" }
                }
            }
        }
    }
}



