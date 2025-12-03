//Description, Type, Monthly Amount


use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Debt {
    id: usize,
    description: String,
    debt_type: String,
    monthly_amount: f64,
}

#[component]
pub fn Debts() -> Element {
    let mut description = use_signal(|| String::new());
    let mut debt_type = use_signal(|| String::new());
    let mut monthly_amount = use_signal(|| String::new());
    let mut debts_list = use_signal(|| Vec::<Debt>::new());
    let mut next_id = use_signal(|| 0usize);
    let mut editing_id = use_signal(|| None::<usize>);
    
    // Calculate total debt
    let total_debt = use_memo(move || {
        debts_list.read().iter().map(|d| d.monthly_amount).sum::<f64>()
    });
    // Listen for global reset signal and clear local state when it increments
    let reset = use_context::<Signal<usize>>();
    use_effect(move || {
        if reset() > 0 {
            debts_list.set(Vec::new());
            next_id.set(0usize);
            editing_id.set(None);
            description.set(String::new());
            debt_type.set(String::new());
            monthly_amount.set(String::new());
        }
    });
    
    rsx! {
        div { class: "space-y-4",
            // Input grid
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                div { class: "flex flex-col",
                    label {
                        r#for: "debt_description",
                        class: "text-sm font-medium text-gray-700 mb-1",
                        "Description"
                    }
                    input {
                        r#type: "text",
                        id: "debt_description",
                        name: "debt_description",
                        placeholder: "e.g., Car Loan, Credit Card",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
                        value: "{description}",
                        oninput: move |evt| {
                            description.set(evt.value());
                        },
                    }
                }
                div { class: "flex flex-col",
                    label {
                        r#for: "debt_type",
                        class: "text-sm font-medium text-gray-700 mb-1",
                        "Type"
                    }
                    input {
                        r#type: "text",
                        id: "debt_type",
                        name: "debt_type",
                        placeholder: "e.g., Auto, Student",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
                        value: "{debt_type}",
                        oninput: move |evt| {
                            debt_type.set(evt.value());
                        },
                    }
                }
                div { class: "flex flex-col",
                    label {
                        r#for: "debt_monthly_amount",
                        class: "text-sm font-medium text-gray-700 mb-1",
                        "Monthly Payment"
                    }
                    input {
                        r#type: "number",
                        id: "debt_monthly_amount",
                        name: "debt_monthly_amount",
                        placeholder: "0.00",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
                        value: "{monthly_amount}",
                        oninput: move |evt| {
                            monthly_amount.set(evt.value());
                        },
                    }
                }
            }
            // Add/Update button
            button {
                class: "px-4 py-2 bg-red-600 hover:bg-red-700 text-white font-medium rounded-lg transition duration-200 shadow-sm hover:shadow-md",
                onclick: move |_| {
                    let desc = description.read().clone();
                    let dtype = debt_type.read().clone();
                    let amount_str = monthly_amount.read().clone();
                    if !desc.is_empty() && !dtype.is_empty() && !amount_str.is_empty() {
                        if let Ok(amount) = amount_str.parse::<f64>() {
                            if let Some(edit_id) = editing_id() {
                                debts_list
                                    .write()
                                    .iter_mut()
                                    .for_each(|debt| {
                                        if debt.id == edit_id {
                                            debt.description = desc.clone();
                                            debt.debt_type = dtype.clone();
                                            debt.monthly_amount = amount;
                                        }
                                    });
                                editing_id.set(None);
                            } else {
                                let new_debt = Debt {
                                    id: next_id(),
                                    description: desc,
                                    debt_type: dtype,
                                    monthly_amount: amount,
                                };
                                debts_list.write().push(new_debt);
                                next_id.set(next_id() + 1);
                            }
                            description.set(String::new());
                            debt_type.set(String::new());
                            monthly_amount.set(String::new());
                        }
                    }
                },
                if editing_id().is_some() {
                    "Update Debt"
                } else {
                    "+ Add Debt"
                }
            }
            // Debts list
            if !debts_list.read().is_empty() {
                div { class: "mt-6 space-y-2",
                    // Table header
                    div { class: "grid grid-cols-12 gap-2 px-3 py-2 bg-gray-100 rounded-lg font-semibold text-sm text-gray-700",
                        div { class: "col-span-4", "Description" }
                        div { class: "col-span-3", "Type" }
                        div { class: "col-span-3 text-right", "Monthly Payment" }
                        div { class: "col-span-2 text-center", "Actions" }
                    }
                    // Debt rows
                    {
                        debts_list
                            .read()
                            .iter()
                            .map(|debt| {
                                let debt_id = debt.id;
                                let debt_desc = debt.description.clone();
                                let debt_type_val = debt.debt_type.clone();
                                let debt_amount = debt.monthly_amount;
                                rsx! {
                                    div {
                                        key: "{debt_id}",
                                        class: "grid grid-cols-12 gap-2 px-3 py-3 bg-gray-50 rounded-lg items-center hover:bg-gray-100 transition",
                                        div { class: "col-span-4 text-gray-800", "{debt_desc}" }
                                        div { class: "col-span-3 text-gray-800", "{debt_type_val}" }
                                        div { class: "col-span-3 text-right font-semibold text-gray-800", "${debt_amount:.2}" }
                                        div { class: "col-span-2 flex gap-2 justify-center",
                                            // Edit button
                                            button {
                                                class: "px-3 py-1 bg-blue-500 hover:bg-blue-600 text-white text-xs font-medium rounded transition",
                                                onclick: move |_| {
                                                    if let Some(d) = debts_list.read().iter().find(|d| d.id == debt_id) {
                                                        description.set(d.description.clone());
                                                        debt_type.set(d.debt_type.clone());
                                                        monthly_amount.set(d.monthly_amount.to_string());
                                                        editing_id.set(Some(debt_id));
                                                    }
                                                },
                                                "Edit"
                                            }
                                            // Delete button
                                            button {
                                                class: "px-3 py-1 bg-red-500 hover:bg-red-600 text-white text-xs font-medium rounded transition",
                                                onclick: move |_| {
                                                    debts_list.write().retain(|d| d.id != debt_id);
                                                    if editing_id() == Some(debt_id) {
                                                        editing_id.set(None);
                                                        description.set(String::new());
                                                        debt_type.set(String::new());
                                                        monthly_amount.set(String::new());
                                                    }
                                                },
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            })
                    }
                }
            }
            // Total display
            div { class: "mt-6 pt-4 border-t border-gray-200",
                div { class: "flex justify-between items-center",
                    span { class: "text-lg font-semibold text-gray-700", "Total Monthly Debt:" }
                    span { class: "text-2xl font-bold text-red-600", "${total_debt:.2}" }
                }
            }
        }
    }
}