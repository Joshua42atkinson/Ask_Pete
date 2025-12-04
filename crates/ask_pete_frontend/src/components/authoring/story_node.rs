use common::expert::StoryNode;
use leptos::prelude::*;

#[component]
pub fn StoryNodeComponent(
    #[prop(into)] node: RwSignal<StoryNode>,
    #[prop(into)] on_mousedown: Callback<web_sys::MouseEvent>,
    #[prop(into)] on_port_mousedown: Callback<(String, String)>, // (node_id, "input" | "output")
    #[prop(into)] on_port_mouseup: Callback<(String, String)>,   // (node_id, "input" | "output")
) -> impl IntoView {
    // We subscribe to the signal to get updates
    let node_data = move || node.get();

    // Cognitive Load Logic
    let passenger_status = move || {
        let count = node_data().passenger_count;
        if count <= 4 {
            ("text-signal-green", "border-signal-green", "Safe")
        } else if count == 5 {
            ("text-signal-yellow", "border-signal-yellow", "At Capacity")
        } else {
            ("text-signal-red", "border-signal-red", "Overload!")
        }
    };

    view! {
        <div
            class="absolute station-node w-72 rounded-lg cursor-move select-none transition-transform"
            style=move || format!("left: {}px; top: {}px;", node_data().x, node_data().y)
            on:mousedown=move |ev| on_mousedown.run(ev)
        >
            // Station Roof / Header
            <div class="bg-slate-900/90 p-3 border-b-2 border-track-gray flex justify-between items-start rounded-t-lg">
                <div class="flex flex-col overflow-hidden">
                    <div class="flex items-center gap-2 text-xs font-bold text-slate-400 uppercase tracking-wider">
                        <span>"🚉 Station"</span>
                        <span class="text-[10px] bg-slate-800 px-1 rounded border border-slate-700">
                            "ID: " {move || node_data().id.chars().take(4).collect::<String>()}
                        </span>
                    </div>
                    <span class="font-bold text-white text-base truncate mt-1 font-mono text-shadow-sm">
                        {move || node_data().title}
                    </span>
                </div>

                // Cognitive Load Indicator (Passenger Count)
                <div class="flex flex-col items-end">
                    <div
                        class=move || format!("flex items-center gap-1 px-2 py-1 rounded bg-slate-950 border {}", passenger_status().1)
                        title=move || format!("Cognitive Load: {}", passenger_status().2)
                    >
                        <span class="text-xs">"📦"</span>
                        <span class=move || format!("font-mono font-bold {}", passenger_status().0)>
                            {move || node_data().passenger_count} "/4"
                        </span>
                    </div>
                </div>
            </div>

            // Content Area (Waiting Room)
            <div class="p-4 bg-railyard-dark/90 text-slate-300 text-sm font-mono min-h-[80px] relative">
                <div class="line-clamp-3">
                    {move || if node_data().content.is_empty() {
                        view! { <span class="italic text-slate-600">"Empty station..."</span> }.into_any()
                    } else {
                        view! { <span>{node_data().content}</span> }.into_any()
                    }}
                </div>

                // Platform Footer
                <div class="station-platform mt-3 pt-2 flex justify-between items-center text-[10px] text-slate-500 uppercase tracking-widest">
                    <span>"Platform 1"</span>
                    <span>"Zone " {move || node_data().complexity_level}</span>
                </div>
            </div>

            // Input Port (Westbound Track)
            <div
                class="absolute -left-3 top-1/2 -translate-y-1/2 w-6 h-6 bg-slate-900 border-2 border-track-gray rounded-full hover:border-boilermaker-gold hover:scale-110 transition-all cursor-crosshair flex items-center justify-center shadow-lg z-10"
                on:mousedown=move |ev| { ev.stop_propagation(); on_port_mousedown.run((node_data().id.clone(), "input".to_string())); }
                on:mouseup=move |ev| { ev.stop_propagation(); on_port_mouseup.run((node_data().id.clone(), "input".to_string())); }
                title="Arriving Track"
            >
                <div class="w-2 h-2 bg-signal-green rounded-full animate-pulse"></div>
            </div>

            // Output Port (Eastbound Track)
            <div
                class="absolute -right-3 top-1/2 -translate-y-1/2 w-6 h-6 bg-slate-900 border-2 border-track-gray rounded-full hover:border-boilermaker-gold hover:scale-110 transition-all cursor-crosshair flex items-center justify-center shadow-lg z-10"
                on:mousedown=move |ev| { ev.stop_propagation(); on_port_mousedown.run((node_data().id.clone(), "output".to_string())); }
                on:mouseup=move |ev| { ev.stop_propagation(); on_port_mouseup.run((node_data().id.clone(), "output".to_string())); }
                title="Departing Track"
            >
                <div class="w-2 h-2 bg-signal-yellow rounded-full"></div>
            </div>
        </div>
    }
}
