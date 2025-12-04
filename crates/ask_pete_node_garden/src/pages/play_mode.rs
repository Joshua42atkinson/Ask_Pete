use crate::api::get_graph;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use pete_core::expert::StoryGraph;
use pete_core::models::triggers::{GameState, TriggerCondition, TriggerEffect};

#[derive(Params, PartialEq, Clone, Debug)]
pub struct PlayModeParams {
    pub id: Option<String>,
}

#[component]
pub fn PlayMode() -> impl IntoView {
    let params = use_params::<PlayModeParams>();
    let (graph, set_graph) = signal(None::<StoryGraph>);
    let (current_node_id, set_current_node_id) = signal(None::<String>);
    let (game_state, set_game_state) = signal(GameState::new());
    let (_history, set_history) = signal(Vec::<String>::new()); // Log of events

    // Toast Notification State
    let (toast_message, set_toast_message) = signal(None::<String>);

    // Load graph
    Effect::new(move |_| {
        let id = params
            .get()
            .map(|p| p.id)
            .unwrap_or_default()
            .unwrap_or_default();
        if !id.is_empty() {
            leptos::task::spawn_local(async move {
                if let Ok(g) = get_graph(Some(id)).await {
                    set_graph.set(Some(g.clone()));
                    // Start at the first node (or find a start node)
                    if let Some(first) = g.nodes.first() {
                        set_current_node_id.set(Some(first.id.clone()));
                    }
                }
            });
        }
    });

    // Helper to evaluate condition
    let check_condition = move |condition: &TriggerCondition, state: &GameState| -> bool {
        match condition {
            TriggerCondition::None => true,
            TriggerCondition::GreaterThan { variable, value } => state.get_var(variable) > *value,
            TriggerCondition::LessThan { variable, value } => state.get_var(variable) < *value,
            TriggerCondition::Equals { variable, value } => {
                (state.get_var(variable) - *value).abs() < 0.001
            }
            TriggerCondition::HasItem { item_id } => state.has_item(item_id),
        }
    };

    // Handle node visit (apply effects)
    Effect::new(move |_| {
        if let Some(node_id) = current_node_id.get() {
            if let Some(g) = graph.get() {
                if let Some(node) = g.nodes.iter().find(|n| n.id == node_id) {
                    // Apply effect
                    let effect = node.logic.effect.clone();
                    let title = node.title.clone();

                    set_game_state.update(|state| match &effect {
                        TriggerEffect::None => {}
                        TriggerEffect::ModifyVariable { variable, delta } => {
                            let current = state.get_var(variable);
                            state.set_var(variable.clone(), current + delta);
                            set_toast_message
                                .set(Some(format!("{} changed by {}", variable, delta)));
                        }
                        TriggerEffect::GrantItem { item_id } => {
                            if !state.inventory.contains(item_id) {
                                state.inventory.push(item_id.clone());
                                set_toast_message.set(Some(format!("Acquired: {}", item_id)));
                            }
                        }
                        TriggerEffect::ConsumeItem { item_id } => {
                            state.inventory.retain(|i| i != item_id);
                            set_toast_message.set(Some(format!("Used: {}", item_id)));
                        }
                    });

                    // Log visit
                    set_history.update(|h| h.push(format!("Visited: {}", title)));

                    // Clear toast after 3 seconds
                    if toast_message.get().is_some() {
                        leptos::task::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(3000).await;
                            set_toast_message.set(None);
                        });
                    }
                }
            }
        }
    });

    // Get current node
    let current_node = move || {
        graph.get().and_then(|g| {
            current_node_id
                .get()
                .and_then(|id| g.nodes.into_iter().find(|n| n.id == id))
        })
    };

    // Get available choices (connections)
    let choices = move || {
        if let Some(g) = graph.get() {
            if let Some(id) = current_node_id.get() {
                return g
                    .connections
                    .iter()
                    .filter(|c| c.from_node == id)
                    .filter_map(|c| {
                        let target = g.nodes.iter().find(|n| n.id == c.to_node)?;

                        if check_condition(&target.logic.condition, &game_state.get()) {
                            Some(target.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
            }
        }
        Vec::new()
    };

    // Simple Markdown Parser (Bold, Italic, Paragraphs)
    let render_content = move |content: String| -> Vec<String> {
        content.split("\n\n").map(|s| s.to_string()).collect()
    };

    let parse_inline = move |text: &str| -> Vec<AnyView> {
        // Very basic parser: split by ** for bold
        let parts: Vec<&str> = text.split("**").collect();
        parts
            .iter()
            .enumerate()
            .map(|(i, part)| {
                if i % 2 == 1 {
                    view! { <strong class="text-boilermaker-gold">{part.to_string()}</strong> }
                        .into_any()
                } else {
                    view! { <span>{part.to_string()}</span> }.into_any()
                }
            })
            .collect()
    };

    view! {
        <div class="min-h-screen bg-slate-950 text-slate-200 font-sans flex flex-col relative overflow-hidden">
            // Background Ambience
            <div class="absolute inset-0 pointer-events-none opacity-20"
                 style="background-image: radial-gradient(circle at 50% 50%, #1e293b 0%, #020617 100%);">
            </div>

            // Header
            <header class="bg-slate-900/80 backdrop-blur border-b border-slate-800 p-4 flex justify-between items-center z-10 sticky top-0">
                <h1 class="text-xl font-bold text-boilermaker-gold flex items-center gap-2">
                    <span>"🚂"</span> "Play Mode"
                </h1>
                <div class="flex gap-6 text-sm font-mono">
                    <div class="flex gap-2 items-center">
                        <span class="text-slate-500">"INVENTORY"</span>
                        <span class="text-white bg-slate-800 px-2 py-0.5 rounded border border-slate-700">
                            {move || if game_state.get().inventory.is_empty() {
                                "Empty".to_string()
                            } else {
                                game_state.get().inventory.join(", ")
                            }}
                        </span>
                    </div>
                    <div class="flex gap-2 items-center">
                        <span class="text-slate-500">"STATS"</span>
                        <span class="text-cyan-400">{move || format!("{:?}", game_state.get().variables)}</span>
                    </div>
                </div>
            </header>

            // Main Content
            <main class="flex-1 max-w-3xl mx-auto w-full p-8 flex flex-col gap-8 z-10">
                {move || if let Some(node) = current_node() {
                    view! {
                        <div class="animate-fade-in space-y-8">
                            // Node Title
                            <div class="border-b border-slate-800 pb-4">
                                <h2 class="text-4xl font-bold text-white tracking-tight">{node.title}</h2>
                                <div class="flex gap-2 mt-2">
                                    <span class="text-xs font-mono text-slate-500 bg-slate-900 px-2 py-1 rounded">
                                        {format!("Complexity: {}", node.complexity_level)}
                                    </span>
                                </div>
                            </div>

                            // Node Content (Rich Text)
                            <div class="prose prose-invert prose-lg max-w-none">
                                {render_content(node.content).into_iter().map(|paragraph| {
                                    view! {
                                        <p class="mb-4 leading-relaxed text-slate-300">
                                            {parse_inline(&paragraph)}
                                        </p>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>

                            // Choices
                            <div class="mt-12 space-y-4">
                                <h3 class="text-xs uppercase tracking-widest text-slate-500 font-bold mb-4">"Available Routes"</h3>
                                <div class="grid gap-4">
                                    <For
                                        each=choices
                                        key=|n| n.id.clone()
                                        children=move |target| {
                                            let target_id = target.id.clone();
                                            view! {
                                                <button
                                                    class="group relative w-full text-left p-6 bg-slate-900/50 hover:bg-slate-800 border border-slate-700 hover:border-boilermaker-gold rounded-xl transition-all duration-200 shadow-lg hover:shadow-boilermaker-gold/10"
                                                    on:click=move |_| set_current_node_id.set(Some(target_id.clone()))
                                                >
                                                    <div class="flex items-center justify-between">
                                                        <span class="text-xl font-semibold text-slate-200 group-hover:text-white transition-colors">
                                                            {target.title}
                                                        </span>
                                                        <span class="text-slate-500 group-hover:text-boilermaker-gold transition-colors text-2xl">
                                                            "➔"
                                                        </span>
                                                    </div>
                                                    // Optional: Preview text could go here
                                                </button>
                                            }
                                        }
                                    />
                                </div>

                                {move || if choices().is_empty() {
                                    view! {
                                        <div class="p-8 bg-slate-900/30 border border-slate-800 rounded-xl text-center">
                                            <div class="text-4xl mb-4">"🏁"</div>
                                            <p class="text-slate-400 text-lg italic">"End of the Line"</p>
                                            <button
                                                class="mt-4 px-4 py-2 bg-slate-800 hover:bg-slate-700 text-white rounded transition-colors"
                                                on:click=move |_| window().history().unwrap().back().unwrap()
                                            >
                                                "Return to Editor"
                                            </button>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span /> }.into_any()
                                }}
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="flex items-center justify-center h-64">
                            <div class="flex flex-col items-center gap-4">
                                <div class="animate-spin h-8 w-8 border-4 border-boilermaker-gold border-t-transparent rounded-full"></div>
                                <p class="text-slate-500">"Loading Scenario..."</p>
                            </div>
                        </div>
                    }.into_any()
                }}
            </main>

            // Toast Notification
            {move || toast_message.get().map(|msg| {
                view! {
                    <div class="fixed bottom-8 right-8 z-50 animate-slide-in-up">
                        <div class="bg-slate-800 border-l-4 border-boilermaker-gold text-white px-6 py-4 rounded shadow-2xl flex items-center gap-3">
                            <span class="text-2xl">"🔔"</span>
                            <div>
                                <p class="font-bold text-sm text-boilermaker-gold">"UPDATE"</p>
                                <p class="text-sm">{msg}</p>
                            </div>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
