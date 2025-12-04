use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    *,
};

use crate::pages::ask_pete::AskPete;
use crate::pages::authoring::AuthoringPage;
use crate::pages::not_found::NotFound;
use crate::pages::research_dashboard::ResearchDashboard;

// Train Yard Architecture
use crate::pages::engine_cab_layout::EngineCabLayout;
use crate::pages::knowledge_library::KnowledgeLibrary;
use crate::pages::train_yard_layout::TrainYardLayout;
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

    // [NEW] Geolocation Logic
    Effect::new(move |_| {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();
            if let Ok(geolocation) = navigator.geolocation() {
                let on_success = Closure::wrap(Box::new(move |position: web_sys::Position| {
                    let coords = position.coords();
                    set_latitude.set(coords.latitude());
                    set_longitude.set(coords.longitude());
                    set_steam.update(|s| *s += 0.1);
                    set_coal.update(|c| *c = (*c - 0.05f32).max(0.0f32));
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

    // Detect Port for Audience Separation
    let port = web_sys::window()
        .and_then(|w| w.location().port().ok())
        .unwrap_or_default();

    if port == "8081" {
        // === RESEARCHER UI ===
        view! {
            <Router>
                <Routes fallback=|| "Not Found.">
                    <Route path=path!("/") view=ResearchDashboard/>
                    <Route path=path!("/settings") view=crate::pages::settings::SettingsPage/>
                    <Route path=path!("/ai-test") view=crate::components::ai_test_panel::AiTestPanel/>
                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </Router>
        }.into_any()
    } else if port == "8082" {
        // === TEACHER UI ===
        view! {
            <Router>
                <Routes fallback=|| "Not Found.">
                    <ParentRoute path=path!("/") view=TrainYardLayout>
                        <Route path=path!("") view=move || view! { <crate::components::authoring::node_canvas::NodeCanvas/> }/>
                        <Route path=path!("library") view=move || view! { <KnowledgeLibrary/> }/>
                    </ParentRoute>
                    <Route path=path!("/settings") view=crate::pages::settings::SettingsPage/>
                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </Router>
        }.into_any()
    } else if port == "8083" {
        // === PLAYER UI ===
        view! {
            <>
                <crate::components::gps_hud::GpsHud
                    coal=coal
                    steam=steam
                    latitude=latitude
                    longitude=longitude
                />
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
        }.into_any()
    } else {
        // === DEFAULT / DEV MODE ===
        view! {
             <>
                <crate::components::gps_hud::GpsHud
                    coal=coal
                    steam=steam
                    latitude=latitude
                    longitude=longitude
                />
                <Router>
                    <Routes fallback=|| "Not Found.">
                        <Route path=path!("/") view=AskPete/>
                        
                        // Dev Mode Access to All
                        <Route path=path!("/research") view=ResearchDashboard/>
                        
                        <ParentRoute path=path!("/yard") view=TrainYardLayout>
                             <Route path=path!("") view=move || view! { <crate::components::authoring::node_canvas::NodeCanvas/> }/>
                             <Route path=path!("library") view=move || view! { <KnowledgeLibrary/> }/>
                        </ParentRoute>

                        <Route path=path!("/play") view=crate::pages::student_dashboard::StudentDashboard/>
                        <Route path=path!("/journey/:id") view=crate::pages::play_mode::PlayMode/>
                        <Route path=path!("/cab/:quest_id") view=EngineCabLayout/>
                        <Route path=path!("/weigh-station") view=WeighStation/>
                        <Route path=path!("/character-creation") view=crate::pages::character_creation::CharacterCreationPage/>
                        
                        <Route path=path!("/chat") view=crate::components::ai_mirror_chat::AiMirrorChat/>
                        <Route path=path!("/authoring") view=AuthoringPage/>
                        <Route path=path!("/settings") view=crate::pages::settings::SettingsPage/>
                        <Route path=path!("/ai-mirror") view=crate::components::ai_mirror_chat::AiMirrorChat/>
                        <Route path=path!("/ai-test") view=crate::components::ai_test_panel::AiTestPanel/>

                        <Route path=path!("/*any") view=NotFound/>
                    </Routes>
                </Router>
            </>
        }.into_any()
    }
}
