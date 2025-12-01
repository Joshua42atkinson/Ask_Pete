use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::*;

#[component]
pub fn AdminPanel() -> impl IntoView {
    // Mock data for MVP
    let _department = "Computer Science"; // Prefixed with _ to suppress unused warning
    let budget = 1000.0;
    let current_usage = 125.50;
    let safety_lockout_active = false;

    view! {
        <div class="p-6 bg-gray-800 text-white rounded-lg shadow-xl">
            <h2 class="text-2xl font-bold mb-4">"Internal Recharge Center"</h2>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                // Budget Card
                <div class="bg-gray-700 p-4 rounded-lg">
                    <h3 class="text-lg font-semibold text-gray-300">"Departmental Budget"</h3>
                    <div class="text-3xl font-bold text-green-400 mt-2">
                        "$" {budget}
                    </div>
                    <p class="text-sm text-gray-400 mt-1">"FY 2025 Allocation"</p>
                </div>

                // Usage Card
                <div class="bg-gray-700 p-4 rounded-lg">
                    <h3 class="text-lg font-semibold text-gray-300">"Current Usage"</h3>
                    <div class="text-3xl font-bold text-yellow-400 mt-2">
                        "$" {current_usage}
                    </div>
                    <p class="text-sm text-gray-400 mt-1">"Coal Burned: 125,500 units"</p>
                </div>
            </div>

            // Safety Status
            <div class="mt-6 p-4 bg-gray-900 rounded-lg border border-gray-600">
                <div class="flex items-center justify-between">
                    <div>
                        <h3 class="text-lg font-semibold">"Safety Lockout Status"</h3>
                        <p class="text-sm text-gray-400">"AI Mirror Monitoring System"</p>
                    </div>
                    <div class={if safety_lockout_active { "text-red-500 font-bold" } else { "text-green-500 font-bold" }}>
                        {if safety_lockout_active { "LOCKED" } else { "ACTIVE" }}
                    </div>
                </div>
            </div>

            // Action Buttons
            <div class="mt-6 flex gap-4">
                <button class="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded transition">
                    "Generate Journal Voucher"
                </button>
                <button class="px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded transition">
                    "Download Usage Report"
                </button>
            </div>
        </div>
    }
}
