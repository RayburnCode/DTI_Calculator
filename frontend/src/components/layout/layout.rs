use dioxus::prelude::*;
use crate::Route;
use crate::components::layout::{Navbar};

#[component]
pub fn AppLayout() -> Element {
    let reset_signal = use_signal(|| 0usize);
    use_context_provider(|| reset_signal);

    // Toast signal for brief notifications (optional auto-clear on web)
    let toast = use_signal(|| None::<String>);
    use_context_provider(|| toast);

    rsx! {
        div { class: "min-h-screen bg-gray-900 text-gray-100 flex flex-col",
            // Header
            Navbar {}
            // Main Content Area
            div { class: "pt-20 px-4 sm:px-6 lg:px-12 py-8",
                div { class: "", Outlet::<Route> {} }
            }
            // Toast overlay (bottom-right)
            if let Some(msg) = toast.read().as_ref() {
                div { class: "fixed bottom-6 right-6 z-50",
                    div { class: "bg-black/80 text-white rounded-md px-4 py-2 shadow-lg",
                        "{msg}"
                    }
                }
            }
        }
    }
} 
