use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum ActivePanel {
    None,
    Yard,
    WeighStation,
    Comm,
    Party, // [NEW]
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct PhysicsState {
    pub mass: f32,
    pub power: f32,
    pub velocity: f32,
    pub miles: f32,
}

#[component]
pub fn DispatcherConsole(children: Children) -> impl IntoView {
    let (active_panel, set_active_panel) = signal(ActivePanel::Yard);
    let (physics_state, set_physics_state) = signal(PhysicsState::default());
    let (campaign_state, set_campaign_state) = signal(crate::api::CampaignState::default()); // [NEW]
    let (is_active, set_is_active) = signal(true);

    // Poll simulation state
    Effect::new(move |_| {
        spawn_local(async move {
            while is_active.get() {
                if let Ok(response) = gloo_net::http::Request::get("/api/simulation/state")
                    .send()
                    .await
                {
                    if let Ok(state) = response.json::<PhysicsState>().await {
                        set_physics_state.set(state);
                    }
                }

                // Poll Campaign State [NEW]
                if let Ok(state) = crate::api::fetch_campaign_state().await {
                    set_campaign_state.set(state);
                }

                gloo_timers::future::sleep(std::time::Duration::from_millis(100)).await;
            }
        });

        on_cleanup(move || {
            set_is_active.set(false);
        });
    });

    view! {
        <div class="dispatcher-grid">
            // 1. THE SIGNAL GANTRY (Top Header)
            <header class="grid-header">
                <div class="logo text-metallic-gold text-xl">"ASK PETE: STATION MASTER"</div>
                <div class="flex-grow"></div>
                <div class="status-lights flex gap-4">
                    <span class="text-gauge-green font-mono">"SYSTEM: ONLINE"</span>
                </div>
            </header>

            // 2. THE SWITCH TRACK (Left Sidebar)
            <nav class="grid-sidebar p-2 gap-4">
                <button
                    class=move || if active_panel.get() == ActivePanel::Yard { "mechanical-button-primary w-full text-xs" } else { "mechanical-button-secondary w-full text-xs" }
                    on:click=move |_| set_active_panel.set(ActivePanel::Yard)
                >
                    "YARD"
                </button>
                <button
                    class=move || if active_panel.get() == ActivePanel::WeighStation { "mechanical-button-primary w-full text-xs" } else { "mechanical-button-secondary w-full text-xs" }
                    on:click=move |_| set_active_panel.set(ActivePanel::WeighStation)
                >
                    "WEIGH"
                </button>
                <button
                    class=move || if active_panel.get() == ActivePanel::Comm { "mechanical-button-primary w-full text-xs" } else { "mechanical-button-secondary w-full text-xs" }
                    on:click=move |_| set_active_panel.set(ActivePanel::Comm)
                >
                    "COMM"
                </button>
                <button
                    class=move || if active_panel.get() == ActivePanel::Party { "mechanical-button-primary w-full text-xs" } else { "mechanical-button-secondary w-full text-xs" }
                    on:click=move |_| set_active_panel.set(ActivePanel::Party)
                >
                    "PARTY"
                </button>
            </nav>

            // 3. THE MAIN LINE (Main Content Area)
            <main class="grid-main relative">
                <div class="viewport-container h-full w-full overflow-auto">
                    {children()}
                </div>

                // Floating Glass Panels
                <div
                    class="glass-panel absolute top-4 left-4 bottom-4 w-96 transition-transform duration-300 transform z-10 p-6"
                    class:translate-x-0=move || active_panel.get() == ActivePanel::WeighStation
                    class:-translate-x-full=move || active_panel.get() != ActivePanel::WeighStation
                    class:pointer-events-none=move || active_panel.get() != ActivePanel::WeighStation
                >
                    <h2 class="text-metallic-gold text-2xl mb-4">"WEIGH STATION"</h2>
                    <p class="text-steam-white">"Vocabulary and Concept Weight Analysis"</p>
                    // Add Weigh Station content here
                </div>

                <div
                    class="glass-panel absolute top-4 left-4 bottom-4 w-96 transition-transform duration-300 transform z-10 p-6"
                    class:translate-x-0=move || active_panel.get() == ActivePanel::Comm
                    class:-translate-x-full=move || active_panel.get() != ActivePanel::Comm
                    class:pointer-events-none=move || active_panel.get() != ActivePanel::Comm
                >
                    <h2 class="text-metallic-gold text-2xl mb-4">"MENTOR COMM"</h2>
                    <p class="text-steam-white mb-4">"Communication Uplink with Pete"</p>
                    <crate::components::ai_mirror_chat::AiMirrorChat />
                </div>

                <div
                    class="glass-panel absolute top-4 left-4 bottom-4 w-96 transition-transform duration-300 transform z-10 p-6 overflow-auto"
                    class:translate-x-0=move || active_panel.get() == ActivePanel::Party
                    class:-translate-x-full=move || active_panel.get() != ActivePanel::Party
                    class:pointer-events-none=move || active_panel.get() != ActivePanel::Party
                >
                    <h2 class="text-metallic-gold text-2xl mb-4">"PARTY MANIFEST"</h2>

                    <div class="mb-6">
                        <h3 class="text-purdue-gold text-sm mb-2">"CAMPAIGN ID"</h3>
                        <div class="font-mono text-steam-white text-lg border border-old-gold/30 p-2 rounded bg-black/20 flex justify-between items-center">
                            <span>{move || campaign_state.get().campaign_id}</span>
                            <span class="text-xs text-purdue-gold/70 border border-purdue-gold/30 px-2 py-1 rounded">
                                {move || format!("{:?}", campaign_state.get().style)}
                            </span>
                        </div>
                    </div>

                    <div class="grid grid-cols-2 gap-4 mb-6">
                        <div class="bg-black/20 p-3 rounded border border-old-gold/30">
                            <span class="text-xs text-purdue-gold block mb-1">"GROUP COAL"</span>
                            <span class="text-2xl text-signal-red font-bold">{move || campaign_state.get().collective_coal}</span>
                        </div>
                        <div class="bg-black/20 p-3 rounded border border-old-gold/30">
                            <span class="text-xs text-purdue-gold block mb-1">"GROUP STEAM"</span>
                            <span class="text-2xl text-gauge-green font-bold">{move || campaign_state.get().collective_steam}</span>
                        </div>
                    </div>

                    <div class="mb-6">
                        <h3 class="text-purdue-gold text-sm mb-2">"ACTIVE VOTE"</h3>
                        {move || {
                            match campaign_state.get().active_vote {
                                Some(vote) => view! {
                                    <div class="bg-black/40 p-4 rounded border border-purdue-gold/50">
                                        <p class="text-steam-white mb-4 font-bold">{vote.question}</p>
                                        <div class="flex flex-col gap-2">
                                            {vote.options.into_iter().enumerate().map(|(idx, option)| {
                                                let campaign_id = campaign_state.get().campaign_id.clone();
                                                view! {
                                                    <button
                                                        class="mechanical-button-secondary text-xs py-2"
                                                        on:click=move |_| {
                                                            let c_id = campaign_id.clone();
                                                            spawn_local(async move {
                                                                let _ = crate::api::submit_vote(c_id, idx).await;
                                                            });
                                                        }
                                                    >
                                                        {option}
                                                    </button>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                        <div class="mt-4 text-xs text-purdue-gold/70">
                                            "Tally: " {format!("{:?}", vote.current_tally)}
                                        </div>
                                    </div>
                                }.into_any(),
                                None => view! {
                                    <div class="text-steam-white/50 italic text-sm">"No active votes pending."</div>
                                }.into_any()
                            }
                        }}
                    </div>

                    <div>
                        <h3 class="text-purdue-gold text-sm mb-2">"MEMBERS"</h3>
                        <ul class="space-y-2">
                            {move || campaign_state.get().party_members.values().map(|profile| {
                                let role_color = match profile.role {
                                    crate::api::MemberRole::Engineer => "bg-signal-red",
                                    crate::api::MemberRole::Conductor => "bg-purdue-gold",
                                    crate::api::MemberRole::Stoker => "bg-gauge-green",
                                    crate::api::MemberRole::Signalman => "bg-purdue-prime", // Using Prime (Purple-ish) for Signalman
                                };
                                view! {
                                    <li class="flex items-center gap-2 text-steam-white text-sm bg-black/20 p-2 rounded border border-white/5">
                                        <div class={format!("w-3 h-3 rounded-full {}", role_color)} title={format!("{:?}", profile.role)}></div>
                                        <div class="flex flex-col">
                                            <span class="font-bold">{profile.name.clone()}</span>
                                            <span class="text-xs text-white/50">{profile.archetype.clone()}</span>
                                        </div>
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                </div>
            </main>

            // 4. THE INSPECTION PIT (Right/Bottom Panel)
            <aside class="grid-properties text-steam-white p-4 overflow-hidden">
                <h3 class="text-metallic-gold text-lg mb-4 border-b border-old-gold pb-2">"TELEMETRY"</h3>
                <div class="grid grid-cols-2 gap-4 font-mono text-sm">
                    <div class="flex flex-col">
                        <span class="text-purdue-gold/50 text-xs">"VELOCITY"</span>
                        <span class="text-xl text-gauge-green">{move || format!("{:.2} m/s", physics_state.get().velocity)}</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="text-purdue-gold/50 text-xs">"DISTANCE"</span>
                        <span class="text-xl text-steam-white">{move || format!("{:.2} mi", physics_state.get().miles)}</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="text-purdue-gold/50 text-xs">"LOAD (MASS)"</span>
                        <span class="text-xl text-signal-red">{move || format!("{:.2} kg", physics_state.get().mass)}</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="text-purdue-gold/50 text-xs">"WILL (POWER)"</span>
                        <span class="text-xl text-purdue-prime">{move || format!("{:.2} kW", physics_state.get().power)}</span>
                    </div>
                </div>
            </aside>
        </div>
    }
}
