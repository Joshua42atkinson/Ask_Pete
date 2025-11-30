use pete_core::expert::StoryNode;
use pete_core::models::triggers::{TriggerCondition, TriggerEffect};
use leptos::prelude::*;
// use wasm_bindgen::JsCast; // Unused

#[component]
pub fn Inspector(
    #[prop(into)] node: RwSignal<StoryNode>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_delete: Callback<()>,
) -> impl IntoView {
    let (active_tab, set_active_tab) = signal("properties".to_string()); // "properties", "style", "logic"
    let (is_analyzing, set_is_analyzing) = signal(false);
    let (analysis_result, set_analysis_result) = signal(Option::<String>::None);

    let run_inspection = move |_| {
        set_is_analyzing.set(true);
        set_timeout(
            move || {
                set_is_analyzing.set(false);
                set_analysis_result.set(Some("Analysis complete. The node structure is sound, but the narrative flow could be improved.".to_string()));
            },
            std::time::Duration::from_secs(2),
        );
    };

    view! {
        <div class="absolute right-0 top-0 bottom-0 w-96 bg-slate-900 border-l border-white/10 shadow-2xl flex flex-col z-30">
            // Header
            <div class="h-16 flex items-center justify-between px-6 border-b border-white/10 bg-slate-950 shrink-0">
                <div class="flex items-center gap-2">
                    <span class="text-2xl">"🕵️"</span>
                    <h2 class="text-lg font-bold text-white tracking-wider">"THE INSPECTOR"</h2>
                </div>
                <button
                    class="text-slate-400 hover:text-white transition-colors"
                    on:click=move |_| on_close.run(())
                >
                    "✕"
                </button>
            </div>

            // Tabs
            <div class="flex border-b border-white/10 bg-slate-900">
                <button
                    class=move || format!("flex-1 py-2 text-sm font-bold transition-colors {}", if active_tab.get() == "properties" { "text-boilermaker-gold border-b-2 border-boilermaker-gold" } else { "text-slate-500 hover:text-slate-300" })
                    on:click=move |_| set_active_tab.set("properties".to_string())
                >
                    "Properties"
                </button>
                <button
                    class=move || format!("flex-1 py-2 text-sm font-bold transition-colors {}", if active_tab.get() == "logic" { "text-boilermaker-gold border-b-2 border-boilermaker-gold" } else { "text-slate-500 hover:text-slate-300" })
                    on:click=move |_| set_active_tab.set("logic".to_string())
                >
                    "Logic"
                </button>
                <button
                    class=move || format!("flex-1 py-2 text-sm font-bold transition-colors {}", if active_tab.get() == "style" { "text-boilermaker-gold border-b-2 border-boilermaker-gold" } else { "text-slate-500 hover:text-slate-300" })
                    on:click=move |_| set_active_tab.set("style".to_string())
                >
                    "Style"
                </button>
            </div>

            // Content
            <div class="flex-grow overflow-y-auto p-6 space-y-6">
                {move || match active_tab.get().as_str() {
                    "properties" => view! {
                        <div class="space-y-6">
                            // Node Identity
                            <div class="space-y-2">
                                <label class="text-xs uppercase font-bold text-slate-500">"Station Name"</label>
                                <input
                                    type="text"
                                    class="w-full bg-slate-800 border border-slate-700 rounded p-2 text-white focus:border-cyan-500 focus:outline-none transition-colors"
                                    prop:value=move || node.get().title
                                    on:input=move |ev| {
                                        let val = event_target_value(&ev);
                                        node.update(|n| n.title = val);
                                    }
                                />
                            </div>

                            <div class="space-y-2">
                                <label class="text-xs uppercase font-bold text-slate-500">"Learning Content"</label>
                                <textarea
                                    class="w-full h-32 bg-slate-800 border border-slate-700 rounded p-2 text-white focus:border-cyan-500 focus:outline-none transition-colors font-mono text-sm"
                                    prop:value=move || node.get().content
                                    on:input=move |ev| {
                                        let val = event_target_value(&ev);
                                        node.update(|n| n.content = val);
                                    }
                                />
                            </div>

                            // Cognitive Load Controls
                            <div class="p-4 bg-slate-800 rounded border border-slate-700 space-y-4">
                                <h3 class="font-bold text-slate-300 flex items-center gap-2">
                                    "📦 Cargo Manifest"
                                </h3>

                                <div class="space-y-2">
                                    <div class="flex justify-between text-sm">
                                        <span class="text-slate-400">"Passenger Count"</span>
                                        <span class="font-mono font-bold text-white">{move || node.get().passenger_count}</span>
                                    </div>
                                    <input
                                        type="range"
                                        min="1"
                                        max="10"
                                        class="w-full accent-cyan-500"
                                        prop:value=move || node.get().passenger_count
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev).parse::<u8>().unwrap_or(1);
                                            node.update(|n| n.passenger_count = val);
                                        }
                                    />
                                    <div class="flex justify-between text-xs text-slate-500">
                                        <span>"Light"</span>
                                        <span>"Heavy"</span>
                                    </div>
                                </div>
                            </div>

                            // AI Inspector Section
                            <div class="border-t border-white/10 pt-6">
                                <button
                                    class="w-full py-3 bg-boilermaker-gold hover:bg-white text-black font-bold rounded shadow-lg transition-all flex items-center justify-center gap-2"
                                    on:click=run_inspection
                                    disabled=move || is_analyzing.get()
                                >
                                    {move || if is_analyzing.get() {
                                        view! { <span class="animate-spin">"⚙️"</span> " Inspecting..." }.into_any()
                                    } else {
                                        view! { <span>"🔍"</span> " Run Inspection" }.into_any()
                                    }}
                                </button>

                                {move || analysis_result.get().map(|result| {
                                    view! {
                                        <div class="mt-4 p-4 bg-slate-800 rounded border-l-4 border-cyan-500 animate-in fade-in slide-in-from-bottom-2">
                                            <div class="prose prose-invert prose-sm">
                                                <pre class="whitespace-pre-wrap font-sans text-slate-300">{result}</pre>
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    }.into_any(),
                    "logic" => view! {
                        <div class="space-y-6">
                            <div class="p-4 bg-slate-800/50 rounded border border-slate-700">
                                <h3 class="font-bold text-purdue-gold mb-2 flex items-center gap-2">
                                    <span>"🔒"</span> "Unlock Condition"
                                </h3>
                                <p class="text-xs text-slate-400 mb-4">"What must be true to enter this station?"</p>

                                <div class="space-y-3">
                                    <select
                                        class="w-full bg-slate-900 border border-slate-600 rounded p-2 text-sm text-white"
                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            node.update(|n| {
                                                n.logic.condition = match val.as_str() {
                                                    "HasItem" => TriggerCondition::HasItem { item_id: "".to_string() },
                                                    "GreaterThan" => TriggerCondition::GreaterThan { variable: "".to_string(), value: 0.0 },
                                                    _ => TriggerCondition::None,
                                                };
                                            });
                                        }
                                    >
                                        <option value="None" selected=move || matches!(node.get().logic.condition, TriggerCondition::None)>"None (Always Open)"</option>
                                        <option value="HasItem" selected=move || matches!(node.get().logic.condition, TriggerCondition::HasItem { .. })>"Requires Item"</option>
                                        <option value="GreaterThan" selected=move || matches!(node.get().logic.condition, TriggerCondition::GreaterThan { .. })>"Variable > Value"</option>
                                    </select>

                                    {move || match node.get().logic.condition {
                                        TriggerCondition::HasItem { item_id } => view! {
                                            <input
                                                type="text"
                                                placeholder="Item ID (e.g., 'rusty_key')"
                                                class="w-full bg-slate-900 border border-slate-600 rounded p-2 text-sm text-white"
                                                prop:value=item_id
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev);
                                                    node.update(|n| n.logic.condition = TriggerCondition::HasItem { item_id: val });
                                                }
                                            />
                                        }.into_any(),
                                        TriggerCondition::GreaterThan { variable, value } => view! {
                                            <div class="flex gap-2">
                                                <input
                                                    type="text"
                                                    placeholder="Variable (e.g., 'strength')"
                                                    class="w-2/3 bg-slate-900 border border-slate-600 rounded p-2 text-sm text-white"
                                                    prop:value=variable
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        node.update(|n| {
                                                            if let TriggerCondition::GreaterThan { value, .. } = &n.logic.condition {
                                                                n.logic.condition = TriggerCondition::GreaterThan { variable: val, value: *value };
                                                            }
                                                        });
                                                    }
                                                />
                                                <input
                                                    type="number"
                                                    placeholder="0"
                                                    class="w-1/3 bg-slate-900 border border-slate-600 rounded p-2 text-sm text-white"
                                                    prop:value=value
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev).parse::<f32>().unwrap_or(0.0);
                                                        node.update(|n| {
                                                            if let TriggerCondition::GreaterThan { variable, .. } = &n.logic.condition {
                                                                n.logic.condition = TriggerCondition::GreaterThan { variable: variable.clone(), value: val };
                                                            }
                                                        });
                                                    }
                                                />
                                            </div>
                                        }.into_any(),
                                        _ => view! { <div class="text-xs text-slate-600 italic">"No condition set."</div> }.into_any()
                                    }}
                                </div>
                            </div>

                            <div class="p-4 bg-slate-800/50 rounded border border-slate-700">
                                <h3 class="font-bold text-purdue-gold mb-2 flex items-center gap-2">
                                    <span>"🎁"</span> "Completion Effect"
                                </h3>
                                <p class="text-xs text-slate-400 mb-4">"What happens when the player completes this station?"</p>

                                <div class="space-y-3">
                                    <select
                                        class="w-full bg-slate-900 border border-slate-600 rounded p-2 text-sm text-white"
                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            node.update(|n| {
                                                n.logic.effect = match val.as_str() {
                                                    "GrantItem" => TriggerEffect::GrantItem { item_id: "".to_string() },
                                                    "ModifyVariable" => TriggerEffect::ModifyVariable { variable: "".to_string(), delta: 1.0 },
                                                    _ => TriggerEffect::None,
                                                };
                                            });
                                        }
                                    >
                                        <option value="None" selected=move || matches!(node.get().logic.effect, TriggerEffect::None)>"None"</option>
                                        <option value="GrantItem" selected=move || matches!(node.get().logic.effect, TriggerEffect::GrantItem { .. })>"Grant Item"</option>
                                        <option value="ModifyVariable" selected=move || matches!(node.get().logic.effect, TriggerEffect::ModifyVariable { .. })>"Modify Variable"</option>
                                    </select>

                                    {move || match node.get().logic.effect {
                                        TriggerEffect::GrantItem { item_id } => view! {
                                            <input
                                                type="text"
                                                placeholder="Item ID to Grant"
                                                class="w-full bg-slate-900 border border-slate-600 rounded p-2 text-sm text-white"
                                                prop:value=item_id
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev);
                                                    node.update(|n| n.logic.effect = TriggerEffect::GrantItem { item_id: val });
                                                }
                                            />
                                        }.into_any(),
                                        TriggerEffect::ModifyVariable { variable, delta } => view! {
                                            <div class="flex gap-2">
                                                <input
                                                    type="text"
                                                    placeholder="Variable"
                                                    class="w-2/3 bg-slate-900 border border-slate-600 rounded p-2 text-sm text-white"
                                                    prop:value=variable
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        node.update(|n| {
                                                            if let TriggerEffect::ModifyVariable { delta, .. } = &n.logic.effect {
                                                                n.logic.effect = TriggerEffect::ModifyVariable { variable: val, delta: *delta };
                                                            }
                                                        });
                                                    }
                                                />
                                                <input
                                                    type="number"
                                                    placeholder="+1"
                                                    class="w-1/3 bg-slate-900 border border-slate-600 rounded p-2 text-sm text-white"
                                                    prop:value=delta
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev).parse::<f32>().unwrap_or(0.0);
                                                        node.update(|n| {
                                                            if let TriggerEffect::ModifyVariable { variable, .. } = &n.logic.effect {
                                                                n.logic.effect = TriggerEffect::ModifyVariable { variable: variable.clone(), delta: val };
                                                            }
                                                        });
                                                    }
                                                />
                                            </div>
                                        }.into_any(),
                                        _ => view! { <div class="text-xs text-slate-600 italic">"No effect set."</div> }.into_any()
                                    }}
                                </div>
                            </div>
                        </div>
                    }.into_any(),
                    "style" => view! {
                        <div class="space-y-6">
                            // Contrast
                            <div class="space-y-2">
                                <label class="block text-xs uppercase font-bold text-purdue-gold">"Contrast (Highlight)"</label>
                                <div class="flex items-center gap-3">
                                    <button
                                        class=move || format!("px-4 py-2 rounded border transition-colors {}", if node.get().style.contrast { "bg-yellow-500/20 border-yellow-500 text-yellow-400" } else { "bg-slate-800 border-slate-600 text-slate-400" })
                                        on:click=move |_| node.update(|n| n.style.contrast = !n.style.contrast)
                                    >
                                        "⚡ Power Node"
                                    </button>
                                    <p class="text-xs text-slate-500">"Make this node stand out visually."</p>
                                </div>
                            </div>

                            // Alignment
                            <div class="space-y-2">
                                <label class="block text-xs uppercase font-bold text-purdue-gold">"Alignment"</label>
                                <div class="flex bg-slate-800 rounded border border-slate-600 p-1">
                                    <button
                                        class=move || format!("flex-1 py-1 rounded text-xs {}", if node.get().style.alignment == "left" { "bg-slate-600 text-white" } else { "text-slate-400 hover:text-white" })
                                        on:click=move |_| node.update(|n| n.style.alignment = "left".to_string())
                                    >
                                        "Left"
                                    </button>
                                    <button
                                        class=move || format!("flex-1 py-1 rounded text-xs {}", if node.get().style.alignment == "center" { "bg-slate-600 text-white" } else { "text-slate-400 hover:text-white" })
                                        on:click=move |_| node.update(|n| n.style.alignment = "center".to_string())
                                    >
                                        "Center"
                                    </button>
                                    <button
                                        class=move || format!("flex-1 py-1 rounded text-xs {}", if node.get().style.alignment == "right" { "bg-slate-600 text-white" } else { "text-slate-400 hover:text-white" })
                                        on:click=move |_| node.update(|n| n.style.alignment = "right".to_string())
                                    >
                                        "Right"
                                    </button>
                                </div>
                            </div>

                            // Proximity
                            <div class="space-y-2">
                                <div class="flex justify-between text-sm">
                                    <span class="text-purdue-gold font-bold text-xs uppercase">"Proximity (Padding)"</span>
                                    <span class="font-mono font-bold text-white">{move || format!("{:.1}x", node.get().style.proximity)}</span>
                                </div>
                                <input
                                    type="range"
                                    min="0.5"
                                    max="3.0"
                                    step="0.1"
                                    class="w-full accent-boilermaker-gold"
                                    prop:value=move || node.get().style.proximity
                                    on:input=move |ev| {
                                        let val = event_target_value(&ev).parse::<f32>().unwrap_or(1.0);
                                        node.update(|n| n.style.proximity = val);
                                    }
                                />
                            </div>
                        </div>
                    }.into_any(),
                    _ => view! { <div>"Unknown Tab"</div> }.into_any()
                }}

                // Delete Button (Always visible at bottom)
                <div class="pt-4 border-t border-white/10">
                    <button
                        class="w-full px-4 py-2 bg-red-900/50 hover:bg-red-900 text-red-200 border border-red-800 rounded transition-colors text-sm font-bold"
                        on:click=move |_| on_delete.run(())
                    >
                        "Demolish Station"
                    </button>
                </div>
            </div>
        </div>
    }
}
