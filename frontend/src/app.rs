use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    *,
};

use crate::pages::ask_pete::AskPete;
use crate::pages::authoring::AuthoringPage;
use crate::pages::not_found::NotFound;
use crate::pages::research_dashboard::ResearchDashboard;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <crate::components::boilermaker::BoilermakerShell>
                <Routes fallback=|| "Not Found.">
                    <Route path=path!("/") view=AskPete/>
                    <Route path=path!("/chat") view=crate::components::ai_mirror_chat::AiMirrorChat/>
                    <Route path=path!("/authoring") view=AuthoringPage/>
                    <Route path=path!("/settings") view=crate::pages::settings::SettingsPage/>
                    // Hidden developer routes
                    <Route path=path!("/research") view=ResearchDashboard/>
                    <Route path=path!("/ai-mirror") view=crate::components::ai_mirror_chat::AiMirrorChat/>
                    <Route path=path!("/ai-test") view=crate::components::ai_test_panel::AiTestPanel/>
                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </crate::components::boilermaker::BoilermakerShell>
        </Router>
    }
}
