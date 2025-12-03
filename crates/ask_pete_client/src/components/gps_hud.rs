use leptos::prelude::ClassAttribute;
use leptos::prelude::*;

#[component]
pub fn GpsHud(
    coal: ReadSignal<f32>,
    steam: ReadSignal<f32>,
    latitude: ReadSignal<f64>,
    longitude: ReadSignal<f64>,
    current_zone: ReadSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <div class="fixed bottom-4 right-4 bg-slate-900/90 border border-boilermaker-gold rounded-lg p-4 shadow-xl z-50 w-64 backdrop-blur-md">
            <h3 class="text-boilermaker-gold font-bold text-sm mb-2 uppercase tracking-wider border-b border-white/10 pb-1">
                "Locomotive Telemetry"
            </h3>

            // Zone Indicator
            <div class="mb-3">
                 <div class="flex justify-between text-xs text-slate-400 mb-1">
                    <span>"CURRENT ZONE"</span>
                </div>
                <div class="p-2 rounded bg-slate-800 border border-slate-700 text-center">
                    {move || match current_zone.get() {
                        Some(zone) => view! { <span class="text-green-400 font-bold animate-pulse">{zone}</span> }.into_any(),
                        None => view! { <span class="text-slate-500 italic">"In Transit..."</span> }.into_any(),
                    }}
                </div>
            </div>

            // Coal Gauge
            <div class="mb-3">
                <div class="flex justify-between text-xs text-slate-400 mb-1">
                    <span>"COAL (Attention)"</span>
                    <span>{move || format!("{:.0}%", coal.get())}</span>
                </div>
                <div class="w-full bg-slate-800 h-2 rounded-full overflow-hidden">
                    <div
                        class="h-full bg-gradient-to-r from-orange-600 to-red-500 transition-all duration-500"
                        style:width=move || format!("{}%", coal.get())
                    ></div>
                </div>
            </div>

            // Steam Gauge
            <div class="mb-3">
                <div class="flex justify-between text-xs text-slate-400 mb-1">
                    <span>"STEAM (Mastery)"</span>
                    <span>{move || format!("{:.0} PSI", steam.get())}</span>
                </div>
                <div class="w-full bg-slate-800 h-2 rounded-full overflow-hidden">
                    <div
                        class="h-full bg-gradient-to-r from-cyan-600 to-blue-400 transition-all duration-500"
                        style:width=move || format!("{}%", (steam.get() / 100.0).min(1.0) * 100.0)
                    ></div>
                </div>
            </div>

            // GPS Coordinates
            <div class="text-xs font-mono text-emerald-400 bg-black/40 p-2 rounded border border-emerald-900/50">
                <div class="flex justify-between">
                    <span>"LAT:"</span>
                    <span>{move || format!("{:.6}", latitude.get())}</span>
                </div>
                <div class="flex justify-between">
                    <span>"LON:"</span>
                    <span>{move || format!("{:.6}", longitude.get())}</span>
                </div>
            </div>
        </div>
    }
}
