use leptos::prelude::*;

#[component]
pub fn MenuBar() -> impl IntoView {
    view! {
        <div class="bg-slate-900 border-b border-slate-700 px-4 py-2 flex items-center gap-1 text-sm">
            // File Menu
            <div class="relative group">
                <button class="px-3 py-1 hover:bg-slate-800 rounded text-slate-200 font-medium">
                    "File"
                </button>
                <div class="absolute left-0 top-full mt-1 w-48 bg-slate-800 border border-slate-700 rounded shadow-xl opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all z-50">
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"New Graph"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Open..."</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Save"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Save As..."</a>
                    <div class="border-t border-slate-700 my-1"></div>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Export..."</a>
                </div>
            </div>

            // Edit Menu
            <div class="relative group">
                <button class="px-3 py-1 hover:bg-slate-800 rounded text-slate-200 font-medium">
                    "Edit"
                </button>
                <div class="absolute left-0 top-full mt-1 w-48 bg-slate-800 border border-slate-700 rounded shadow-xl opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all z-50">
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Undo"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Redo"</a>
                    <div class="border-t border-slate-700 my-1"></div>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Cut"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Copy"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Paste"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Delete"</a>
                </div>
            </div>

            // View Menu
            <div class="relative group">
                <button class="px-3 py-1 hover:bg-slate-800 rounded text-slate-200 font-medium">
                    "View"
                </button>
                <div class="absolute left-0 top-full mt-1 w-48 bg-slate-800 border border-slate-700 rounded shadow-xl opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all z-50">
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Zoom In"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Zoom Out"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Fit to Screen"</a>
                    <div class="border-t border-slate-700 my-1"></div>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Show Grid"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Show Minimap"</a>
                </div>
            </div>

            // Insert Menu
            <div class="relative group">
                <button class="px-3 py-1 hover:bg-slate-800 rounded text-slate-200 font-medium">
                    "Insert"
                </button>
                <div class="absolute left-0 top-full mt-1 w-48 bg-slate-800 border border-slate-700 rounded shadow-xl opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all z-50">
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"New Node"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"From Template..."</a>
                    <div class="border-t border-slate-700 my-1"></div>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Image..."</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Video..."</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Audio..."</a>
                </div>
            </div>

            // Help Menu
            <div class="relative group">
                <button class="px-3 py-1 hover:bg-slate-800 rounded text-slate-200 font-medium">
                    "Help"
                </button>
                <div class="absolute left-0 top-full mt-1 w-48 bg-slate-800 border border-slate-700 rounded shadow-xl opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all z-50">
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Documentation"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Keyboard Shortcuts"</a>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"Video Tutorials"</a>
                    <div class="border-t border-slate-700 my-1"></div>
                    <a href="#" class="block px-4 py-2 hover:bg-slate-700 text-slate-200">"About"</a>
                </div>
            </div>
        </div>
    }
}
