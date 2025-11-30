use leptos::ev::MouseEvent;
use leptos::prelude::*;

// --- Core Layout Components ---

#[component]
pub fn BoilermakerShell(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-industrial-slate text-steam-white font-ui selection:bg-purdue-prime selection:text-black overflow-x-hidden">
            // Header
            <header class="sticky top-0 z-50 bg-industrial-surface/95 backdrop-blur-md border-b border-purdue-gold/30 shadow-lg">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <div class="w-10 h-10 bg-purdue-prime text-black rounded-full flex items-center justify-center font-black text-xl border-2 border-white/10 shadow-inner">
                            "P"
                        </div>
                        <div>
                            <h1 class="text-xl font-black text-purdue-prime tracking-wider uppercase leading-none">
                                "ASK PETE"
                            </h1>
                            <p class="text-[10px] text-purdue-gold font-mono tracking-[0.2em] uppercase opacity-80">
                                "Purdue Expert Tuition Engine"
                            </p>
                        </div>
                    </div>

                    // Navigation
                    <nav class="hidden md:flex items-center gap-1">
                        <NavButton href="/yard" label="Dashboard" />
                        <NavButton href="/yard/chat" label="Comms" />
                        <NavButton href="/yard/authoring" label="Forge" />
                        <NavButton href="/yard/settings" label="Config" />
                    </nav>
                </div>
            </header>

            // Main Content Area
            <main class="flex-grow relative">
                // Blueprint Grid Overlay
                <div class="absolute inset-0 blueprint-grid pointer-events-none z-0"></div>

                <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                    {children()}
                </div>
            </main>

            // Footer
            <footer class="border-t border-purdue-gold/10 bg-industrial-surface py-6 mt-auto">
                <div class="max-w-7xl mx-auto px-4 text-center">
                    <p class="text-xs text-purdue-gold/40 font-mono uppercase tracking-widest">
                        "System v0.6.0 // Sovereign Gamutainment // Boilermaker Industrial 2.0"
                    </p>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn NavButton(#[prop(into)] href: String, #[prop(into)] label: String) -> impl IntoView {
    view! {
        <a
            href=href
            class="px-4 py-2 text-sm font-bold text-purdue-gold hover:text-purdue-prime hover:bg-purdue-gold/10 transition-all duration-200 uppercase tracking-wide rounded-sm border border-transparent hover:border-purdue-gold/30"
        >
            {label}
        </a>
    }
}

// --- UI Components ---

#[component]
pub fn BoilermakerPanel(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class={format!("boilermaker-panel bg-industrial-surface border border-purdue-gold/30 shadow-xl {}", class)}>
            {children()}
        </div>
    }
}

#[component]
pub fn ChamferedPanel(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class={format!("relative p-6 bg-industrial-surface border border-purdue-gold/20 chamfered-corners {}", class)}>
            // Corner Accents
            <div class="absolute top-0 left-0 w-2 h-2 border-t border-l border-purdue-prime opacity-50"></div>
            <div class="absolute top-0 right-0 w-2 h-2 border-t border-r border-purdue-prime opacity-50"></div>
            <div class="absolute bottom-0 left-0 w-2 h-2 border-b border-l border-purdue-prime opacity-50"></div>
            <div class="absolute bottom-0 right-0 w-2 h-2 border-b border-r border-purdue-prime opacity-50"></div>

            {children()}
        </div>
    }
}

#[component]
pub fn MechanicalButton(
    #[prop(optional, into)] class: String,
    #[prop(optional)] primary: bool,
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let base_class = if primary {
        "mechanical-button-primary"
    } else {
        "mechanical-button-secondary"
    };

    view! {
        <button
            class={format!("{} {}", base_class, class)}
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn PressureGauge(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: Signal<f64>,
    #[prop(optional, into)] label: String,
) -> impl IntoView {
    view! {
        <div class={format!("w-full space-y-1 {}", class)}>
            <div class="flex justify-between text-xs font-mono text-purdue-gold uppercase tracking-wider">
                <span>{label}</span>
                <span>{move || format!("{:.0}%", value.get() * 100.0)}</span>
            </div>
            <div class="pressure-gauge bg-black/50">
                <div
                    class="pressure-gauge-fill"
                    style=move || format!("width: {}%", value.get() * 100.0)
                ></div>
            </div>
        </div>
    }
}

#[component]
pub fn MetricCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(optional, into)] icon: String,
) -> impl IntoView {
    view! {
        <div class="p-4 bg-industrial-surface border border-purdue-gold/20 chamfered-corners flex items-center justify-between group hover:border-purdue-prime/50 transition-colors">
            <div>
                <p class="text-xs text-purdue-gold/60 font-mono uppercase tracking-widest mb-1">{label}</p>
                <p class="text-2xl font-black text-steam-white group-hover:text-purdue-prime transition-colors">{value}</p>
            </div>
            <div class="text-3xl opacity-20 group-hover:opacity-100 transition-opacity grayscale group-hover:grayscale-0">
                {icon}
            </div>
        </div>
    }
}

#[component]
pub fn StatusLight(#[prop(into)] status: String) -> impl IntoView {
    let color_class = match status.as_str() {
        "online" => "bg-gauge-green shadow-[0_0_10px_#4F7942]",
        "offline" => "bg-signal-red shadow-[0_0_10px_#C8102E]",
        "busy" => "bg-purdue-prime shadow-[0_0_10px_#FFD700]",
        _ => "bg-purdue-dust",
    };

    view! {
        <div class="flex items-center gap-2 font-mono text-xs uppercase tracking-widest text-purdue-gold/80">
            <div class={format!("w-2 h-2 rounded-full animate-pulse {}", color_class)}></div>
            {status}
        </div>
    }
}
