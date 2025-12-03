use leptos::prelude::*;

#[component]
pub fn Tooltip(text: String, children: Children) -> impl IntoView {
    view! {
        <div class="group relative inline-block">
            {children()}
            <div class="pointer-events-none absolute bottom-full left-1/2 -translate-x-1/2 mb-2 w-max max-w-xs rounded bg-slate-900 px-2 py-1 text-xs text-white opacity-0 shadow transition-opacity group-hover:opacity-100 z-50">
                {text}
                <div class="absolute top-full left-1/2 -translate-x-1/2 border-4 border-transparent border-t-slate-900"></div>
            </div>
        </div>
    }
}

#[component]
pub fn Toast(message: ReadSignal<Option<String>>, on_close: Callback<()>) -> impl IntoView {
    view! {
        {move || message.get().map(|msg| view! {
            <div class="fixed bottom-4 right-4 bg-slate-800 text-white p-4 rounded shadow-lg border border-slate-600 z-50 animate-fade-in">
                <span>{msg}</span>
                <button class="ml-4 text-slate-400 hover:text-white" on:click=move |_| on_close.run(())>"✕"</button>
            </div>
        })}
    }
}
