use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    *,
};

use crate::pages::not_found::NotFound;
use crate::pages::research_dashboard::ResearchDashboard;

use crate::ui_theme::provide_theme_context;

#[component]
pub fn App() -> impl IntoView {
    provide_theme_context();

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
    }
}
