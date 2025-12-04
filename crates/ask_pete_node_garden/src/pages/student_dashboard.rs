use crate::ui_theme::{
    blueprint_grid_background, chamfered_panel_classes, mechanical_button_classes,
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Scenario {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub status: String, // "Locked", "Available", "In Progress", "Completed"
    pub tags: Vec<String>,
}

async fn fetch_scenarios() -> Vec<Scenario> {
    let resp = match gloo_net::http::Request::get("/api/scenarios").send().await {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to fetch scenarios: {}", e);
            return vec![];
        }
    };

    match resp.json().await {
        Ok(scenarios) => scenarios,
        Err(e) => {
            log::error!("Failed to parse scenarios: {}", e);
            vec![]
        }
    }
}

/// The "Train Yards" - Student Dashboard for selecting scenarios
#[component]
pub fn StudentDashboard() -> impl IntoView {
    let scenarios_resource = LocalResource::new(|| fetch_scenarios());

    view! {
        <div class="min-h-screen bg-[#121212] text-[#F0F0F0] font-ui relative overflow-hidden">
            // Background
            <div class={blueprint_grid_background()}></div>

            <div class="relative z-10 container mx-auto px-6 py-12">
                <header class="mb-12 text-center">
                    <h1 class="text-5xl font-bold text-[#CEB888] mb-4 tracking-tight">"TRAIN YARDS"</h1>
                    <p class="text-xl text-slate-400 max-w-2xl mx-auto">
                        "Select a scenario to begin your journey. Manage your cognitive load and master the tracks."
                    </p>
                </header>

                <Suspense fallback=move || view! { <p class="text-center text-[#CEB888]">"Loading Schedules..."</p> }>
                    {move || {
                        scenarios_resource.get().map(|scenarios| {
                            if scenarios.is_empty() {
                                view! {
                                    <div class="text-center text-slate-500">
                                        "No scenarios available. The tracks are silent."
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                                        {scenarios.into_iter().map(|scenario| {
                                            let status_color = match scenario.status.as_str() {
                                                "Locked" => "text-slate-600",
                                                "Available" => "text-[#CEB888]",
                                                "In Progress" => "text-blue-400",
                                                "Completed" => "text-[#4F7942]",
                                                _ => "text-slate-400",
                                            };

                                            let is_locked = scenario.status == "Locked";

                                            view! {
                                                <div class={format!("{} flex flex-col h-full transition-transform hover:-translate-y-1", chamfered_panel_classes(true))}>
                                                    <div class="flex justify-between items-start mb-4">
                                                        <span class="text-xs font-mono border border-current px-2 py-1 rounded opacity-70">
                                                            {scenario.difficulty}
                                                        </span>
                                                        <span class={format!("text-xs font-bold uppercase tracking-wider {}", status_color)}>
                                                            {scenario.status.clone()}
                                                        </span>
                                                    </div>

                                                    <h3 class="text-2xl font-bold mb-3 text-white">{scenario.title}</h3>
                                                    <p class="text-slate-400 mb-6 flex-grow leading-relaxed">
                                                        {scenario.description}
                                                    </p>

                                                    // Tags
                                                    <div class="flex flex-wrap gap-2 mb-4">
                                                        {scenario.tags.iter().map(|tag| {
                                                            view! { <span class="text-[10px] bg-slate-800 px-2 py-1 rounded text-slate-300">{tag.clone()}</span> }
                                                        }).collect::<Vec<_>>()}
                                                    </div>

                                                    <div class="mt-auto">
                                                        {if is_locked {
                                                            view! {
                                                                <button class="w-full py-3 border border-slate-700 text-slate-600 cursor-not-allowed font-mono uppercase tracking-widest">
                                                                    "Access Denied"
                                                                </button>
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <a
                                                                    href={format!("/journey/{}", scenario.id)}
                                                                    class={format!("block text-center {}", mechanical_button_classes("primary"))}
                                                                >
                                                                    "Depart Station"
                                                                </a>
                                                            }.into_any()
                                                        }}
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}
