use leptos::prelude::*;
use crate::bevy_app::run_bevy_app;

#[component]
pub fn BevyCanvas() -> impl IntoView {
    let canvas_id = "bevy-canvas";

    Effect::new(move |_| {
        // Call the Bevy entry point
        // Note: Bevy takes over the thread/loop, so this might block if not careful.
        // But Bevy 0.13+ on WASM usually uses requestAnimationFrame.
        // We just need to make sure we don't call it twice.
        // For now, we just call it.
        run_bevy_app(canvas_id.to_string());
    });

    view! {
        <div class="w-full h-screen overflow-hidden bg-black">
            <canvas id=canvas_id class="w-full h-full block focus:outline-none" oncontextmenu="return false;"></canvas>
        </div>
    }
}
