use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::hooks::use_query_map;
use web_sys::window;

#[component]
pub fn LandingPage() -> impl IntoView {
    let navigate = use_navigate();
    let query = use_query_map();

    // Check for token and role in query params (Redirect from Google Auth)
    let nav_effect = navigate.clone();
    Effect::new(move |_| {
        let q = query.get();
        if let (Some(token), Some(role_str)) = (q.get("token"), q.get("role")) {
            gloo_console::log!(format!("Login Successful! Role: {}", role_str));

            // Save to LocalStorage
            if let Some(win) = window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    let _ = storage.set_item("auth_token", &token);
                    let _ = storage.set_item("user_role", &role_str);
                }
            }

            // Redirect based on role
            let target = match role_str.as_str() {
                "Student" => "/cab",
                "Instructor" => "/yard",
                "Researcher" => "/tower",
                _ => "/",
            };
            nav_effect(target, Default::default());
        }
    });

    view! {
        <div class="h-screen w-screen bg-[#1a1a1a] flex flex-col items-center justify-center text-[#e0e0e0] font-mono overflow-hidden relative">
            // Background Atmosphere (Industrial/Station)
            <div class="absolute inset-0 bg-[url('/assets/station_bg.jpg')] bg-cover bg-center opacity-20 pointer-events-none"></div>
            <div class="absolute inset-0 bg-gradient-to-b from-[#0a0a0a] via-transparent to-[#0a0a0a] pointer-events-none"></div>

            // Split-Flap Display Container
            <div class="z-10 flex flex-col items-center space-y-8">
                <div class="flex flex-col items-center space-y-2">
                    <h1 class="text-4xl md:text-6xl font-bold tracking-widest uppercase text-[#d4af37] drop-shadow-md">
                        Grand Central
                    </h1>
                    <div class="h-1 w-32 bg-[#d4af37]"></div>
                </div>

                // Animated Text (Split-Flap Simulation)
                <div class="bg-[#111] border-4 border-[#333] p-6 rounded-lg shadow-2xl">
                    <p class="text-2xl md:text-4xl font-bold text-[#00ffcc] tracking-wider animate-pulse">
                        "WHERE DO YOU WANT TO LEARN TODAY?"
                    </p>
                </div>

                // Ticket Punch Button
                <a
                    href="/auth/login"
                    class="group relative inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-white transition-all duration-200 bg-[#8b0000] font-serif rounded-sm hover:bg-[#a00000] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#8b0000]"
                >
                    <span class="absolute inset-0 w-full h-full -mt-1 rounded-lg opacity-30 bg-gradient-to-b from-transparent via-transparent to-black"></span>
                    <span class="relative flex items-center gap-3">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z" />
                        </svg>
                        "PUNCH TICKET"
                    </span>
                </a>

                <p class="text-sm text-gray-500 mt-4">
                    "Departing for: The Locomotive Cab, The Train Yard, The Signal Tower"
                </p>

                // Model Manager Link (Temporary for Dev)
                <a href="/models" class="text-xs text-gray-600 hover:text-blue-400 underline transition-colors">
                    "[MANAGE OFFLINE MODELS]"
                </a>

                // --- DEV TOOLS: MOCK LOGIN ---
                <div class="flex gap-4 mt-8 p-4 border border-dashed border-gray-700 rounded-lg opacity-50 hover:opacity-100 transition-opacity">
                    <span class="text-xs text-gray-500 uppercase tracking-widest self-center">"Dev Bypass:"</span>
                    <button
                        class="px-3 py-1 text-xs bg-blue-900 text-blue-200 rounded hover:bg-blue-800"
                        on:click={
                            let navigate = navigate.clone();
                            move |_| {
                                if let Some(win) = window() {
                                    if let Ok(Some(storage)) = win.local_storage() {
                                        let _ = storage.set_item("auth_token", "mock_student_token");
                                        let _ = storage.set_item("user_role", "Student");
                                    }
                                }
                                navigate("/cab", Default::default());
                            }
                        }
                    >
                        "Student"
                    </button>
                    <button
                        class="px-3 py-1 text-xs bg-green-900 text-green-200 rounded hover:bg-green-800"
                        on:click={
                            let navigate = navigate.clone();
                            move |_| {
                                if let Some(win) = window() {
                                    if let Ok(Some(storage)) = win.local_storage() {
                                        let _ = storage.set_item("auth_token", "mock_instructor_token");
                                        let _ = storage.set_item("user_role", "Instructor");
                                    }
                                }
                                navigate("/yard", Default::default());
                            }
                        }
                    >
                        "Instructor"
                    </button>
                    <button
                        class="px-3 py-1 text-xs bg-purple-900 text-purple-200 rounded hover:bg-purple-800"
                        on:click={
                            let navigate = navigate.clone();
                            move |_| {
                                if let Some(win) = window() {
                                    if let Ok(Some(storage)) = win.local_storage() {
                                        let _ = storage.set_item("auth_token", "mock_researcher_token");
                                        let _ = storage.set_item("user_role", "Researcher");
                                    }
                                }
                                navigate("/tower", Default::default());
                            }
                        }
                    >
                        "Researcher"
                    </button>
                </div>
            </div>
        </div>
    }
}
