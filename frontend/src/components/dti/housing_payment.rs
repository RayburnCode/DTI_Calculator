//Description, Type, Monthly Amount


use dioxus::prelude::*;


#[component]
pub fn HousingPayment() -> Element {
    let mut principal_interest = use_signal(|| String::new());
    let mut taxes = use_signal(|| String::new());
    let mut insurance = use_signal(|| String::new());
    let mut hoa = use_signal(|| String::new());
    let mut total_housing_payment = use_signal(|| 0.0);
    
    // Helper to parse currency input (removes commas)
    fn parse_currency(value: &str) -> f64 {
        let cleaned: String = value.chars().filter(|c| c.is_numeric() || *c == '.').collect();
        cleaned.parse::<f64>().unwrap_or(0.0)
    }
    
    // Helper to format input display with commas
    fn format_currency_input(value: &str) -> String {
        let cleaned: String = value.chars().filter(|c| c.is_numeric() || *c == '.').collect();
        if cleaned.is_empty() {
            return String::new();
        }
        
        // Don't format while typing - just return cleaned input
        // Only validate it's a valid number format
        if cleaned.contains('.') {
            let parts: Vec<&str> = cleaned.split('.').collect();
            if parts.len() == 2 {
                let integer_part = parts[0];
                let decimal_part = parts[1];
                
                // Add commas to integer part
                let mut result = String::new();
                for (i, ch) in integer_part.chars().rev().enumerate() {
                    if i > 0 && i % 3 == 0 {
                        result.push(',');
                    }
                    result.push(ch);
                }
                let formatted_int = result.chars().rev().collect::<String>();
                
                // Limit decimal to 2 places
                let limited_decimal = if decimal_part.len() > 2 {
                    &decimal_part[..2]
                } else {
                    decimal_part
                };
                
                format!("{}.{}", formatted_int, limited_decimal)
            } else {
                cleaned
            }
        } else {
            // Add commas to integer part only
            let mut result = String::new();
            for (i, ch) in cleaned.chars().rev().enumerate() {
                if i > 0 && i % 3 == 0 {
                    result.push(',');
                }
                result.push(ch);
            }
            result.chars().rev().collect::<String>()
        }
    }
    
    // Calculate total whenever any value changes
    use_effect(move || {
        let pi = parse_currency(&principal_interest.read());
        let tx = parse_currency(&taxes.read());
        let ins = parse_currency(&insurance.read());
        let h = parse_currency(&hoa.read());
        let total = pi + tx + ins + h;
        total_housing_payment.set(total);
    });
    
    // Update global total_housing context
    let mut total_housing_ctx = use_context::<Signal<f64>>();
    use_effect(move || {
        total_housing_ctx.set(total_housing_payment());
    });

    // Listen for global reset signal and clear housing payment fields
    let reset = use_context::<Signal<usize>>();
    use_effect(move || {
        if reset() > 0 {
            principal_interest.set(String::new());
            taxes.set(String::new());
            insurance.set(String::new());
            hoa.set(String::new());
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
                        r#type: "text",
                        id: "principal_interest",
                        name: "principal_interest",
                        placeholder: "0.00",
                        inputmode: "decimal",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
                        value: "{principal_interest}",
                        oninput: move |evt| {
                            let formatted = format_currency_input(&evt.value());
                            principal_interest.set(formatted);
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
                        r#type: "text",
                        id: "taxes",
                        name: "taxes",
                        placeholder: "0.00",
                        inputmode: "decimal",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
                        value: "{taxes}",
                        oninput: move |evt| {
                            let formatted = format_currency_input(&evt.value());
                            taxes.set(formatted);
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
                        r#type: "text",
                        id: "insurance",
                        name: "insurance",
                        placeholder: "0.00",
                        inputmode: "decimal",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
                        value: "{insurance}",
                        oninput: move |evt| {
                            let formatted = format_currency_input(&evt.value());
                            insurance.set(formatted);
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
                        r#type: "text",
                        id: "hoa",
                        name: "hoa",
                        placeholder: "0.00",
                        inputmode: "decimal",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
                        value: "{hoa}",
                        oninput: move |evt| {
                            let formatted = format_currency_input(&evt.value());
                            hoa.set(formatted);
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