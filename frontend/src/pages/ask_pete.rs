use crate::components::boilermaker::ChamferedPanel;
use crate::components::loading_spinner::LoadingSpinner;
use crate::game::engine::GameEngine;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn AskPete() -> impl IntoView {
    // Rebranding: "Ask Pete"
    view! {
        <div class="min-h-screen bg-purdue-black text-purdue-dust font-sans selection:bg-purdue-gold selection:text-black">
            // Header / Navigation Bar Substitute
            <div class="w-full bg-purdue-dark border-b border-purdue-gold/20 p-4">
                <div class="max-w-6xl mx-auto flex items-center gap-4">
                    <div class="w-12 h-12 bg-purdue-gold text-black rounded-full flex items-center justify-center font-black text-2xl border-2 border-white shadow-lg">
                        "P"
                    </div>
                    <div>
                        <h1 class="text-2xl font-bold text-purdue-gold tracking-wide uppercase">"Ask Pete"</h1>
                        <p class="text-xs text-slate-400 uppercase tracking-widest">"Purdue Expert Tuition Engine"</p>
                        <h2 class="text-2xl font-black text-old-gold uppercase tracking-wider">"Welcome to the Forge"</h2>
                    </div>
                    <ChamferedPanel class="border-l-4 border-l-old-gold">
                    <div class="space-y-4 text-steam-white">
                        <p class="text-lg leading-relaxed">
                            "I am "
                            <span class="font-black text-old-gold">"Pete"</span>
                            ", your "
                            <span class="font-bold">"Socratic Conductor"</span>
                            ". My job is not to give you answers, but to help you "
                            <span class="font-bold text-old-gold">"construct"</span>
                            " them."
                        </p>
                        <div class="h-px w-full bg-gradient-to-r from-transparent via-old-gold/30 to-transparent"></div>
                        <p class="text-purdue-dust font-mono">
                            "Below is the active Knowledge Graph, rendered as an interactive exploration. Each choice you make forges a new pathway through the material."
                        </p>
                        <p class="text-sm text-old-gold font-bold uppercase tracking-wider">
                            "→ Let's begin."
                        </p>
                    </div>
                </ChamferedPanel>

                // The Game Interface
                <PeteTerminal />
            </div>
            </div>
        </div>
    }
}

#[component]
fn PeteTerminal() -> impl IntoView {
    // State
    let (engine, set_engine) = signal(None::<GameEngine>);
    let (error_msg, set_error_msg) = signal(None::<String>);

    // Fetch the graph on load
    Effect::new(move |_| {
        spawn_local(async move {
            match crate::api::get_graph().await {
                Ok(graph) => {
                    // Initialize Engine with fetched graph
                    set_engine.set(Some(GameEngine::new(graph)));
                }
                Err(e) => set_error_msg.set(Some(format!("Could not load Pete's brain: {}", e))),
            }
        });
    });

    let handle_choice = move |target_id: String| {
        set_engine.update(|e| {
            if let Some(eng) = e {
                eng.make_choice(target_id);
            }
        });
    };

    let restart = move |_| {
        spawn_local(async move {
            if let Ok(graph) = crate::api::get_graph().await {
                set_engine.set(Some(GameEngine::new(graph)));
            }
        });
    };

    view! {
        <ChamferedPanel>
            <div class="min-h-[500px] flex flex-col relative">
                // Background Watermark
                <div class="absolute inset-0 flex items-center justify-center opacity-5 pointer-events-none">
                    <h1 class="text-9xl font-black text-white">"PETE"</h1>
                </div>

                <Show when=move || error_msg.get().is_some()>
                    <div class="p-8 text-signal-red bg-signal-red/10 border border-signal-red chamfered-corners">
                        {move || error_msg.get()}
                    </div>
                </Show>

                <Show
                    when=move || engine.get().is_some()
                    fallback=|| view! {
                        <div class="flex-grow flex items-center justify-center text-old-gold gap-3">
                            <LoadingSpinner message="Stoking the Engine...".to_string() size="lg".to_string() />
                        </div>
                    }
                >
                    {move || {
                        let current_engine = engine.get().unwrap();
                        let current_node = current_engine.get_current_node();
                        let options = current_engine.get_options();

                        view! {
                            <div class="flex-grow flex flex-col justify-between z-10 p-6">
                                // Content Area
                                <div class="space-y-6 animate-fade-in">
                                    <div class="flex items-center gap-3 text-old-gold/60 text-sm uppercase tracking-widest font-bold font-mono">
                                        <span class="w-2 h-2 bg-old-gold rounded-full animate-pulse"></span>
                                        "Current Node"
                                    </div>

                                    <h2 class="text-4xl font-bold text-white leading-tight">
                                        {current_node.map(|n| n.title.clone()).unwrap_or("End of Path".to_string())}
                                    </h2>

                                    <div class="prose prose-invert prose-lg text-slate-300">
                                        {current_node.map(|n| n.content.clone()).unwrap_or("The simulation has ended.".to_string())}
                                    </div>
                                </div>

                                // Choices Area
                                <div class="mt-12 space-y-4">
                                    <div class="h-px w-full bg-gradient-to-r from-transparent via-old-gold/30 to-transparent"></div>
                                    <p class="text-center text-sm text-slate-500 uppercase font-mono">"Available Actions"</p>

                                    <div class="grid grid-cols-1 gap-3">
                                        {
                                            let options_for_list = options.clone();
                                            view! {
                                                <For
                                                    each=move || options_for_list.clone()
                                                    key=|opt| opt.0.clone()
                                                    children=move |(id, title)| {
                                                        let target = id.clone();
                                                        view! {
                                                            <button
                                                                class="group relative w-full p-4 text-left transition-all duration-200 hover:scale-[1.01]"
                                                                on:click=move |_| handle_choice(target.clone())
                                                            >
                                                                <div class="absolute inset-0 bg-white/5 chamfered-corners border border-white/10 group-hover:bg-old-gold/10 group-hover:border-old-gold transition-colors"></div>
                                                                <div class="relative flex items-center justify-between px-2">
                                                                    <span class="font-bold text-purdue-dust group-hover:text-white transition-colors">
                                                                        {title}
                                                                    </span>
                                                                    <span class="text-old-gold opacity-0 group-hover:opacity-100 transition-opacity">
                                                                        "→"
                                                                    </span>
                                                                </div>
                                                            </button>
                                                        }
                                                    }
                                                />
                                            }
                                        }

                                        <Show when=move || options.is_empty()>
                                            <button
                                                class="w-full p-4 bg-old-gold text-black font-bold chamfered-corners hover:bg-white hover:shadow-lg hover:shadow-old-gold/50 transition-all uppercase tracking-wide"
                                                on:click=restart
                                            >
                                                "↺ Restart Simulation"
                                            </button>
                                        </Show>
                                    </div>
                                </div>
                            </div>
                        }
                    }}
                </Show>
            </div>
        </ChamferedPanel>
    }
}
