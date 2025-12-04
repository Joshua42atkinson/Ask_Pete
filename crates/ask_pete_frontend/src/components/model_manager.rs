use crate::components::model_card::{ModelCard, ModelMetadata, ModelStatus};
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn ModelManager() -> impl IntoView {
    // Mock Data for now - eventually this comes from a config or API
    let (models, _set_models) = signal(vec![
        ModelMetadata {
            id: "gemma-2b-it".to_string(),
            name: "Gemma 2B IT".to_string(),
            description: "Google's lightweight instruction-tuned model.".to_string(),
            size_mb: 1400.0,
            url: "https://huggingface.co/google/gemma-2b-it-GGUF/resolve/main/gemma-2b-it.q4_k_m.gguf".to_string(),
        },
        ModelMetadata {
            id: "tinyllama-1.1b".to_string(),
            name: "TinyLlama 1.1B".to_string(),
            description: "Compact model, good for older devices.".to_string(),
            size_mb: 600.0,
            url: "https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf".to_string(),
        },
    ]);

    // State
    let (model_statuses, set_model_statuses) = signal(HashMap::new());

    // Handlers
    let handle_download = Callback::new(move |id: String| {
        leptos::logging::log!("Downloading model: {}", id);
        // TODO: Hook up to pete_core::ai::gemma_client download logic
        // For now, simulate download
        set_model_statuses.update(|map| {
            map.insert(id.clone(), ModelStatus::Downloading(0.1));
        });

        // Simulate progress (mock)
        set_timeout(
            move || {
                set_model_statuses.update(|map| {
                    map.insert(id.clone(), ModelStatus::Ready);
                });
            },
            std::time::Duration::from_secs(2),
        );
    });

    let handle_activate = Callback::new(move |id: String| {
        leptos::logging::log!("Activating model: {}", id);
        set_model_statuses.update(|map| {
            // Reset others
            for val in map.values_mut() {
                if *val == ModelStatus::Active {
                    *val = ModelStatus::Ready;
                }
            }
            map.insert(id, ModelStatus::Active);
        });
    });

    view! {
        <div class="p-6 bg-gray-900 min-h-screen text-white">
            <header class="mb-8">
                <h1 class="text-3xl font-bold mb-2">"Model Manager"</h1>
                <p class="text-gray-400">"Manage your local AI models. Downloaded models run entirely on your device."</p>
            </header>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <For
                    each=move || models.get()
                    key=|model| model.id.clone()
                    children=move |model| {
                        let id = model.id.clone();
                        let status = move || {
                            model_statuses.get().get(&id).cloned().unwrap_or(ModelStatus::NotDownloaded)
                        };

                        view! {
                            <ModelCard
                                model=model
                                status=status()
                                on_download=handle_download
                                on_activate=handle_activate
                            />
                        }
                    }
                />
            </div>
        </div>
    }
}
