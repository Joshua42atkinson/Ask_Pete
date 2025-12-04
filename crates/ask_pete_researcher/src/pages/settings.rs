use crate::components::model_selector::ModelSelector;
use leptos::prelude::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto p-6 space-y-8 animate-fade-in">
             <div class="text-center space-y-2">
                <h1 class="text-4xl font-black text-white tracking-tight">
                    "System " <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-blue-600">"Settings"</span>
                </h1>
                <p class="text-slate-400">"Manage AI models and system diagnostics."</p>
            </div>

            <ModelSelector />

            // Placeholder for future diagnostics
            <div class="p-4 rounded-lg bg-white/5 border border-white/10 text-center text-slate-500 italic">
                "Additional diagnostics coming in Phase 5..."
            </div>
        </div>
    }
}
