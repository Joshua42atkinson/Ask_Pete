use leptos::prelude::*;

#[component]
pub fn VoiceInput(
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + Send + Sync>>,
) -> impl IntoView {
    let (is_listening, set_is_listening) = signal(false);
    let on_input = std::sync::Arc::new(on_input);
    
    // Mock listening effect
    Effect::new(move |_| {
        let on_input = on_input.clone();
        if is_listening.get() {
            // In a real app, this would start the Web Speech API
            // For now, we simulate a "listening" session that ends after 3 seconds
            leptos::task::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(3000).await;
                set_is_listening.set(false);
                if let Some(cb) = on_input.as_ref() {
                    cb("This is a simulated voice command.".to_string());
                }
            });
        }
    });

    view! {
        <div class="relative flex items-center justify-center">
            // Ripple Effect (only visible when listening)
            <div 
                class="absolute inset-0 bg-boilermaker-gold rounded-full opacity-20 animate-ping"
                class:hidden=move || !is_listening.get()
            ></div>
            
            <button
                class=move || {
                    let base = "relative z-10 p-4 rounded-full transition-all duration-300 shadow-lg flex items-center justify-center";
                    if is_listening.get() {
                        format!("{} bg-red-600 text-white hover:bg-red-700 scale-110", base)
                    } else {
                        format!("{} bg-slate-800 text-boilermaker-gold border border-boilermaker-gold/30 hover:bg-slate-700 hover:border-boilermaker-gold", base)
                    }
                }
                on:click=move |_| set_is_listening.update(|l| *l = !*l)
                title="Toggle Voice Input"
                id="voice-input-btn"
            >
                {move || if is_listening.get() {
                    view! { <span class="animate-pulse">"🎙️"</span> }.into_any()
                } else {
                    view! { <span>"🎙️"</span> }.into_any()
                }}
            </button>
            
            // Status Text
            <div 
                class="absolute -bottom-8 left-1/2 transform -translate-x-1/2 text-[10px] text-boilermaker-gold font-mono uppercase tracking-widest whitespace-nowrap transition-opacity duration-300"
                class:opacity-0=move || !is_listening.get()
                class:opacity-100=move || is_listening.get()
            >
                "Listening..."
            </div>
        </div>
    }
}
