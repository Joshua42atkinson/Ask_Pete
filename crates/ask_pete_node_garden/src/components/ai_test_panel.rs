// use crate::ai::gemma_client::Gemma1BClient;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn AiTestPanel() -> impl IntoView {
    let (output, set_output) = signal("Ready to load model...".to_string());
    let (is_loading, set_is_loading) = signal(false);
    let (prompt, set_prompt) = signal("Who are you?".to_string());

    let run_inference = move |_| {
        set_is_loading.set(true);
        set_output.set("Loading model (this may take a while)...".to_string());

        let p = prompt.get();

        spawn_local(async move {
            // 1. Initialize Client
            match pete_core::ai::gemma_client::Gemma1BClient::new().await {
                Ok(mut client) => {
                    set_output.set("Model loaded. Generating...".to_string());

                    // 2. Run Inference
                    match client.generate(&p, 50) {
                        Ok(response) => set_output.set(format!("Response: {}", response)),
                        Err(e) => set_output.set(format!("Generation Error: {}", e)),
                    }
                }
                Err(e) => set_output.set(format!("Load Error: {}", e)),
            }
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="p-4 border rounded-lg bg-gray-800 text-white shadow-lg max-w-md mx-auto mt-10">
            <h2 class="text-xl font-bold mb-4">"Gemma 1B (WASM) Test"</h2>

            <div class="mb-4">
                <label class="block text-sm font-medium mb-1">"Prompt"</label>
                <input
                    type="text"
                    class="w-full p-2 rounded bg-gray-700 border border-gray-600 focus:border-blue-500 outline-none"
                    prop:value=move || prompt.get()
                    on:input=move |ev| set_prompt.set(event_target_value(&ev))
                />
            </div>

            <button
                class="w-full py-2 px-4 bg-blue-600 hover:bg-blue-700 rounded font-bold transition disabled:opacity-50"
                on:click=run_inference
                disabled=move || is_loading.get()
            >
                {move || if is_loading.get() { "Processing..." } else { "Run Inference" }}
            </button>

            <div class="mt-4 p-3 bg-gray-900 rounded min-h-[100px] whitespace-pre-wrap font-mono text-sm">
                {move || output.get()}
            </div>

            <p class="mt-2 text-xs text-gray-400">
                "Note: First run requires downloading ~700MB. Check console for WebGPU status."
            </p>

            <crate::components::model_selector::ModelSelector />
        </div>
    }
}
