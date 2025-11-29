use leptos::prelude::*;

#[component]
pub fn Toast(
    #[prop(into)] message: Signal<Option<String>>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    view! {
        {move || message.get().map(|msg| {
            view! {
                <div class="fixed bottom-4 right-4 z-50 animate-in slide-in-from-bottom-5 fade-in duration-300">
                    <div class="bg-slate-800 border border-boilermaker-gold text-white px-4 py-3 rounded shadow-lg flex items-center gap-3">
                        <span class="text-xl">"💾"</span>
                        <div>
                            <h4 class="font-bold text-sm text-boilermaker-gold">"System Notification"</h4>
                            <p class="text-sm">{msg}</p>
                        </div>
                        <button
                            class="ml-2 text-slate-400 hover:text-white transition-colors"
                            on:click=move |_| on_close.run(())
                        >
                            "✕"
                        </button>
                    </div>
                </div>
            }
        })}
    }
}
