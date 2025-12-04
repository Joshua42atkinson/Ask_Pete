use domain_trainyard::BlueprintStation;
use leptos::prelude::*;
use pete_core::trainyard::StoryGraph;

#[component]
fn App() -> impl IntoView {
    view! {
        <BlueprintStation
            on_close=Callback::new(|_| gloo_console::log!("Closed"))
            on_generate=Callback::new(|g: StoryGraph| gloo_console::log!(format!("Generated graph with {} nodes", g.nodes.len())))
        />
    }
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| view! { <App/> })
}
