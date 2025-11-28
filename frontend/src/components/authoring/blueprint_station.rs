use common::expert::StoryGraph;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlueprintRequest {
    pub subject: String,
    pub focus: f32,
    pub literary_device: String,
    pub vocabulary: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlueprintResponse {
    pub graph: StoryGraph,
    pub reasoning: String,
}

#[component]
pub fn BlueprintStation(
    on_close: Callback<()>,
    on_generate: Callback<StoryGraph>,
) -> impl IntoView {
    let (subject, set_subject) = signal("".to_string());
    let (focus, set_focus) = signal(0.5); // 0.0 = Academic, 1.0 = Story
    let (device, set_device) = signal("Hero's Journey".to_string());
    let (vocab_text, set_vocab_text) = signal("".to_string());
    let (is_generating, set_is_generating) = signal(false);

    let devices = vec![
        "Hero's Journey",
        "Murder Mystery",
        "Sci-Fi Exploration",
        "Historical Drama",
        "Comedy of Errors",
        "Cyberpunk Heist",
    ];

    let generate_handler = move |_| {
        set_is_generating.set(true);
        let req = BlueprintRequest {
            subject: subject.get(),
            focus: focus.get(),
            literary_device: device.get(),
            vocabulary: vocab_text
                .get()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        };

        leptos::task::spawn_local(async move {
            let client = reqwest::Client::new();
            match client
                .post("/api/architect/blueprint")
                .json(&req)
                .send()
                .await
            {
                Ok(res) => {
                    if let Ok(blueprint) = res.json::<BlueprintResponse>().await {
                        on_generate.run(blueprint.graph);
                        on_close.run(());
                    } else {
                        gloo_console::error!("Failed to parse blueprint response");
                    }
                }
                Err(e) => {
                    gloo_console::error!(format!("Failed to generate blueprint: {}", e));
                }
            }
            set_is_generating.set(false);
        });
    };

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm">
            <div class="bg-slate-900 border-2 border-boilermaker-gold rounded-lg w-[600px] max-h-[90vh] overflow-auto shadow-2xl flex flex-col">
                // Header
                <div class="p-6 border-b border-white/10 flex justify-between items-center bg-slate-950">
                    <div>
                        <h2 class="text-2xl font-bold text-boilermaker-gold flex items-center gap-2">
                            <span>"📐"</span>
                            "The Blueprint Station"
                        </h2>
                        <p class="text-slate-400 text-sm">"AI Curriculum Architect"</p>
                    </div>
                    <button
                        class="text-slate-400 hover:text-white transition-colors"
                        on:click=move |_| on_close.run(())
                    >
                        "✕"
                    </button>
                </div>

                // Body
                <div class="p-6 space-y-6 flex-grow">
                    // 1. Subject
                    <div class="space-y-2">
                        <label class="block text-sm font-bold text-purdue-gold">"SUBJECT MATTER"</label>
                        <input
                            type="text"
                            class="w-full bg-black/40 border border-slate-600 rounded p-3 text-white focus:border-boilermaker-gold outline-none transition-colors"
                            placeholder="e.g. Introduction to Calculus, Business Ethics..."
                            prop:value=move || subject.get()
                            on:input=move |ev| set_subject.set(event_target_value(&ev))
                        />
                    </div>

                    // 2. Focus Slider
                    <div class="space-y-2">
                        <div class="flex justify-between text-sm">
                            <span class="text-cyan-400">"Pure Academic"</span>
                            <span class="text-slate-400">"Focus Balance"</span>
                            <span class="text-pink-400">"Pure Narrative"</span>
                        </div>
                        <input
                            type="range"
                            min="0"
                            max="1"
                            step="0.1"
                            class="w-full h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-boilermaker-gold"
                            prop:value=move || focus.get()
                            on:input=move |ev| set_focus.set(event_target_value(&ev).parse().unwrap_or(0.5))
                        />
                        <div class="text-center text-xs text-boilermaker-gold font-mono">
                            {move || format!("{:.0}% Story", focus.get() * 100.0)}
                        </div>
                    </div>

                    // 3. Literary Device
                    <div class="space-y-2">
                        <label class="block text-sm font-bold text-purdue-gold">"LITERARY DEVICE"</label>
                        <select
                            class="w-full bg-black/40 border border-slate-600 rounded p-3 text-white focus:border-boilermaker-gold outline-none"
                            on:change=move |ev| set_device.set(event_target_value(&ev))
                            prop:value=move || device.get()
                        >
                            {devices.into_iter().map(|d| {
                                view! { <option value=d>{d}</option> }
                            }).collect::<Vec<_>>()}
                        </select>
                    </div>

                    // 4. Vocabulary
                    <div class="space-y-2">
                        <label class="block text-sm font-bold text-purdue-gold">"VOCABULARY (Comma Separated)"</label>
                        <textarea
                            class="w-full h-24 bg-black/40 border border-slate-600 rounded p-3 text-white focus:border-boilermaker-gold outline-none resize-none"
                            placeholder="derivative, integral, limit, continuity..."
                            prop:value=move || vocab_text.get()
                            on:input=move |ev| set_vocab_text.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                // Footer
                <div class="p-6 border-t border-white/10 bg-slate-950 flex justify-end gap-3">
                    <button
                        class="px-4 py-2 text-slate-300 hover:text-white transition-colors"
                        on:click=move |_| on_close.run(())
                    >
                        "Cancel"
                    </button>
                    <button
                        class="px-6 py-2 bg-boilermaker-gold hover:bg-white text-black font-bold rounded shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                        disabled=move || subject.get().is_empty() || is_generating.get()
                        on:click=generate_handler
                    >
                        {move || if is_generating.get() {
                            view! { <span class="animate-spin">"⚙️"</span> "Architecting..." }.into_any()
                        } else {
                            view! { <span>"✨"</span> "Generate Blueprint" }.into_any()
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}
