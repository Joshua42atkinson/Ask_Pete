use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ModelMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub size_mb: f64,
    pub url: String,
}

#[derive(Clone, PartialEq)]
pub enum ModelStatus {
    NotDownloaded,
    Downloading(f64), // Progress 0.0 - 1.0
    Ready,
    Active,
}

#[component]
pub fn ModelCard(
    model: ModelMetadata,
    status: ModelStatus,
    #[prop(into)] on_download: Callback<String>,
    #[prop(into)] on_activate: Callback<String>,
) -> impl IntoView {
    let status_display = match status {
        ModelStatus::NotDownloaded => view! { <span class="text-gray-500">"Not Downloaded"</span> }.into_any(),
        ModelStatus::Downloading(progress) => view! {
            <div class="w-full bg-gray-700 rounded-full h-2.5 dark:bg-gray-700">
                <div class="bg-blue-600 h-2.5 rounded-full" style=format!("width: {:.0}%", progress * 100.0)></div>
            </div>
            <span class="text-sm text-blue-400">{format!("{:.0}%", progress * 100.0)}</span>
        }.into_any(),
        ModelStatus::Ready => view! { <span class="text-green-500">"Ready"</span> }.into_any(),
        ModelStatus::Active => view! { <span class="text-yellow-400 font-bold">"ACTIVE"</span> }.into_any(),
    };

    let action_button = match status {
        ModelStatus::NotDownloaded => view! {
            <button
                class="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded text-white font-bold transition-colors"
                on:click=move |_| on_download.run(model.id.clone())
            >
                "Download"
            </button>
        }.into_any(),
        ModelStatus::Ready => view! {
            <button
                class="px-4 py-2 bg-green-600 hover:bg-green-700 rounded text-white font-bold transition-colors"
                on:click=move |_| on_activate.run(model.id.clone())
            >
                "Activate"
            </button>
        }.into_any(),
        ModelStatus::Active => view! {
            <button class="px-4 py-2 bg-gray-600 rounded text-gray-300 cursor-not-allowed" disabled>
                "Active"
            </button>
        }.into_any(),
        ModelStatus::Downloading(_) => view! {
            <button class="px-4 py-2 bg-gray-600 rounded text-gray-300 cursor-not-allowed" disabled>
                "Downloading..."
            </button>
        }.into_any(),
    };

    view! {
        <div class="p-4 bg-gray-800 rounded-lg border border-gray-700 shadow-md flex flex-col gap-3">
            <div class="flex justify-between items-start">
                <div>
                    <h3 class="text-xl font-bold text-white">{model.name}</h3>
                    <p class="text-sm text-gray-400">{model.description}</p>
                </div>
                <div class="text-xs font-mono text-gray-500 bg-gray-900 px-2 py-1 rounded">
                    {format!("{:.1} MB", model.size_mb)}
                </div>
            </div>

            <div class="mt-auto flex items-center justify-between">
                <div class="flex-1 mr-4">
                    {status_display}
                </div>
                {action_button}
            </div>
        </div>
    }
}
