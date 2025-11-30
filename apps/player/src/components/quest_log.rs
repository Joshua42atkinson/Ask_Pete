use leptos::prelude::*;
use pete_core::PlayerCharacter;

#[component]
pub fn QuestLog() -> impl IntoView {
    // Poll for updates every 2 seconds to keep the UI fresh
    let (trigger, set_trigger) = signal(0);
    
    Effect::new(move |_| {
        let handle = set_interval_with_handle(
            move || set_trigger.update(|n| *n += 1),
            std::time::Duration::from_secs(2),
        ).ok();
        
        on_cleanup(move || {
            if let Some(h) = handle {
                h.clear();
            }
        });
    });

    let character_resource = LocalResource::new(move || async move {
        trigger.get(); // Depend on trigger to re-fetch
        match gloo_net::http::Request::get("/api/player_character").send().await {
            Ok(resp) => resp.json::<PlayerCharacter>().await.ok(),
            Err(_) => None,
        }
    });

    view! {
        <div class="fixed top-4 left-4 bg-slate-900/95 border-2 border-boilermaker-gold/50 rounded-lg p-4 shadow-[0_0_15px_rgba(206,184,136,0.1)] z-50 w-80 backdrop-blur-md transition-all duration-300 hover:bg-slate-900 hover:border-boilermaker-gold hover:shadow-[0_0_25px_rgba(206,184,136,0.2)] group">
            <h3 class="text-boilermaker-gold font-bold text-sm mb-2 uppercase tracking-wider border-b border-white/10 pb-1 flex justify-between items-center">
                <span>"Current Objective"</span>
                <span class="text-[10px] text-slate-500">"SYNCED"</span>
            </h3>
            <Suspense fallback=|| view! { <div class="animate-pulse h-12 bg-slate-800 rounded"></div> }>
                {move || {
                    match character_resource.get() {
                        Some(Some(char)) => view! {
                            <div class="animate-fade-in">
                                <h4 class="text-white font-bold text-sm mb-1 text-shadow-sm">{char.current_quest_title}</h4>
                                <p class="text-xs text-slate-300 leading-relaxed border-l-2 border-slate-700 pl-2 italic">
                                    {char.current_step_description}
                                </p>
                                
                                // Optional: Show Inventory count or other quick stats
                                <div class="mt-3 flex gap-2 text-[10px] text-slate-400">
                                    <span>{format!("Inventory: {}", char.inventory.len())}</span>
                                    <span>"•"</span>
                                    <span>{format!("Fate: {}", char.fate_points)}</span>
                                </div>
                            </div>
                        }.into_any(),
                        _ => view! { 
                            <div class="text-xs text-slate-500 italic">
                                "Waiting for neural link..."
                            </div> 
                        }.into_any(),
                    }
                }}
            </Suspense>
        </div>
    }
}
