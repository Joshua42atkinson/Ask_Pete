use crate::components::ai_mirror_chat::AiMirrorChat;
use crate::components::personalize_pete::PersonalizePete;
use crate::ui_theme::{blueprint_grid_background, use_theme, MakeupLayer};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

/// The Student's immersive learning environment ("The Engine Cab")
/// Game-style interface with AI-powered interactions
#[component]
pub fn EngineCabLayout() -> impl IntoView {
    let params = use_params_map();
    let quest_id = move || params.read().get("quest_id").unwrap_or_default();
    let theme = use_theme();

    // State for overlays
    let (show_personalize, set_show_personalize) = signal(false);
    let (show_ai_mirror, set_show_ai_mirror) = signal(false);

    // Audio/Text State
    let (audio_enabled, set_audio_enabled) = signal(false);

    view! {
        // 1. MAKEUP LAYER (CSS Variables Injection)
        <MakeupLayer/>

        <div class="engine-cab-layout h-screen w-screen overflow-hidden relative text-[var(--text-primary)] font-ui">
            // 2. BASE LAYER (Dynamic Background)
            <div class="absolute inset-0 bg-[var(--bg-primary)] transition-colors duration-500 z-0">
                // Secondary background pattern/gradient
                <div class="absolute inset-0 bg-gradient-to-b from-[var(--bg-secondary)]/50 to-transparent"></div>

                // Grid overlay (optional based on theme, but good for industrial feel)
                <div class={blueprint_grid_background()}></div>
            </div>

            // 3. CONTENT LAYER (The HUD and Main View)
            <div class="relative z-10 h-full flex flex-col">
                // Top HUD
                <header class="h-16 border-b border-[var(--border-color)]/30 bg-[var(--bg-secondary)]/80 backdrop-blur px-6 flex items-center justify-between shrink-0">
                    <div class="flex items-center gap-4">
                        <div class="text-sm font-mono uppercase tracking-widest text-[var(--accent-primary)]">
                            "QUEST PROTOCOL: " {move || quest_id()}
                        </div>
                    </div>

                    // Center: Cognitive Load Gauge (Placeholder)
                    <div class="w-96 h-2 bg-black/50 rounded-full overflow-hidden border border-[var(--border-color)]/30">
                        <div class="h-full w-[35%] bg-[var(--accent-primary)] shadow-[0_0_10px_var(--glow-color)]"></div>
                    </div>

                    // Right: Utility Controls
                    <div class="flex items-center gap-3">
                        // Audio Toggle
                        <button
                            class="p-2 rounded hover:bg-[var(--accent-primary)]/10 text-[var(--accent-primary)] transition-colors"
                            on:click=move |_| set_audio_enabled.update(|v| *v = !*v)
                            title={move || if audio_enabled.get() { "Mute Audio" } else { "Enable Audio" }}
                        >
                            {move || if audio_enabled.get() { "🔊" } else { "🔇" }}
                        </button>

                        // Personalize Pete Toggle
                        <button
                            class="px-4 py-1.5 border border-[var(--border-color)] text-[var(--accent-primary)] text-xs font-mono uppercase tracking-wider hover:bg-[var(--accent-primary)]/10 transition-all"
                            on:click=move |_| set_show_personalize.update(|v| *v = !*v)
                        >
                            "Personalize"
                        </button>
                    </div>
                </header>

                // Main Workspace
                <main class="flex-grow p-6 overflow-auto relative">
                    <div class="max-w-5xl mx-auto h-full flex flex-col justify-center items-center">
                        <div class="p-12 border border-[var(--border-color)] bg-[var(--bg-secondary)]/50 backdrop-blur chamfered-corners max-w-2xl w-full text-center shadow-[0_0_30px_rgba(0,0,0,0.5)]">
                            <h1 class="text-4xl font-bold text-[#BC13FE] mb-6 tracking-tight">
                                "INTERFACE C ONLINE"
                            </h1>
                            <p class="text-lg opacity-80 mb-8 leading-relaxed">
                                "Welcome to the student experience. This interface adapts to your preferences while maintaining the Boilermaker standard."
                            </p>

                            <div class="flex justify-center gap-4">
                                <button
                                    class="px-8 py-3 bg-[var(--accent-primary)] text-[var(--bg-primary)] font-bold uppercase tracking-widest hover:brightness-110 transition-all shadow-[0_0_15px_var(--glow-color)]"
                                    on:click=move |_| set_show_ai_mirror.set(true)
                                >
                                    "Talk to Pete"
                                </button>
                            </div>
                        </div>
                    </div>
                </main>
            </div>

            // 4. OVERLAY LAYER (Modals & Floating Panels)

            // Personalize Pete Modal
            {move || show_personalize.get().then(|| view! {
                <div class="absolute inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center animate-fade-in">
                    <div class="relative">
                        <button
                            class="absolute -top-12 right-0 text-white/50 hover:text-white"
                            on:click=move |_| set_show_personalize.set(false)
                        >
                            "CLOSE [X]"
                        </button>
                        <PersonalizePete
                            current_style=theme.makeup
                            on_change=Callback::new(move |style| {
                                theme.set_makeup.set(style);
                            })
                        />
                    </div>
                </div>
            })}

            // AI Mirror (Pete) Floating Panel
            {move || show_ai_mirror.get().then(|| view! {
                <div class="absolute right-6 bottom-6 w-96 h-[600px] z-40 shadow-2xl animate-slide-up">
                    <div class="h-full flex flex-col bg-[var(--bg-secondary)] border border-[var(--border-color)]">
                        <div class="flex items-center justify-between p-3 border-b border-[var(--border-color)]/30 bg-black/20">
                            <span class="text-xs font-mono text-[#BC13FE] font-bold tracking-widest">"PETE.AI // MIRROR"</span>
                            <button
                                class="text-[var(--accent-primary)] hover:text-white"
                                on:click=move |_| set_show_ai_mirror.set(false)
                            >
                                "MINIMIZE"
                            </button>
                        </div>
                        <div class="flex-grow overflow-hidden">
                            <AiMirrorChat/>
                        </div>
                    </div>
                </div>
            })}
        </div>
    }
}
