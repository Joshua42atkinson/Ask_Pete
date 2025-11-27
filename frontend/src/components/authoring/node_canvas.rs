use crate::api::{get_graph, save_graph};
use crate::components::authoring::property_editor::PropertyEditor;
use crate::components::authoring::story_node::StoryNodeComponent;
use crate::components::authoring::template_selector::TemplateSelector;
use common::expert::{Connection, StoryGraph, StoryNode};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn NodeCanvas() -> impl IntoView {
    let (nodes, set_nodes) = signal(Vec::<RwSignal<StoryNode>>::new());
    let (connections, set_connections) = signal(Vec::<Connection>::new());
    let (graph_meta, set_graph_meta) = signal((String::new(), String::new())); // id, title
    let (dragging_id, set_dragging_id) = signal(None::<String>);
    let (selected_node_id, set_selected_node_id) = signal(None::<String>);
    let (offset, set_offset) = signal((0.0, 0.0)); // Mouse offset relative to node top-left
    let (show_template_selector, set_show_template_selector) = signal(false);

    // Loading and error states
    let (loading, set_loading) = signal(true);
    let (error_message, set_error_message) = signal(None::<String>);

    // Connection dragging state
    let (connecting_source, set_connecting_source) = signal(None::<String>); // node_id
    let (mouse_pos, set_mouse_pos) = signal((0.0, 0.0)); // For drawing the temp line

    // Load graph on mount
    Effect::new(move |_| {
        leptos::logging::log!("🎨 NodeCanvas: Starting graph load...");
        set_loading.set(true);
        set_error_message.set(None);

        leptos::task::spawn_local(async move {
            leptos::logging::log!("🌐 NodeCanvas: Fetching graph from API...");
            match get_graph().await {
                Ok(graph) => {
                    leptos::logging::log!("✅ NodeCanvas: Graph loaded successfully!");
                    let node_signals = graph.nodes.into_iter().map(|n| RwSignal::new(n)).collect();
                    set_nodes.set(node_signals);
                    set_connections.set(graph.connections);
                    set_graph_meta.set((graph.id, graph.title));
                    set_loading.set(false);
                }
                Err(e) => {
                    let error_msg = format!("Failed to load graph: {}", e);
                    leptos::logging::error!("❌ NodeCanvas: {}", error_msg);
                    set_error_message.set(Some(error_msg));
                    set_loading.set(false);
                }
            }
        });
    });

    // Save graph handler
    let save_graph_handler = move |_| {
        leptos::task::spawn_local(async move {
            let current_nodes: Vec<StoryNode> = nodes.get().iter().map(|s| s.get()).collect();
            let current_connections = connections.get();
            let (id, title) = graph_meta.get();
            let graph = StoryGraph {
                id: if id.is_empty() {
                    "demo_graph".to_string()
                } else {
                    id
                },
                title: if title.is_empty() {
                    "New Story".to_string()
                } else {
                    title
                },
                nodes: current_nodes,
                connections: current_connections,
            };

            match save_graph(graph).await {
                Ok(_) => leptos::logging::log!("Graph saved successfully"),
                Err(e) => leptos::logging::error!("Failed to save graph: {}", e),
            }
        });
    };

    let on_mouse_move = move |ev: web_sys::MouseEvent| {
        let mx = ev.client_x() as f64;
        let my = ev.client_y() as f64;
        set_mouse_pos.set((mx, my));

        if let Some(id) = dragging_id.get() {
            // Find the node being dragged
            if let Some(node_signal) = nodes.get().iter().find(|n| n.get().id == id) {
                node_signal.update(|n| {
                    n.x = mx - offset.get().0;
                    n.y = my - offset.get().1;
                });
            }
        }
    };

    let on_mouse_up = move |_| {
        set_dragging_id.set(None);
        set_connecting_source.set(None);
    };

    // Port handlers
    let on_port_mousedown = move |node_id: String, port_type: String| {
        if port_type == "output" {
            set_connecting_source.set(Some(node_id));
        }
    };

    let on_port_mouseup = move |node_id: String, port_type: String| {
        if let Some(source_id) = connecting_source.get() {
            if port_type == "input" && source_id != node_id {
                // Create connection
                set_connections.update(|c| {
                    c.push(Connection {
                        id: uuid::Uuid::new_v4().to_string(),
                        from_node: source_id,
                        to_node: node_id,
                    });
                });
                set_connecting_source.set(None);
            }
        }
    };

    // Helper to get node position by ID
    let get_node_pos = move |id: &str| -> Option<(f64, f64)> {
        nodes.get().iter().find(|n| n.get().id == id).map(|n| {
            let d = n.get();
            (d.x, d.y)
        })
    };

    view! {
        <div class="relative w-full h-full bg-railyard-dark overflow-hidden"
             style="background-image: radial-gradient(#3d3d5c 1px, transparent 1px); background-size: 40px 40px;"
             on:mousemove=on_mouse_move
             on:mouseup=on_mouse_up
             on:mouseleave=on_mouse_up
        >
            // Toolbar
            <div class="absolute top-4 right-4 z-10 flex gap-2">
                <button
                    class="px-4 py-2 bg-boilermaker-gold hover:bg-white text-black rounded shadow-lg font-bold transition-colors border-2 border-boilermaker-gold flex items-center gap-2"
                    on:click=move |_| set_show_template_selector.set(true)
                >
                    <span>"🤖"</span>
                    "Design with Pete"
                </button>
                <button
                    class="px-4 py-2 bg-slate-800 hover:bg-slate-700 border border-white/10 text-white rounded shadow-lg font-bold transition-colors"
                    on:click=move |_| {
                        let new_node = StoryNode {
                            id: uuid::Uuid::new_v4().to_string(),
                            title: "New Station".to_string(),
                            content: "Learning objective...".to_string(),
                            x: 100.0,
                            y: 100.0,
                            passenger_count: 1,
                            complexity_level: 1,
                            learner_profiles: vec![],
                            gardens_active: vec![],
                        };
                        set_nodes.update(|n| n.push(RwSignal::new(new_node)));
                    }
                >
                    "+ Add Station"
                </button>
                <button
                    class="px-4 py-2 bg-cyan-600 hover:bg-cyan-500 text-white rounded shadow-lg font-bold transition-colors"
                    on:click=save_graph_handler
                >
                    "Save Graph"
                </button>
            </div>

            // SVG Layer for Connections (Tracks)
            <svg class="absolute inset-0 w-full h-full pointer-events-none z-0">
                <defs>
                    // Train Signal Arrow
                    <marker id="arrowhead" markerWidth="12" markerHeight="12" refX="10" refY="6" orient="auto">
                        <path d="M0,0 L12,6 L0,12 L4,6 Z" fill="#CFB991" />
                    </marker>
                    // Track Pattern (Ties)
                    <pattern id="track-ties" x="0" y="0" width="20" height="8" patternUnits="userSpaceOnUse" patternTransform="rotate(90)">
                        <rect x="0" y="0" width="20" height="4" fill="#3d3d5c" />
                    </pattern>
                </defs>
                <For
                    each=move || connections.get()
                    key=|c| c.id.clone()
                    children=move |c| {
                        let from_pos = get_node_pos(&c.from_node).unwrap_or((0.0, 0.0));
                        let to_pos = get_node_pos(&c.to_node).unwrap_or((0.0, 0.0));
                        // Adjust for station width (w-72 = 288px) and height approx 150px
                        // Input/Output ports are vertically centered
                        let x1 = from_pos.0 + 288.0;
                        let y1 = from_pos.1 + 75.0;
                        let x2 = to_pos.0;
                        let y2 = to_pos.1 + 75.0;

                        // Bezier Control Points
                        let curvature = 0.5 * (x2 - x1).abs().max(100.0);
                        let cp1_x = x1 + curvature;
                        let cp1_y = y1;
                        let cp2_x = x2 - curvature;
                        let cp2_y = y2;

                        let path_d = format!(
                            "M {} {} C {} {}, {} {}, {} {}",
                            x1, y1, cp1_x, cp1_y, cp2_x, cp2_y, x2, y2
                        );

                        view! {
                            <g>
                                // Track Bed (Thick Dark Line)
                                <path d=path_d.clone() stroke="#1a1a2e" stroke-width="12" fill="none" />
                                // Track Ties (Dashed Line)
                                <path d=path_d.clone() stroke="#3d3d5c" stroke-width="8" stroke-dasharray="4,8" fill="none" />
                                // Rails (Two Thin Lines)
                                <path d=path_d.clone() stroke="#64748b" stroke-width="6" fill="none" />
                                <path d=path_d.clone() stroke="#1a1a2e" stroke-width="2" fill="none" />
                                // Signal Arrow
                                <path d=path_d stroke="transparent" stroke-width="1" fill="none" marker-end="url(#arrowhead)" />
                            </g>
                        }
                    }
                />

                // Temp Connection Line
                {move || if let Some(source_id) = connecting_source.get() {
                    let from_pos = get_node_pos(&source_id).unwrap_or((0.0, 0.0));
                    let x1 = from_pos.0 + 288.0;
                    let y1 = from_pos.1 + 75.0;
                    let (x2, y2) = mouse_pos.get();
                    view! {
                        <line x1=x1 y1=y1 x2=x2 y2=y2 stroke="#CFB991" stroke-width="4" stroke-dasharray="10,5" />
                    }.into_any()
                } else {
                    view! { <span /> }.into_any()
                }}
            </svg>

            // Canvas Area
            <div class="absolute inset-0 z-10">
                <For
                    each=move || nodes.get()
                    key=|node| node.get().id
                    children=move |node| {
                        view! {
                            <StoryNodeComponent
                                node=node
                                on_mousedown=Callback::new(move |ev: web_sys::MouseEvent| {
                                    let id = node.get().id;
                                    set_dragging_id.set(Some(id.clone()));
                                    set_selected_node_id.set(Some(id));
                                    let rect = ev.target().unwrap().unchecked_into::<web_sys::Element>().get_bounding_client_rect();
                                    set_offset.set((
                                        ev.client_x() as f64 - rect.left(),
                                        ev.client_y() as f64 - rect.top(),
                                    ));
                                })
                                on_port_mousedown=on_port_mousedown
                                on_port_mouseup=on_port_mouseup
                            />
                        }
                    }
                />
            </div>

            // Loading / Error / Empty State Display
            {move || {
                if loading.get() {
                    view! {
                        <div class="absolute inset-0 flex items-center justify-center pointer-events-none z-20">
                            <div class="bg-slate-800/90 border border-cyan-500/30 rounded-lg p-6 flex flex-col items-center gap-3">
                                <div class="animate-spin h-8 w-8 border-4 border-cyan-500 border-t-transparent rounded-full"></div>
                                <p class="text-cyan-400 font-medium">"Loading train yard..."</p>
                            </div>
                        </div>
                    }.into_any()
                } else if let Some(error) = error_message.get() {
                    view! {
                        <div class="absolute inset-0 flex items-center justify-center pointer-events-none z-20">
                            <div class="bg-red-900/80 border-2 border-red-500 rounded-lg p-6 max-w-md">
                                <h3 class="text-red-200 font-bold text-lg mb-2">"⚠️ Error Loading Graph"</h3>
                                <p class="text-red-100 text-sm mb-4">{error}</p>
                            </div>
                        </div>
                    }.into_any()
                } else if nodes.get().is_empty() {
                    view! {
                        <div class="absolute inset-0 flex items-center justify-center pointer-events-none z-20">
                            <div class="text-center opacity-50">
                                <div class="text-6xl mb-4">"🚉"</div>
                                <p class="text-slate-400 text-lg mb-2">"The Train Yard is Empty"</p>
                                <p class="text-slate-500 text-sm">"Click 'Design with Pete' to start building your route"</p>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <span /> }.into_any()
                }
            }}

            // Property Editor
            {move || selected_node_id.get().and_then(|id| {
                nodes.get().iter().find(|n| n.get().id == id).cloned().map(|node_signal| {
                    view! {
                        <PropertyEditor
                            node=node_signal
                            on_close=move || set_selected_node_id.set(None)
                            on_delete=move || {
                                set_nodes.update(|n| n.retain(|x| x.get().id != id));
                                set_connections.update(|c| c.retain(|x| x.from_node != id && x.to_node != id));
                                set_selected_node_id.set(None);
                            }
                        />
                    }
                })
            })}

            // Template Selector Modal
            {move || if show_template_selector.get() {
                view! {
                    <TemplateSelector
                        on_close=move || set_show_template_selector.set(false)
                        on_generate={Callback::new(move |new_nodes: Vec<StoryNode>| {
                            // Add generated nodes to canvas
                            for node in new_nodes {
                                set_nodes.update(|n| n.push(RwSignal::new(node)));
                            }
                        })}
                    />
                }.into_any()
            } else {
                view! { <span /> }.into_any()
            }}
        </div>
    }
}
