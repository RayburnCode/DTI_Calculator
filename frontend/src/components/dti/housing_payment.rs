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

    // Listen for global reset signal and clear housing payment fields
    let reset = use_context::<Signal<usize>>();
    use_effect(move || {
        if reset() > 0 {
            principal_interest.set(0.0);
            taxes.set(0.0);
            insurance.set(0.0);
            hoa.set(0.0);
            total_housing_payment.set(0.0);
        }
    });
    
    rsx! {
        div {
            class: "space-y-4",
            
            // Input grid
            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                
                div {
                    class: "flex flex-col",
                    label {
                        r#for: "principal_interest",
                        class: "text-sm font-medium text-gray-700 mb-1",
                        "Principal & Interest"
                    }
                    input {
                        r#type: "number",
                        id: "principal_interest",
                        name: "principal_interest",
                        placeholder: "0.00",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
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
                    class: "flex flex-col",
                    label {
                        r#for: "taxes",
                        class: "text-sm font-medium text-gray-700 mb-1",
                        "Property Taxes"
                    }
                    input {
                        r#type: "number",
                        id: "taxes",
                        name: "taxes",
                        placeholder: "0.00",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
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
                    class: "flex flex-col",
                    label {
                        r#for: "insurance",
                        class: "text-sm font-medium text-gray-700 mb-1",
                        "Homeowners Insurance"
                    }
                    input {
                        r#type: "number",
                        id: "insurance",
                        name: "insurance",
                        placeholder: "0.00",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
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
                    class: "flex flex-col",
                    label {
                        r#for: "hoa",
                        class: "text-sm font-medium text-gray-700 mb-1",
                        "HOA Fees"
                    }
                    input {
                        r#type: "number",
                        id: "hoa",
                        name: "hoa",
                        placeholder: "0.00",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
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
            
            // Total display
            div {
                class: "mt-6 pt-4 border-t border-gray-200",
                div {
                    class: "flex justify-between items-center",
                    span {
                        class: "text-lg font-semibold text-gray-700",
                        "Total Housing Payment:"
                    }
                    span {
                        class: "text-2xl font-bold text-blue-600",
                        "${total_housing_payment:.2}"
                    }
                }
            }
        }
    }
}