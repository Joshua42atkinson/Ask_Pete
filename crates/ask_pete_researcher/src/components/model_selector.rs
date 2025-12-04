use crate::api::{DownloadProgress, ModelInfo};
use crate::components::boilermaker::{ChamferedPanel, MechanicalButton, PressureGauge};
use gloo_timers::future::TimeoutFuture;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn ModelSelector() -> impl IntoView {
    let (models, set_models) = signal(Vec::<ModelInfo>::new());
    let (progress, set_progress) = signal(DownloadProgress::default());
    let (is_polling, set_is_polling) = signal(false);
    let (error_msg, set_error_msg) = signal(None::<String>);

    // Fetch models on mount
    Effect::new(move |_| {
        spawn_local(async move {
            match crate::api::get_models().await {
                Ok(list) => set_models.set(list),
                Err(e) => set_error_msg.set(Some(e)),
            }
        });
    });

    // Polling logic for progress
    Effect::new(move |_| {
        if is_polling.get() {
            spawn_local(async move {
                loop {
                    if !is_polling.get_untracked() {
                        break;
                    }

                    match crate::api::get_download_progress().await {
                        Ok(p) => {
                            let status = p.status.clone();
                            set_progress.set(p);

                            if status == "completed" || status == "error" || status == "idle" {
                                set_is_polling.set(false);
                                // Refresh model list to show checkmark
                                if let Ok(list) = crate::api::get_models().await {
                                    set_models.set(list);
                                }
                                break;
                            }
                        }
                        Err(_) => {
                            // If polling fails, stop to prevent infinite error loops
                            set_is_polling.set(false);
                            break;
                        }
                    }
                    TimeoutFuture::new(500).await;
                }
            });
        }
    });

    let handle_download = StoredValue::new(move |id: String| {
        set_error_msg.set(None);
        spawn_local(async move {
            match crate::api::download_model(id).await {
                Ok(_) => set_is_polling.set(true),
                Err(e) => set_error_msg.set(Some(e)),
            }
        });
    });

    view! {
        <div class="space-y-6">
            // The Foundry Header
            <div class="flex items-center gap-4 mb-8">
                <div class="w-3 h-12 bg-old-gold chamfered-corners"></div>
                <div>
                    <h2 class="text-3xl font-black text-steam-white uppercase tracking-wider">"Select Your Engine"</h2>
                   <p class="text-purdue-dust text-sm font-mono uppercase tracking-widest">"The Foundry // Neural Pathway Forging"</p>
                </div>
            </div>

            <Show when=move || error_msg.get().is_some()>
                <ChamferedPanel class="border-signal-red">
                    <div class="flex items-center gap-3 text-signal-red">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
                        </svg>
                        <span class="font-bold">{move || error_msg.get()}</span>
                    </div>
                </ChamferedPanel>
            </Show>

            <ChamferedPanel class="gold-glow">
                <div class="space-y-6">
                    // Pressure Gauge Progress Section
                    <Show when=move || progress.get().status == "downloading">
                        <div class="p-6 bg-purdue-dark/50 chamfered-corners border border-old-gold/50 space-y-4">
                            <div class="flex items-center gap-3 text-old-gold mb-4">
                                <svg class="w-6 h-6 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                </svg>
                                <span class="text-lg font-bold font-mono uppercase tracking-wider">"Forging Neural Pathways..."</span>
                            </div>

                            <PressureGauge
                                value=Signal::derive(move || progress.get().percentage as f64)
                                label="Foundry Progress".to_string()
                            />

                            <div class="text-xs text-purdue-dust text-right font-mono mt-2">
                                {move || progress.get().speed.unwrap_or_else(|| "Calculating throughput...".to_string())}
                            </div>
                        </div>
                    </Show>

                    // Engine Selection Grid
                    <div class="grid gap-6">
                        <For
                            each=move || models.get()
                            key=|m| m.id.clone()
                            children=move |model| {
                                let m_id = model.id.clone();
                                let model_downloaded = model.downloaded;

                                // Map model names to industrial variants
                                let (engine_name, engine_desc) = if model.name.contains("small") || model.name.contains("1b") {
                                    ("Speedy Pete", "Lightweight. Fast.  Efficient.")
                                } else {
                                    ("Deep Thinker Pete", "Heavy Duty. Nuanced. Powerful.")
                                };

                                view! {
                                    <div class="group p-6 bg-purdue-dark/30 chamfered-corners border border-old-gold/30 hover:border-old-gold transition-all duration-300 gold-glow-hover">
                                        <div class="flex items-start justify-between gap-4">
                                            <div class="flex-1">
                                                <div class="flex items-center gap-3 mb-2">
                                                    <div class="w-10 h-10 bg-old-gold/20 border border-old-gold chamfered-corners flex items-center justify-center">
                                                        <svg class="w-6 h-6 text-old-gold" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"></path>
                                                        </svg>
                                                    </div>
                                                    <div>
                                                        <h3 class="text-xl font-black text-steam-white uppercase tracking-wide">{engine_name}</h3>
                                                        <p class="text-xs text-old-gold font-mono uppercase tracking-widest">{engine_desc}</p>
                                                    </div>
                                                </div>
                                                <div class="mt-3 space-y-1">
                                                    <div class="text-sm text-purdue-dust font-mono">
                                                        <span class="text-steam-white">"Capacity: "</span>
                                                        {model.size.clone()}
                                                    </div>
                                                    <div class="text-xs text-purdue-dust/70 font-mono">
                                                        <span class="text-steam-white/70">"ID: "</span>
                                                        {model.id.clone()}
                                                    </div>
                                                </div>
                                            </div>

                                            <Show
                                                when=move || model_downloaded
                                                fallback=move || {
                                                    let m_id = m_id.clone();
                                                    view! {
                                                        <MechanicalButton
                                                            primary=true
                                                            on_click=Callback::new(move |_: MouseEvent| handle_download.get_value()(m_id.clone()))
                                                            class="whitespace-nowrap"
                                                        >
                                                            "Forge"
                                                        </MechanicalButton>
                                                    }
                                                }
                                            >
                                                <div class="flex items-center text-gauge-green gap-2 px-4 py-2 bg-gauge-green/10 chamfered-corners border border-gauge-green">
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                    </svg>
                                                    <span class="text-sm font-bold font-mono uppercase">"Operational"</span>
                                                </div>
                                            </Show>
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
            </ChamferedPanel>
        </div>
    }
}
