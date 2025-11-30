use crate::api::{create_character, CreateCharacterRequest, MemberRole};
use leptos::prelude::*;

#[component]
pub fn CharacterCreationPage() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let (selected_role, set_selected_role) = signal(MemberRole::Engineer);
    let (submitted, set_submitted) = signal(false);

    let roles = vec![
        (
            MemberRole::Engineer,
            "The Engineer",
            "bg-signal-red",
            "DPS / Warrior",
            "Pushes the Iron Horse forward. High Coal output.",
        ),
        (
            MemberRole::Conductor,
            "The Conductor",
            "bg-purdue-gold",
            "Tank / Leader",
            "Manages the track and schedule. High Defense.",
        ),
        (
            MemberRole::Stoker,
            "The Stoker",
            "bg-gauge-green",
            "Healer / Support",
            "Keeps the fire burning. High Steam output.",
        ),
        (
            MemberRole::Signalman,
            "The Signalman",
            "bg-purdue-prime",
            "Mage / Control",
            "Controls the flow and switches. Manages Complexity.",
        ),
    ];

    let on_submit = move |_| {
        let req = CreateCharacterRequest {
            name: name.get(),
            role: selected_role.get(),
            archetype: format!("{:?}", selected_role.get()), // Simple mapping for now
        };

        leptos::task::spawn_local(async move {
            if let Ok(_) = create_character(req).await {
                set_submitted.set(true);
            }
        });
    };

    view! {
        <div class="min-h-screen bg-void-black text-steam-white font-sans p-8 flex flex-col items-center">
            <header class="mb-12 text-center">
                <h1 class="text-4xl font-bold text-metallic-gold mb-2">"RAILWAY JOB APPLICATION"</h1>
                <p class="text-purdue-gold/70">"Sign the contract. Board the train. Survive the journey."</p>
            </header>

            {move || if submitted.get() {
                view! {
                    <div class="text-center p-12 border border-gauge-green bg-gauge-green/10 rounded-lg">
                        <h2 class="text-2xl font-bold text-gauge-green mb-4">"APPLICATION ACCEPTED"</h2>
                        <p>"Report to the platform immediately. The train is leaving."</p>
                        <a href="/yard/dashboard" class="inline-block mt-6 mechanical-button-primary">"PROCEED TO DASHBOARD"</a>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="w-full max-w-4xl">
                        // 1. IDENTITY
                        <section class="mb-12">
                            <h2 class="text-xl text-purdue-gold mb-4 border-b border-purdue-gold/30 pb-2">"I. APPLICANT IDENTITY"</h2>
                            <div class="flex flex-col gap-2">
                                <label class="text-sm text-white/50">"FULL NAME"</label>
                                <input
                                    type="text"
                                    class="bg-black/40 border border-old-gold p-3 rounded text-lg focus:border-purdue-gold outline-none text-steam-white"
                                    placeholder="Enter your name..."
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    prop:value=move || name.get()
                                />
                            </div>
                        </section>

                        // 2. POSITION
                        <section class="mb-12">
                            <h2 class="text-xl text-purdue-gold mb-4 border-b border-purdue-gold/30 pb-2">"II. DESIRED POSITION"</h2>
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                                {roles.iter().map(|(role, title, color, archetype, desc)| {
                                    let r = *role;
                                    let c = *color;
                                    view! {
                                        <div
                                            class=move || {
                                                let base = "cursor-pointer p-6 rounded border transition-all duration-200 hover:scale-105";
                                                if selected_role.get() == r {
                                                    format!("{} {} border-opacity-100 bg-white/10", base, match r {
                                                        MemberRole::Engineer => "border-signal-red",
                                                        MemberRole::Conductor => "border-purdue-gold",
                                                        MemberRole::Stoker => "border-gauge-green",
                                                        MemberRole::Signalman => "border-purdue-prime",
                                                    })
                                                } else {
                                                    format!("{} border-white/10 hover:border-white/30 bg-black/20", base)
                                                }
                                            }
                                            on:click=move |_| set_selected_role.set(r)
                                        >
                                            <div class={format!("w-12 h-12 rounded-full mb-4 flex items-center justify-center {}", c)}>
                                                // Icon placeholder
                                            </div>
                                            <h3 class="font-bold text-lg mb-1">{*title}</h3>
                                            <span class="text-xs text-purdue-gold uppercase tracking-wider block mb-2">{*archetype}</span>
                                            <p class="text-sm text-white/70 leading-relaxed">{*desc}</p>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </section>

                        // 3. CONTRACT
                        <div class="flex justify-center mt-12">
                            <button
                                class="mechanical-button-primary text-xl px-12 py-4"
                                disabled=move || name.get().is_empty()
                                on:click=on_submit
                            >
                                "SIGN CONTRACT"
                            </button>
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
