use crate::components::tooltip::Tooltip;
use common::expert::StoryGraph;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrainYardRequest {
    pub subject: String,
    pub focus: f32,
    pub literary_device: String,
    pub vocabulary: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrainYardResponse {
    pub graph: StoryGraph,
    pub reasoning: String,
}

#[component]
pub fn TrainYard(on_close: Callback<()>, on_generate: Callback<StoryGraph>) -> impl IntoView {
    let (subject, set_subject) = signal("".to_string());
    let (focus, set_focus) = signal(0.5); // 0.0 = Academic, 1.0 = Story
    let (device, set_device) = signal("Hero's Journey".to_string());
    let (vocab_text, set_vocab_text) = signal("".to_string());
    let (vocab_mode, set_vocab_mode) = signal(false); // false = Auto, true = Manual
    let (is_generating, set_is_generating) = signal(false);

    let generate_handler = move |_| {
        set_is_generating.set(true);
        let req = TrainYardRequest {
            subject: subject.get(),
            focus: focus.get(),
            literary_device: device.get(),
            vocabulary: if vocab_mode.get() {
                vocab_text
                    .get()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                vec![] // Empty triggers auto-detect in backend
            },
        };

        leptos::task::spawn_local(async move {
            let client = reqwest::Client::new();
            // TODO: Update API endpoint if backend changes, keeping /api/architect/blueprint for now or updating to /api/train_yard/dispatch
            match client
                .post("/api/architect/blueprint")
                .json(&req)
                .send()
                .await
            {
                Ok(res) => {
                    if let Ok(response) = res.json::<TrainYardResponse>().await {
                        on_generate.run(response.graph);
                        on_close.run(());
                    } else {
                        gloo_console::error!("Failed to parse train yard response");
                    }
                }
                Err(e) => {
                    gloo_console::error!(format!("Failed to generate from train yard: {}", e));
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
                            <span>"🚉"</span>
                            "The Train Yard"
                        </h2>
                        <p class="text-slate-400 text-sm">"Curriculum Dispatcher & Logistics"</p>
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
                        <Tooltip text="The cargo manifest or academic topic.".to_string()>
                            <label class="block text-sm font-bold text-purdue-gold cursor-help decoration-dotted underline underline-offset-4">"CARGO MANIFEST (SUBJECT)"</label>
                        </Tooltip>
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
                            <span class="text-cyan-400">"Pure Cargo (Facts)"</span>
                            <Tooltip text="Balancing the payload (facts) with the journey (story).".to_string()>
                                <span class="text-slate-400 cursor-help decoration-dotted underline underline-offset-4">"Load Balance"</span>
                            </Tooltip>
                            <span class="text-pink-400">"Scenic Route (Story)"</span>
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
                            {move || format!("{:.0}% Scenic", focus.get() * 100.0)}
                        </div>
                    </div>

                    // 3. Literary Device
                    <div class="space-y-2">
                        <Tooltip text="The route or track layout for the journey.".to_string()>
                            <label class="block text-sm font-bold text-purdue-gold cursor-help decoration-dotted underline underline-offset-4">"TRACK LAYOUT (THEME)"</label>
                        </Tooltip>
                        <select
                            class="w-full bg-black/40 border border-slate-600 rounded p-3 text-white focus:border-boilermaker-gold outline-none"
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_device.set(val);
                            }
                            prop:value=move || device.get()
                        >
                            {common::models::literary_device::LiteraryDevice::all().into_iter().map(|d| {
                                let label = d.to_string();
                                view! { <option value=label.clone()>{label.clone()}</option> }
                            }).collect::<Vec<_>>()}
                        </select>
                    </div>

                    // 4. Vocabulary Mode
                    <div class="space-y-2">
                        <div class="flex justify-between items-center">
                            <label class="block text-sm font-bold text-purdue-gold">"WAYPOINTS (VOCABULARY)"</label>
                            <button
                                class="text-xs text-cyan-400 hover:text-white underline"
                                on:click=move |_| set_vocab_mode.update(|m| *m = !*m)
                            >
                                {move || if vocab_mode.get() { "Switch to Auto-Route" } else { "Manual Waypoints" }}
                            </button>
                        </div>

                        {move || if vocab_mode.get() {
                            view! {
                                <textarea
                                    class="w-full h-24 bg-black/40 border border-slate-600 rounded p-3 text-white focus:border-boilermaker-gold outline-none resize-none"
                                    placeholder="derivative, integral, limit, continuity..."
                                    prop:value=move || vocab_text.get()
                                    on:input=move |ev| set_vocab_text.set(event_target_value(&ev))
                                />
                            }.into_any()
                        } else {
                            view! {
                                <div class="w-full h-24 bg-slate-800/50 border border-slate-700 border-dashed rounded p-4 flex items-center justify-center text-slate-400 text-sm italic">
                                    "Waypoints will be automatically identified based on the Cargo Manifest."
                                </div>
                            }.into_any()
                        }}
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
                        prop:disabled=move || subject.get().is_empty() || is_generating.get()
                        on:click=generate_handler
                    >
                        {move || if is_generating.get() {
                            view! { <span class="animate-spin">"⚙️"</span> "Dispatching..." }.into_any()
                        } else {
                            view! { <span>"🚂"</span> "Dispatch Train" }.into_any()
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}
