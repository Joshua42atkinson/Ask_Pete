use crate::components::boilermaker::{BoilermakerPanel, MechanicalButton, MetricCard, StatusLight};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="space-y-8 animate-fade-in">
            // Hero Section
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div class="lg:col-span-2 space-y-6">
                    <BoilermakerPanel class="bg-industrial-surface/80 backdrop-blur-sm">
                        <div class="flex items-start justify-between mb-6">
                            <div>
                                <h2 class="text-3xl font-black text-purdue-prime uppercase tracking-tight mb-2">
                                    "System Status: Operational"
                                </h2>
                                <p class="text-purdue-gold/60 font-mono text-sm tracking-widest">
                                    "Authorized Personnel Only // Clearance Level 4"
                                </p>
                            </div>
                            <StatusLight status="online" />
                        </div>

                        <p class="text-steam-white/80 leading-relaxed mb-8 max-w-2xl">
                            "Welcome to the Purdue Expert Tuition Engine (PETE). This system is designed to facilitate high-bandwidth Socratic dialogue and instructional design operations. All interactions are monitored for quality assurance."
                        </p>

                        <div class="flex gap-4">
                            <a href="/tower/chat">
                                <MechanicalButton primary=true class="w-full sm:w-auto">
                                    "Initiate Comms"
                                </MechanicalButton>
                            </a>
                            <a href="/tower/authoring">
                                <MechanicalButton class="w-full sm:w-auto">
                                    "Access Forge"
                                </MechanicalButton>
                            </a>
                        </div>
                    </BoilermakerPanel>

                    // Metrics Grid
                    <div class="grid grid-cols-1 sm::grid-cols-3 gap-4">
                        <MetricCard
                            label="System Load"
                            value="12%"
                            icon="⚡"
                        />
                        <MetricCard
                            label="Active Nodes"
                            value="843"
                            icon="🔗"
                        />
                        <MetricCard
                            label="Uptime"
                            value="42h 18m"
                            icon="⏱️"
                        />
                    </div>
                </div>

                // Sidebar / Feed
                <div class="space-y-4">
                    <BoilermakerPanel class="h-full">
                        <h3 class="text-sm font-bold text-purdue-gold uppercase tracking-widest mb-4 border-b border-purdue-gold/20 pb-2">
                            "System Logs"
                        </h3>
                        <div class="space-y-3 font-mono text-xs text-purdue-gold/70">
                            <div class="flex justify-between">
                                <span>"14:02:22"</span>
                                <span class="text-purdue-prime">"SYS_INIT_COMPLETE"</span>
                            </div>
                            <div class="flex justify-between">
                                <span>"14:02:21"</span>
                                <span>"LOADING_MODULES..."</span>
                            </div>
                            <div class="flex justify-between">
                                <span>"14:02:20"</span>
                                <span>"CONNECTING_DB..."</span>
                            </div>
                            <div class="flex justify-between opacity-50">
                                <span>"14:02:18"</span>
                                <span>"BOOT_SEQUENCE_START"</span>
                            </div>
                        </div>
                    </BoilermakerPanel>
                </div>
            </div>
        </div>
    }
}
