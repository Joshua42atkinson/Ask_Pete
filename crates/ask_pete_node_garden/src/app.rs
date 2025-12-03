use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    *,
};

use crate::pages::landing_page::LandingPage;
use crate::pages::not_found::NotFound;
use crate::ui_theme::provide_theme_context;

// Import Sub-App Pages
use ask_pete_client::pages::character_creation::CharacterCreationPage;
use ask_pete_client::pages::engine_cab_layout::EngineCabLayout;
use ask_pete_client::pages::play_mode::PlayMode;
use ask_pete_client::pages::student_dashboard::StudentDashboard;
use ask_pete_client::pages::weigh_station::WeighStation;

use crate::components::authoring::node_canvas::NodeCanvas;
use ask_pete_researcher::pages::knowledge_library::KnowledgeLibrary;
use ask_pete_researcher::pages::train_yard_layout::TrainYardLayout;

use ask_pete_researcher::pages::research_dashboard::ResearchDashboard;

#[component]
pub fn App() -> impl IntoView {
    provide_theme_context();

    // Global State (GPS, Auth, etc.) can be initialized here
    // For now, we rely on LocalStorage for Auth state in the router views

    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                // The Ticket Counter (Login)
                <Route path=path!("/") view=LandingPage />

                // The Locomotive Cab (Student)
                // TODO: Add Route Protection (Check Role)
                <ParentRoute path=path!("/cab") view=move || view! { <Outlet/> }>
                    <Route path=path!("") view=StudentDashboard />
                    <Route path=path!("journey/:id") view=PlayMode />
                    <Route path=path!("engine/:quest_id") view=EngineCabLayout />
                    <Route path=path!("weigh-station") view=WeighStation />
                    <Route path=path!("character-creation") view=CharacterCreationPage />
                </ParentRoute>

                // The Train Yard (Instructor)
                <ParentRoute path=path!("/yard") view=TrainYardLayout>
                    <Route path=path!("") view=move || view! { <NodeCanvas/> }/>
                    <Route path=path!("library") view=move || view! { <KnowledgeLibrary/> }/>
                </ParentRoute>

                // The Signal Tower (Researcher)
                <ParentRoute path=path!("/tower") view=move || view! { <Outlet/> }>
                    <Route path=path!("") view=ResearchDashboard />
                </ParentRoute>

                // Shared/Settings
                <Route path=path!("/settings") view=crate::pages::settings::SettingsPage/>
                <Route path=path!("/models") view=crate::components::model_manager::ModelManager/>
            </Routes>
        </Router>
    }
}
