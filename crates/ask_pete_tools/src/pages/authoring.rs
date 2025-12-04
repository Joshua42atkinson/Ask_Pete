use domain_trainyard::authoring::node_canvas::NodeCanvas;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn AuthoringPage() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <div class="h-screen w-screen flex flex-col">
            // Header
            <header class="h-16 bg-slate-900 border-b border-white/10 flex items-center px-6 justify-between shrink-0">
                <div class="flex items-center gap-4">
                    // Back button to home
                    <button
                        class="px-3 py-1.5 bg-slate-800 hover:bg-slate-700 border border-white/10 text-white rounded transition-colors flex items-center gap-2 text-sm font-medium"
                        on:click=move |_| navigate("/", Default::default())
                        title="Return to home page"
                    >
                        <span class="text-lg">"←"</span>
                        <span>"Home"</span>
                    </button>
                    <h1 class="text-xl font-bold text-white tracking-widest">"ASK PETE" <span class="text-cyan-400">"AUTHOR"</span></h1>
                </div>
                <div class="flex items-center gap-4">
                    <span class="text-slate-500 text-sm">"v0.1.0-alpha"</span>
                </div>
            </header>

            // Main Content
            <main class="flex-grow relative">
                <NodeCanvas />
            </main>
        </div>
    }
}
