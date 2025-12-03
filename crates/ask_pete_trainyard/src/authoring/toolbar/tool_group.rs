use leptos::prelude::*;

#[component]
pub fn ToolGroup(
    /// Group title (e.g., "Edit", "View")
    #[prop(into)]
    title: String,
    /// Whether to show a divider after this group
    #[prop(default = true)]
    show_divider: bool,
    /// Children components (buttons)
    children: Children,
) -> impl IntoView {
    let _title = RwSignal::new(title.to_string());
    view! {
        <div class="flex items-center gap-1">
            {children()}
            {move || if show_divider {
                view! {
                    <div class="w-px h-6 bg-slate-700 mx-1"></div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

#[component]
pub fn ToolButton(
    /// Icon or emoji for the button
    #[prop(into)]
    icon: String,
    /// Button label
    #[prop(into)]
    label: String,
    /// Tooltip text
    #[prop(into, optional)]
    tooltip: Option<String>,
    /// Click handler
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,
    /// Button variant
    #[prop(default = "default")]
    variant: &'static str,
) -> impl IntoView {
    let tooltip_text = tooltip.unwrap_or(label.clone());

    let button_class = match variant {
        "primary" => "px-3 py-1.5 bg-boilermaker-gold hover:bg-yellow-500 text-black font-semibold rounded shadow-sm transition-all",
        "success" => "px-3 py-1.5 bg-green-600 hover:bg-green-500 text-white font-semibold rounded shadow-sm transition-all",
        "danger" => "px-3 py-1.5 bg-red-600 hover:bg-red-500 text-white font-semibold rounded shadow-sm transition-all",
        _ => "px-3 py-1.5 bg-slate-700 hover:bg-slate-600 text-slate-200 rounded shadow-sm transition-all border border-slate-600",
    };

    view! {
        <button
            class=button_class
            on:click=move |_e| {
                if let Some(handler) = on_click {
                    handler.run(());
                }
            }
            title=tooltip_text
        >
            <span class="flex items-center gap-1.5">
                <span class="text-base">{icon}</span>
                <span class="text-sm">{label}</span>
            </span>
        </button>
    }
}
