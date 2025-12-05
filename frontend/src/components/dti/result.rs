use dioxus::prelude::*;

#[component]
pub fn Result() -> Element {
    // Get totals from context
    let total_income = use_context::<Signal<f64>>();
    let total_debt = use_context::<Signal<f64>>();
    let total_housing = use_context::<Signal<f64>>();
    
    // Calculate DTI ratios
    let frontend_dti = use_memo(move || {
        let income = total_income();
        if income > 0.0 {
            (total_housing() / income) * 100.0
        } else {
            0.0
        }
    });
    
    let backend_dti = use_memo(move || {
        let income = total_income();
        if income > 0.0 {
            ((total_housing() + total_debt()) / income) * 100.0
        } else {
            0.0
        }
    });
    
    // Check if we have any data entered
    let has_data = use_memo(move || {
        total_income() > 0.0 || total_debt() > 0.0 || total_housing() > 0.0
    });
    
    rsx! {
        if has_data() {
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                // Frontend DTI Card
                div { class: "bg-gradient-to-br from-blue-50 to-blue-100 rounded-lg shadow-lg p-6 border-2 border-blue-300",
                    h3 { class: "text-xl font-bold text-blue-900 mb-2", "Front-End DTI" }
                    p { class: "text-sm text-blue-700 mb-4", "Housing expenses only" }
                    div { class: "text-center",
                        p { class: "text-5xl font-bold text-blue-600 mb-2", "{frontend_dti():.2}%" }
                        p { class: "text-xs text-blue-600 mt-2", "Mortgage PITI ÷ Gross Income" }
                    }
                    div { class: "mt-4 pt-4 border-t border-blue-300",
                        if frontend_dti() <= 28.0 {
                            p { class: "text-sm text-green-700 font-semibold",
                                "✓ Excellent - Within recommended range"
                            }
                        } else if frontend_dti() <= 33.0 {
                            p { class: "text-sm text-yellow-700 font-semibold",
                                "⚠ Caution - Above ideal range"
                            }
                        } else {
                            p { class: "text-sm text-red-700 font-semibold",
                                "⚠ High - May affect loan approval"
                            }
                        }
                    }
                }
                // Backend DTI Card
                div { class: "bg-gradient-to-br from-purple-50 to-purple-100 rounded-lg shadow-lg p-6 border-2 border-purple-300",
                    h3 { class: "text-xl font-bold text-purple-900 mb-2", "Back-End DTI" }
                    p { class: "text-sm text-purple-700 mb-4", "All monthly debt payments" }
                    div { class: "text-center",
                        p { class: "text-5xl font-bold text-purple-600 mb-2", "{backend_dti():.2}%" }
                        p { class: "text-xs text-purple-600 mt-2",
                            "(Mortgage PITI + Debts) ÷ Gross Income"
                        }
                    }
                    div { class: "mt-4 pt-4 border-t border-purple-300",
                        if backend_dti() <= 36.0 {
                            p { class: "text-sm text-green-700 font-semibold",
                                "✓ Excellent - Within recommended range"
                            }
                        } else if backend_dti() <= 43.0 {
                            p { class: "text-sm text-yellow-700 font-semibold",
                                "⚠ Caution - Higher risk"
                            }
                        } else {
                            p { class: "text-sm text-red-700 font-semibold",
                                "⚠ High - May impact approval"
                            }
                        }
                    }
                }
            }
        } else {
            div { class: "bg-gray-50 rounded-lg shadow-md p-8 text-center border border-gray-200",
                p { class: "text-gray-600 text-lg",
                    "Enter your income, housing payment, and debts above to see your DTI ratios"
                }
            }
        }
    }
}
