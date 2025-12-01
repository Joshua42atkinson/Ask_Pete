use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct WordDefinition {
    pub word: String,
    pub definition: String,
    pub power: String, // e.g., "Unlock Speed Gate"
}

#[component]
pub fn WordSmithy(on_close: Callback<()>, on_save: Callback<WordDefinition>) -> impl IntoView {
    let (word, set_word) = signal("".to_string());
    let (definition, set_definition) = signal("".to_string());
    let (power, set_power) = signal("".to_string());

    let save_handler = move |_| {
        let new_word = WordDefinition {
            word: word.get(),
            definition: definition.get(),
            power: power.get(),
        };
        on_save.run(new_word);
        on_close.run(());
    };

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm">
            <div class="bg-slate-900 border-2 border-boilermaker-gold rounded-lg w-[500px] shadow-2xl flex flex-col">
                // Header
                <div class="p-4 border-b border-white/10 flex justify-between items-center bg-slate-950">
                    <h2 class="text-xl font-bold text-boilermaker-gold flex items-center gap-2">
                        <span>"⚒️"</span>
                        "The Word Smithy"
                    </h2>
                    <button
                        class="text-slate-400 hover:text-white transition-colors"
                        on:click=move |_| on_close.run(())
                    >
                        "✕"
                    </button>
                </div>

                // Body
                <div class="p-6 space-y-4">
                    // Word
                    <div class="space-y-1">
                        <label class="block text-xs font-bold text-purdue-gold uppercase">"Word"</label>
                        <input
                            type="text"
                            class="w-full bg-black/40 border border-slate-600 rounded p-2 text-white focus:border-boilermaker-gold outline-none"
                            placeholder="e.g. Friction"
                            prop:value=move || word.get()
                            on:input=move |ev| set_word.set(event_target_value(&ev))
                        />
                    </div>

                    // Definition
                    <div class="space-y-1">
                        <label class="block text-xs font-bold text-purdue-gold uppercase">"Definition"</label>
                        <textarea
                            class="w-full h-20 bg-black/40 border border-slate-600 rounded p-2 text-white focus:border-boilermaker-gold outline-none resize-none"
                            placeholder="The resistance that one surface or object encounters when moving over another."
                            prop:value=move || definition.get()
                            on:input=move |ev| set_definition.set(event_target_value(&ev))
                        />
                    </div>

                    // Power (Effect)
                    <div class="space-y-1">
                        <label class="block text-xs font-bold text-purdue-gold uppercase">"Power (Effect)"</label>
                        <input
                            type="text"
                            class="w-full bg-black/40 border border-slate-600 rounded p-2 text-white focus:border-boilermaker-gold outline-none"
                            placeholder="e.g. Slows movement by 50%"
                            prop:value=move || power.get()
                            on:input=move |ev| set_power.set(event_target_value(&ev))
                        />
                        <p class="text-xs text-slate-500">"Define what happens when this word is invoked in the world."</p>
                    </div>
                </div>

                // Footer
                <div class="p-4 border-t border-white/10 bg-slate-950 flex justify-end gap-2">
                    <button
                        class="px-3 py-1 text-slate-300 hover:text-white transition-colors"
                        on:click=move |_| on_close.run(())
                    >
                        "Cancel"
                    </button>
                    <button
                        class="px-4 py-1 bg-boilermaker-gold hover:bg-white text-black font-bold rounded shadow transition-all disabled:opacity-50"
                        disabled=move || word.get().is_empty()
                        on:click=save_handler
                    >
                        "Forge Word"
                    </button>
                </div>
            </div>
        </div>
    }
}
