use leptos::prelude::*;
use pete_core::trainyard::StoryNode;

#[component]
pub fn OwlDiagnostic(
    #[prop(into)] nodes: Signal<Vec<StoryNode>>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    // Analysis Logic
    let analysis = move || {
        let current_nodes = nodes.get();
        let mut issues = Vec::new();
        let mut total_torque = 0.0;
        let mut total_friction = 0.0;

        for node in &current_nodes {
            // 1. Torque Check (Content Density)
            let word_count = node.content.split_whitespace().count();
            if word_count < 5 {
                issues.push((
                    node.id.clone(),
                    node.title.clone(),
                    "Low Torque (Content too sparse)",
                    "bg-yellow-900/50 text-yellow-200",
                ));
            } else if word_count > 100 {
                issues.push((
                    node.id.clone(),
                    node.title.clone(),
                    "Scale Buildup (Too much text)",
                    "bg-red-900/50 text-red-200",
                ));
                total_friction += 1.0;
            } else {
                total_torque += 1.0;
            }

            // 2. Friction Check (Complexity)
            if node.passenger_count > 4 {
                issues.push((
                    node.id.clone(),
                    node.title.clone(),
                    "High Friction (Boiler Pressure Critical)",
                    "bg-red-900/50 text-red-200",
                ));
                total_friction += 2.0;
            }

            // 3. Signal-to-Noise (Empty Fields)
            if node.title.trim().is_empty() || node.title == "New Station" {
                issues.push((
                    node.id.clone(),
                    "Untitled".to_string(),
                    "Weak Signal (Generic Title)",
                    "bg-blue-900/50 text-blue-200",
                ));
            }
        }

        let system_integrity = if current_nodes.is_empty() {
            100.0
        } else {
            let raw_score: f64 = total_torque * 10.0 - total_friction * 5.0;
            let score = raw_score.clamp(0.0, 100.0);
            score
        };

        (issues, system_integrity)
    };

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm" on:click=move |_| on_close.run(())>
            <div class="bg-slate-900 border-2 border-boilermaker-gold rounded-lg shadow-2xl w-[600px] max-h-[80vh] flex flex-col overflow-hidden" on:click=move |ev| ev.stop_propagation()>
                // Header
                <div class="bg-slate-950 p-4 border-b border-slate-700 flex justify-between items-center">
                    <div class="flex items-center gap-3">
                        <div class="text-3xl">"🦉"</div>
                        <div>
                            <h2 class="text-xl font-bold text-boilermaker-gold font-mono">"O.W.L. Diagnostic"</h2>
                            <p class="text-xs text-slate-400 uppercase tracking-widest">"Sector HSI // Optimization Matrix"</p>
                        </div>
                    </div>
                    <button
                        class="text-slate-400 hover:text-white transition-colors"
                        on:click=move |_| on_close.run(())
                    >
                        "✖"
                    </button>
                </div>

                // Dashboard
                <div class="p-6 overflow-y-auto custom-scrollbar">
                    // System Integrity Gauge
                    <div class="mb-8">
                        <div class="flex justify-between text-sm mb-2 text-slate-300 font-mono">
                            <span>"System Integrity"</span>
                            <span>{move || format!("{:.0}%", analysis().1)}</span>
                        </div>
                        <div class="h-4 bg-slate-800 rounded-full overflow-hidden border border-slate-700">
                            <div
                                class="h-full transition-all duration-1000 ease-out bg-gradient-to-r from-red-500 via-yellow-500 to-green-500"
                                style=move || format!("width: {:.0}%", analysis().1)
                            ></div>
                        </div>
                    </div>

                    // Issues List
                    <div class="space-y-3">
                        <h3 class="text-sm font-bold text-slate-400 uppercase tracking-wider mb-2">"Diagnostic Log"</h3>
                        {move || {
                            let (issues, _) = analysis();
                            if issues.is_empty() {
                                view! {
                                    <div class="p-4 bg-green-900/20 border border-green-500/30 rounded text-green-300 flex items-center gap-3">
                                        <span class="text-xl">"✅"</span>
                                        <div>
                                            <div class="font-bold">"All Systems Nominal"</div>
                                            <div class="text-sm opacity-80">"No friction detected. Boiler pressure stable."</div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <For
                                        each=move || issues.clone()
                                        key=|item| format!("{}{}", item.0, item.2)
                                        children=move |item| {
                                            view! {
                                                <div class=move || format!("p-3 rounded border border-white/10 flex justify-between items-center {}", item.3)>
                                                    <div class="flex flex-col">
                                                        <span class="font-bold text-sm font-mono">{item.1}</span>
                                                        <span class="text-xs opacity-80">{item.2}</span>
                                                    </div>
                                                    <button class="px-2 py-1 bg-black/20 hover:bg-black/40 rounded text-xs uppercase tracking-wider transition-colors">
                                                        "Locate"
                                                    </button>
                                                </div>
                                            }
                                        }
                                    />
                                }.into_any()
                            }
                        }}
                    </div>
                </div>

                // Footer
                <div class="bg-slate-950 p-3 border-t border-slate-700 text-center">
                    <p class="text-[10px] text-slate-500 font-mono">
                        "\"You can build the fastest engine in the world, but if the Operator's Manual is written in gibberish, the train goes nowhere.\""
                    </p>
                </div>
            </div>
        </div>
    }
}
