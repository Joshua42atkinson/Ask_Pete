use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a "Makeup" style (Livery) that can be applied to the interface.
/// This acts as a "Port" where different aesthetic configurations can be plugged in.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum MakeupStyle {
    #[default]
    StandardIndustrial, // The default Boilermaker look
    NeonNights, // Cyberpunk/High-contrast dark
    Blueprint,  // Technical drawing aesthetic
    IvoryTower, // Light mode / Academic
    RustBucket, // Weathered industrial
}

impl fmt::Display for MakeupStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MakeupStyle::StandardIndustrial => write!(f, "Standard Issue"),
            MakeupStyle::NeonNights => write!(f, "Neon Nights"),
            MakeupStyle::Blueprint => write!(f, "Schematic View"),
            MakeupStyle::IvoryTower => write!(f, "Ivory Tower"),
            MakeupStyle::RustBucket => write!(f, "Heavy Metal"),
        }
    }
}

/// Defines the color overrides for a specific makeup style.
/// If a field is None, it falls back to the hardcoded base theme.
#[derive(Clone, Debug)]
pub struct MakeupPalette {
    pub background_primary: Option<&'static str>,
    pub background_secondary: Option<&'static str>,
    pub accent_primary: Option<&'static str>,
    pub accent_secondary: Option<&'static str>,
    pub text_primary: Option<&'static str>,
    pub border_color: Option<&'static str>,
    pub glow_color: Option<&'static str>,
    pub background_pattern: Option<&'static str>,
}

impl MakeupStyle {
    pub fn palette(&self) -> MakeupPalette {
        match self {
            MakeupStyle::StandardIndustrial => MakeupPalette {
                background_primary: None,
                background_secondary: None,
                accent_primary: None,
                accent_secondary: None,
                text_primary: None,
                border_color: None,
                glow_color: None,
                background_pattern: Some("url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGRlZnM+PHBhdHRlcm4gaWQ9ImdyaWQiIHdpZHRoPSI0MCIgaGVpZ2h0PSI0MCIgcGF0dGVyblVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+PHBhdGggZD0iTSAwIDEwIEwgNDAgMTAgTSAxMCAwIEwgMTAgNDAgTSAwIDIwIEwgNDAgMjAgTSAyMCAwIEwgMjAgNDAgTSAwIDMwIEwgNDAgMzAgTSAzMCAwIEwgMzAgNDAiIGZpbGw9Im5vbmUiIHN0cm9rZT0iIzNhM2EzYSIgb3BhY2l0eT0iMC4zIiBzdHJva2Utd2lkdGg9IjEiLz48L3BhdHRlcm4+PC9kZWZzPjxyZWN0IHdpZHRoPSIxMDAlIiBoZWlnaHQ9IjEwMCUiIGZpbGw9InVybCgjZ3JpZCkiLz48L3N2Zz4=')"),
            },
            MakeupStyle::NeonNights => MakeupPalette {
                background_primary: Some("#050510"),
                background_secondary: Some("#0a0a1a"),
                accent_primary: Some("#00ff9d"),
                accent_secondary: Some("#bc13fe"),
                text_primary: Some("#e0e0e0"),
                border_color: Some("#00ff9d"),
                glow_color: Some("#00ff9d"),
                background_pattern: Some("radial-gradient(circle at 50% 50%, rgba(188, 19, 254, 0.1) 0%, transparent 50%), url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAiIGhlaWdodD0iMjAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGNpcmNsZSBjeD0iMSIgY3k9IjEiIHI9IjEiIGZpbGw9IiMwMGZmOWQiIG9wYWNpdHk9IjAuMiIvPjwvc3ZnPg==')"), // Hex/Dot grid
            },
            MakeupStyle::Blueprint => MakeupPalette {
                background_primary: Some("#003366"),
                background_secondary: Some("#002244"),
                accent_primary: Some("#ffffff"),
                accent_secondary: Some("#89cff0"),
                text_primary: Some("#ffffff"),
                border_color: Some("#ffffff"),
                glow_color: Some("#ffffff"),
                background_pattern: Some("url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTAwIiBoZWlnaHQ9IjEwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZGVmcz48cGF0dGVybiBpZD0iZ3JpZCIgd2lkdGg9IjEwMCIgaGVpZ2h0PSIxMDAiIHBhdHRlcm5Vbml0cz0idXNlclNwYWNlT25Vc2UiPjxwYXRoIGQ9Ik0gMTAwIDAgTCAwIDAgTCAwIDEwMCIgZmlsbD0ibm9uZSIgc3Ryb2tlPSIjZmZmZmZmIiBzdHJva2Utd2lkdGg9IjEiIG9wYWNpdHk9IjAuMiIvPjwvcGF0dGVybj48L2RlZnM+PHJlY3Qgd2lkdGg9IjEwMCUiIGhlaWdodD0iMTAwJSIgZmlsbD0idXJsKCNncmlkKSIvPjwvc3ZnPg==')"),
            },
            MakeupStyle::IvoryTower => MakeupPalette {
                background_primary: Some("#f5f5f0"),
                background_secondary: Some("#ffffff"),
                accent_primary: Some("#ceb888"),
                accent_secondary: Some("#121212"),
                text_primary: Some("#121212"),
                border_color: Some("#ceb888"),
                glow_color: Some("#ceb888"),
                background_pattern: Some("radial-gradient(#ceb888 1px, transparent 1px)"), // Subtle dots
            },
            MakeupStyle::RustBucket => MakeupPalette {
                background_primary: Some("#2a1a1a"),
                background_secondary: Some("#1a0f0f"),
                accent_primary: Some("#cd5c5c"),
                accent_secondary: Some("#d2691e"),
                text_primary: Some("#e0c0a0"),
                border_color: Some("#8b4513"),
                glow_color: Some("#cd5c5c"),
                background_pattern: Some("repeating-linear-gradient(45deg, #2a1a1a 25%, #1a0f0f 25%, #1a0f0f 50%, #2a1a1a 50%, #2a1a1a 75%, #1a0f0f 75%, #1a0f0f 100%)"), // Hazard stripes
            },
        }
    }
}
