use crate::theme::makeup::MakeupStyle;
use leptos::prelude::*;

/// A component for selecting the active "Makeup" (Livery) style.
/// Branded as "Personalize Pete" to fit the Purdue culture.
#[component]
pub fn PersonalizePete(
    #[prop(into)] current_style: Signal<MakeupStyle>,
    #[prop(into)] on_change: Callback<MakeupStyle>,
) -> impl IntoView {
    let styles = vec![
        MakeupStyle::StandardIndustrial,
        MakeupStyle::NeonNights,
        MakeupStyle::Blueprint,
        MakeupStyle::IvoryTower,
        MakeupStyle::RustBucket,
    ];

    view! {
        <div class="personalize-pete p-6 bg-black/80 backdrop-blur border-2 border-[#CEB888] chamfered-corners max-w-md">
            <div class="flex items-center justify-between mb-6 border-b border-[#CEB888]/30 pb-2">
                <h2 class="text-xl font-bold text-[#CEB888] tracking-widest uppercase">
                    "Personalize Pete"
                </h2>
                <div class="w-3 h-3 bg-[#CEB888] rounded-full animate-pulse"></div>
            </div>

            <p class="text-xs text-[#CEB888]/70 mb-4 font-mono">
                "SELECT INTERFACE OVERLAY PROTOCOL. CUSTOMIZE YOUR WORKSPACE."
            </p>

            <div class="space-y-3">
                {styles.into_iter().map(|style| {
                    let style_clone = style.clone();
                    let style_active = style.clone();
                    let is_active = move || current_style.get() == style_active;
                    let on_select = on_change.clone();

                    view! {
                        <button
                            class=move || {
                                let base = "w-full text-left px-4 py-3 border transition-all duration-200 flex items-center justify-between group";
                                if is_active() {
                                    format!("{} bg-[#CEB888]/20 border-[#CEB888] text-[#CEB888] shadow-[0_0_15px_rgba(206,184,136,0.3)]", base)
                                } else {
                                    format!("{} bg-transparent border-[#CEB888]/30 text-slate-400 hover:border-[#CEB888] hover:text-[#CEB888]", base)
                                }
                            }
                            on:click=move |_| on_select.run(style_clone.clone())
                        >
                            <span class="font-mono uppercase tracking-wider text-sm">
                                {style.to_string()}
                            </span>

                            // Status Indicator
                            <div class={
                                let is_active = is_active.clone();
                                move || {
                                    if is_active() {
                                        "w-2 h-2 bg-[#CEB888] shadow-[0_0_8px_#CEB888]"
                                    } else {
                                        "w-2 h-2 bg-slate-700 group-hover:bg-[#CEB888]/50"
                                    }
                                }
                            }></div>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="mt-6 pt-4 border-t border-[#CEB888]/30 flex justify-between items-center text-[10px] text-[#CEB888]/50 font-mono">
                <span>"SYS.VER.3.0.1"</span>
                <span>"BOILERMAKER IND."</span>
            </div>
        </div>
    }
}
