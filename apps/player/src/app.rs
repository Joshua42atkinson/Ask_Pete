use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    *,
};

use crate::pages::not_found::NotFound;

// Train Yard Architecture
use crate::pages::engine_cab_layout::EngineCabLayout;
use crate::pages::weigh_station::WeighStation;

use crate::ui_theme::provide_theme_context;

#[component]
pub fn App() -> impl IntoView {
    provide_theme_context();

    // [TRIGGER REBUILD]
    // [NEW] Global Telemetry State
    let (coal, set_coal) = signal(100.0f32);
    let (steam, set_steam) = signal(0.0f32);
    let (latitude, set_latitude) = signal(0.0f64);
    let (longitude, set_longitude) = signal(0.0f64);
    let (current_zone, set_current_zone) = signal::<Option<String>>(None);

    // [NEW] DevTools State
    let (is_manual_location, set_is_manual_location) = signal(false);
    let (manual_lat, set_manual_lat) = signal(40.4282);
    let (manual_lon, set_manual_lon) = signal(-86.9144);
    let (show_devtools, set_show_devtools) = signal(false);

    // [NEW] Geolocation Logic
    Effect::new(move |_| {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();
            if let Ok(geolocation) = navigator.geolocation() {
                let on_success = Closure::wrap(Box::new(move |position: web_sys::Position| {
                    if is_manual_location.get() {
                        return;
                    }
                    let coords = position.coords();
                    let lat = coords.latitude();
                    let lon = coords.longitude();

                    set_latitude.set(lat);
                    set_longitude.set(lon);
                    set_steam.update(|s| *s += 0.1);
                    set_coal.update(|c| *c = (*c - 0.05f32).max(0.0f32));

                    // Mock Zone Detection (Simulating Backend Check)
                    // In real implementation, this would call VaamService::check_unlock_status
                    if lat > 40.0 && lat < 41.0 {
                        set_current_zone.set(Some("The Bell Tower".to_string()));
                    } else {
                        set_current_zone.set(None);
                    }
                }) as Box<dyn FnMut(_)>);

                let on_error = Closure::wrap(Box::new(move |error: web_sys::PositionError| {
                    gloo_console::error!(format!("Geolocation error: {:?}", error.message()));
                }) as Box<dyn FnMut(_)>);

                let options = web_sys::PositionOptions::new();
                options.set_enable_high_accuracy(true);

                let _ = geolocation.watch_position_with_error_callback_and_options(
                    on_success.as_ref().unchecked_ref(),
                    Some(on_error.as_ref().unchecked_ref()),
                    &options,
                );

                on_success.forget();
                on_error.forget();
            }
        }
    });

    // Manual Location Effect
    Effect::new(move |_| {
        if is_manual_location.get() {
            let lat = manual_lat.get();
            let lon = manual_lon.get();
            set_latitude.set(lat);
            set_longitude.set(lon);

            // Mock Zone Detection for Manual Mode
            if lat > 40.42 && lat < 40.43 {
                set_current_zone.set(Some("The Bell Tower".to_string()));
            } else {
                set_current_zone.set(None);
            }
        }
    });

    // === PLAYER UI ===
    view! {
        <>
            <crate::components::gps_hud::GpsHud
                coal=coal
                steam=steam
                latitude=latitude
                longitude=longitude
                current_zone=current_zone
            />

            <crate::components::quest_log::QuestLog />

            // DevTools Toggle
            <div class="fixed top-4 right-4 z-[100]">
                <button
                    id="devtools-toggle"
                    class="bg-slate-800 text-white p-2 rounded-full shadow-lg hover:bg-slate-700 transition-colors"
                    on:click=move |_| set_show_devtools.update(|v| *v = !*v)
                    title="Open Developer Tools"
                >
                    "🛠️"
                </button>
            </div>

            // DevTools Panel
            {move || if show_devtools.get() {
                view! {
                    <div class="fixed top-16 right-4 bg-slate-900/95 border border-slate-700 p-4 rounded-lg shadow-xl z-50 w-64 text-xs text-slate-300">
                        <h4 class="font-bold text-white mb-2 border-b border-slate-700 pb-1">"DEV TOOLS"</h4>

                        <div class="mb-3">
                            <label class="flex items-center space-x-2 cursor-pointer">
                                <input
                                    type="checkbox"
                                    prop:checked=move || is_manual_location.get()
                                    on:change=move |ev| set_is_manual_location.set(event_target_checked(&ev))
                                    class="form-checkbox h-4 w-4 text-blue-600 bg-slate-800 border-slate-600 rounded focus:ring-blue-500"
                                />
                                <span>"Manual Location Override"</span>
                            </label>
                        </div>

                        <div class="space-y-2 opacity-50 transition-opacity" class:opacity-100=move || is_manual_location.get()>
                            <div>
                                <label class="block text-slate-500 mb-1">"Latitude"</label>
                                <input
                                    type="number"
                                    step="0.0001"
                                    prop:value=move || manual_lat.get()
                                    on:input=move |ev| set_manual_lat.set(event_target_value(&ev).parse().unwrap_or(0.0))
                                    class="w-full bg-slate-800 border border-slate-700 rounded px-2 py-1 text-white focus:border-blue-500 outline-none"
                                    disabled=move || !is_manual_location.get()
                                />
                            </div>
                            <div>
                                <label class="block text-slate-500 mb-1">"Longitude"</label>
                                <input
                                    type="number"
                                    step="0.0001"
                                    prop:value=move || manual_lon.get()
                                    on:input=move |ev| set_manual_lon.set(event_target_value(&ev).parse().unwrap_or(0.0))
                                    class="w-full bg-slate-800 border border-slate-700 rounded px-2 py-1 text-white focus:border-blue-500 outline-none"
                                    disabled=move || !is_manual_location.get()
                                />
                            </div>

                            <div class="pt-2 border-t border-slate-700 mt-2">
                                <p class="text-slate-500 mb-2">"Teleport Presets"</p>
                                <div class="grid grid-cols-1 gap-2">
                                    <button
                                        class="bg-blue-900/50 hover:bg-blue-800 text-blue-200 py-1 px-2 rounded border border-blue-800 transition-colors text-left"
                                        on:click=move |_| {
                                            set_manual_lat.set(40.4282);
                                            set_manual_lon.set(-86.9144);
                                        }
                                        disabled=move || !is_manual_location.get()
                                    >
                                        "📍 Engineering Fountain"
                                    </button>
                                    <button
                                        class="bg-amber-900/50 hover:bg-amber-800 text-amber-200 py-1 px-2 rounded border border-amber-800 transition-colors text-left"
                                        on:click=move |_| {
                                            set_manual_lat.set(40.4270); // Approx Bell Tower
                                            set_manual_lon.set(-86.9140);
                                        }
                                        disabled=move || !is_manual_location.get()
                                    >
                                        "📍 The Bell Tower"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span /> }.into_any()
            }}

            <Router>
                <Routes fallback=|| "Not Found.">
                    <Route path=path!("/") view=crate::pages::student_dashboard::StudentDashboard/>
                    <Route path=path!("/journey/:id") view=crate::pages::play_mode::PlayMode/>
                    <Route path=path!("/cab/:quest_id") view=EngineCabLayout/>
                    <Route path=path!("/weigh-station") view=WeighStation/>
                    <Route path=path!("/character-creation") view=crate::pages::character_creation::CharacterCreationPage/>
                    <Route path=path!("/settings") view=crate::pages::settings::SettingsPage/>
                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </Router>
        </>
    }
}
