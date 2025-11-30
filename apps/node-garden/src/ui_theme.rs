use crate::theme::makeup::MakeupStyle;
/// Boilermaker Industrial Design System
///
/// This module provides the centralized theme system for the "Ask Pete" application,
/// implementing the Purdue Boilermaker Industrial aesthetic with official colors,
/// industrial design patterns, and reusable style helpers.

/// # Boilermaker Color Palette
///
/// Official Purdue colors and industrial-themed variants for the "Ask Pete" interface.
use leptos::prelude::*;

#[derive(Clone, Debug, Copy)]
pub struct ThemeContext {
    pub makeup: Signal<MakeupStyle>,
    pub set_makeup: WriteSignal<MakeupStyle>,
}

pub fn provide_theme_context() {
    // Try to load from local storage
    let initial_makeup = if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(json)) = storage.get_item("pete_makeup_style") {
                serde_json::from_str(&json).unwrap_or_default()
            } else {
                MakeupStyle::default()
            }
        } else {
            MakeupStyle::default()
        }
    } else {
        MakeupStyle::default()
    };

    let (makeup, set_makeup) = signal(initial_makeup);

    // Persist changes to local storage
    Effect::new(move |_| {
        let style = makeup.get();
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(
                    "pete_makeup_style",
                    &serde_json::to_string(&style).unwrap_or_default(),
                );
            }
        }
    });

    provide_context(ThemeContext {
        makeup: makeup.into(),
        set_makeup,
    });
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("ThemeContext not found")
}

pub mod colors {
    /// Primary Background: Boilermaker Black
    /// Used for main backgrounds to create the industrial, console-like feel
    pub const BOILERMAKER_BLACK: &str = "#121212";

    /// Absolute black variant
    pub const PURE_BLACK: &str = "#000000";

    /// Primary Accent: Old Gold (Official Purdue Gold)
    /// Used for borders, highlights, and interactive elements
    pub const OLD_GOLD: &str = "#CEB888";

    /// Secondary Accent: Steam White
    /// Used for primary text and icons
    pub const STEAM_WHITE: &str = "#F0F0F0";

    /// Alert/Action: Signal Red
    /// Used for errors, warnings, and stop actions
    pub const SIGNAL_RED: &str = "#C8102E";

    /// Success: Gauge Green
    /// Used for success states and completion indicators
    pub const GAUGE_GREEN: &str = "#4F7942";

    /// Dark variant for layering
    pub const PURDUE_DARK: &str = "#1a1a1a";

    /// Dust/secondary text color
    pub const PURDUE_DUST: &str = "#a0a0a0";

    /// Majestic Royal Neon Purple (for Pete's branding)
    pub const MAJESTIC_ROYAL_NEON_PURPLE: &str = "#BC13FE";
}

/// # Typography System
///
/// Font families for different content types to maintain the industrial aesthetic
pub mod typography {
    /// Monospace font for AI text and terminal-like displays
    /// Creates the "engineering tool" feel
    pub const AI_FONT: &str = "JetBrains Mono, Roboto Mono, Consolas, monospace";

    /// Sans-serif font for UI labels and navigation
    pub const UI_FONT: &str = "Inter, system-ui, -apple-system, sans-serif";
}

/// # Style Helpers
///
/// Functions that generate Tailwind CSS classes for common Boilermaker Industrial patterns

/// Returns CSS classes for a chamfered panel (45-degree cut corners)
///
/// # Industrial Design
/// - No rounded corners (replaced with chamfered/cut corners)
/// - 1px Old Gold border stroke
/// - Boilermaker Black background
///
/// # Arguments
/// * `with_glow` - If true, adds a subtle gold glow effect
pub fn chamfered_panel_classes(with_glow: bool) -> String {
    let base = "bg-[#121212] border border-[#CEB888] p-6 chamfered-corners";

    if with_glow {
        format!("{} shadow-lg shadow-[#CEB888]/20", base)
    } else {
        base.to_string()
    }
}

/// Returns CSS classes for a mechanical button/switch
///
/// # Industrial Design
/// - Locomotive control switch appearance
/// - Gold glow on hover
/// - Mechanical press animation
///
/// # Arguments
/// * `variant` - Button variant: "primary", "secondary", "danger"
pub fn mechanical_button_classes(variant: &str) -> String {
    let base = "px-6 py-3 font-bold uppercase tracking-wide transition-all duration-200 chamfered-corners border-2";

    match variant {
        "primary" => format!(
            "{} bg-[#CEB888] text-black border-[#CEB888] hover:bg-[#F0F0F0] hover:shadow-lg hover:shadow-[#CEB888]/50 active:scale-95",
            base
        ),
        "secondary" => format!(
            "{} bg-transparent text-[#CEB888] border-[#CEB888] hover:bg-[#CEB888]/10 hover:shadow-lg hover:shadow-[#CEB888]/30 active:scale-95",
            base
        ),
        "danger" => format!(
            "{} bg-[#C8102E] text-white border-[#C8102E] hover:bg-[#d0142f] hover:shadow-lg hover:shadow-[#C8102E]/50 active:scale-95",
            base
        ),
        "success" => format!(
            "{} bg-[#4F7942] text-white border-[#4F7942] hover:bg-[#5a8a4d] hover:shadow-lg hover:shadow-[#4F7942]/50 active:scale-95",
            base
        ),
        _ => mechanical_button_classes("primary"),
    }
}

/// Returns CSS classes for a pressure gauge progress bar
///
/// # Industrial Design
/// - Liquid gold fill effect
/// - Steam pipe or pressure gauge visual metaphor
/// - Industrial font for percentage display
pub fn pressure_gauge_classes() -> String {
    "relative w-full h-8 bg-[#1a1a1a] border-2 border-[#CEB888] chamfered-corners overflow-hidden"
        .to_string()
}

/// Returns CSS classes for the progress fill inside a pressure gauge
pub fn pressure_gauge_fill_classes() -> String {
    "absolute top-0 left-0 h-full bg-gradient-to-r from-[#CEB888] to-[#e0ca9a] transition-all duration-500 ease-out shadow-lg shadow-[#CEB888]/50".to_string()
}

/// Returns CSS classes for gold glow hover effect
pub fn gold_glow_hover() -> String {
    "transition-all duration-200 hover:shadow-lg hover:shadow-[#CEB888]/40 hover:border-[#CEB888]"
        .to_string()
}

/// Returns CSS classes for the industrial grid background
pub fn blueprint_grid_background() -> String {
    "fixed inset-0 bg-[image:var(--bg-pattern)] opacity-10 pointer-events-none z-0".to_string()
}

/// Generates the CSS variables style block for the current makeup
#[component]
pub fn MakeupLayer() -> impl IntoView {
    let theme = use_theme();

    view! {
        <style>
            {move || {
                let style = theme.makeup.get();
                let palette = style.palette();

                // We map the palette to CSS variables that can override the defaults
                // Note: The base components need to be updated to use these variables or we use a global override strategy
                // For now, we'll just set some root variables that we can start using
                format!(
                    ":root {{ --bg-primary: {}; --bg-secondary: {}; --accent-primary: {}; --accent-secondary: {}; --text-primary: {}; --border-color: {}; --glow-color: {}; --bg-pattern: {}; }}",
                    palette.background_primary.unwrap_or(colors::BOILERMAKER_BLACK),
                    palette.background_secondary.unwrap_or(colors::PURDUE_DARK),
                    palette.accent_primary.unwrap_or(colors::OLD_GOLD),
                    palette.accent_secondary.unwrap_or(colors::STEAM_WHITE), // Default secondary accent
                    palette.text_primary.unwrap_or(colors::STEAM_WHITE),
                    palette.border_color.unwrap_or(colors::OLD_GOLD),
                    palette.glow_color.unwrap_or(colors::OLD_GOLD),
                    palette.background_pattern.unwrap_or("none"),
                )
            }}
        </style>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_constants() {
        assert_eq!(colors::OLD_GOLD, "#CEB888");
        assert_eq!(colors::BOILERMAKER_BLACK, "#121212");
        assert_eq!(colors::SIGNAL_RED, "#C8102E");
    }

    #[test]
    fn test_mechanical_button_variants() {
        let primary = mechanical_button_classes("primary");
        assert!(primary.contains("#CEB888"));

        let danger = mechanical_button_classes("danger");
        assert!(danger.contains("#C8102E"));
    }
}
