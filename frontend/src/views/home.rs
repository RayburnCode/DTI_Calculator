use dioxus::prelude::*;

use crate::components::dti::{Income, Debts, HousingPayment, Result};

#[component]
pub fn Home() -> Element {
    rsx! {
        div { id: "main-page", class: "max-w-6xl mx-auto p-6 space-y-8",
            // Header
            div { class: "text-center mb-8",
                h1 { class: "text-3xl font-bold mb-2", "Debt-to-Income Calculator" }
                p { class: "text-gray-200",
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

            // Debts Section
            div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                h2 { class: "text-2xl font-semibold mb-4 text-gray-800 border-b pb-2",
                    "Debts"
                }
                Debts {}
            }

            // Housing Payment Section
            div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                h2 { class: "text-2xl font-semibold mb-4 text-gray-800 border-b pb-2",
                    "Housing Payment"
                }
                HousingPayment {}
            }

            // Results Section
            div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                h2 { class: "text-2xl font-semibold mb-4 text-gray-800 border-b pb-2",
                    "Your Debt-to-Income Ratios"
                }
                Result {}
            }
        }
    }
}



