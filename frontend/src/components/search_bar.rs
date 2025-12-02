use dioxus::prelude::*;


#[component]
pub fn SearchBar() -> Element {
    rsx! {

        form { class: "max-w-md mx-auto",
            label {
                class: "block mb-2.5 text-sm font-medium text-heading sr-only",
                r#for: "search",
                "Search"
            }
            div { class: "relative",
                div { class: "absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none",
                    svg {
                        class: "w-4 h-4 text-body",
                        fill: "none",
                        height: "24",
                        view_box: "0 0 24 24",
                        width: "24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_width: "2",
                        }
                    }
                }
                input {
                    class: "block w-full p-3 ps-9 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body",
                    id: "search",
                    placeholder: "Search",
                    r#type: "search",
                    required: "false",
                }
                button {
                    class: "absolute end-1.5 bottom-1.5 text-white bg-brand hover:bg-brand-strong box-border border border-transparent focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded text-xs px-3 py-1.5 focus:outline-none",
                    r#type: "button",
                    "Search"
                }
            }
        }
    }
}
