use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    *,
};

use crate::pages::not_found::NotFound;

// Train Yard Architecture
use crate::pages::knowledge_library::KnowledgeLibrary;
use crate::pages::train_yard_layout::TrainYardLayout;

use crate::ui_theme::provide_theme_context;

#[component]
pub fn App() -> impl IntoView {
    provide_theme_context();

    // === TEACHER UI ===
    view! {
        <Router>
            <Routes fallback=|| "Not Found.">
                <ParentRoute path=path!("/") view=TrainYardLayout>
                    <Route path=path!("") view=move || view! { <domain_trainyard::authoring::node_canvas::NodeCanvas/> }/>
                    <Route path=path!("library") view=move || view! { <KnowledgeLibrary/> }/>
                </ParentRoute>
                <Route path=path!("/settings") view=crate::pages::settings::SettingsPage/>
                <Route path=path!("/*any") view=NotFound/>
            </Routes>
        </Router>
    }
}
