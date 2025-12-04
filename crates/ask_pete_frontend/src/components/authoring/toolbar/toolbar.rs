use super::menu_bar::MenuBar;
use super::tool_group::{ToolButton, ToolGroup};
use leptos::prelude::*;

#[component]
pub fn Toolbar(
    /// Handler for adding a node
    on_add_node: Callback<()>,
    /// Handler for saving
    on_save: Callback<()>,
    /// Handler for test drive
    on_test_drive: Callback<()>,
    /// Handler for resetting view
    on_reset_view: Callback<()>,
    /// Handler for AI design
    on_ai_design: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="absolute top-0 left-0 right-0 z-20">
            // Menu Bar
            <MenuBar />

            // Quick Actions Toolbar
            <div class="bg-slate-800/95 border-b border-slate-700 px-4 py-2 backdrop-blur-sm">
                <div class="flex items-center gap-2 flex-wrap">
                    // Edit Tools
                    <ToolGroup title="Edit" show_divider=true>
                        <ToolButton
                            icon="➕"
                            label="Add Node"
                            tooltip="Add a new node to the canvas (Ctrl+N)"
                            on_click=on_add_node
                        />
                        <ToolButton
                            icon="✂️"
                            label="Cut"
                            tooltip="Cut selected node (Ctrl+X)"
                        />
                        <ToolButton
                            icon="📋"
                            label="Copy"
                            tooltip="Copy selected node (Ctrl+C)"
                        />
                        <ToolButton
                            icon="🗑️"
                            label="Delete"
                            tooltip="Delete selected node (Delete)"
                        />
                    </ToolGroup>

                    // View Tools
                    <ToolGroup title="View" show_divider=true>
                        <ToolButton
                            icon="🔍"
                            label="Fit"
                            tooltip="Fit all nodes to screen (Ctrl+0)"
                            on_click=on_reset_view
                        />
                        <ToolButton
                            icon="➕"
                            label="Zoom In"
                            tooltip="Zoom in (Ctrl++)"
                        />
                        <ToolButton
                            icon="➖"
                            label="Zoom Out"
                            tooltip="Zoom out (Ctrl+-)"
                        />
                    </ToolGroup>

                    // Actions
                    <ToolGroup title="Actions" show_divider=false>
                        <ToolButton
                            icon="🤖"
                            label="AI Design"
                            tooltip="Design with Pete AI assistant"
                            on_click=on_ai_design
                            variant="primary"
                        />
                        <ToolButton
                            icon="💾"
                            label="Save"
                            tooltip="Save graph (Ctrl+S)"
                            on_click=on_save
                        />
                        <ToolButton
                            icon="▶️"
                            label="Test Drive"
                            tooltip="Preview your story in Play Mode"
                            on_click=on_test_drive
                            variant="success"
                        />
                    </ToolGroup>
                </div>
            </div>
        </div>
    }
}
