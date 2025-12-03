use crate::core::traits::{AssessmentPlugin, NarrativeFramework, NodeTypeExtension, ThemeProvider};
use std::collections::HashMap;
use std::sync::Arc;

/// Central registry for all plugins in the Daydream system.
///
/// The registry manages plugin lifecycle and provides access to
/// registered plugins by ID.
///
/// # Example
///
/// ```rust
/// let mut registry = PluginRegistry::new();
///
/// // Register a narrative framework
/// registry.register_framework(
///     "heros_journey",
///     Box::new(HerosJourneyFramework::new())
/// );
///
/// // Later, retrieve it
/// if let Some(framework) = registry.get_framework("heros_journey") {
///     let stages = framework.get_stages();
/// }
/// ```
#[derive(Default)]
pub struct PluginRegistry {
    narrative_frameworks: HashMap<String, Arc<dyn NarrativeFramework>>,
    themes: HashMap<String, Arc<dyn ThemeProvider>>,
    assessments: HashMap<String, Arc<dyn AssessmentPlugin>>,
    node_extensions: HashMap<String, Arc<dyn NodeTypeExtension>>,
}

impl PluginRegistry {
    /// Create a new empty plugin registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a registry with all built-in plugins loaded
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.load_builtin_plugins();
        registry
    }

    // ========================================================================
    // NARRATIVE FRAMEWORKS
    // ========================================================================

    /// Register a narrative framework plugin
    pub fn register_framework(&mut self, id: &str, framework: Arc<dyn NarrativeFramework>) {
        self.narrative_frameworks.insert(id.to_string(), framework);
        log::info!("Registered narrative framework: {}", id);
    }

    /// Get a narrative framework by ID
    pub fn get_framework(&self, id: &str) -> Option<Arc<dyn NarrativeFramework>> {
        self.narrative_frameworks.get(id).cloned()
    }

    /// List all registered narrative frameworks
    pub fn list_frameworks(&self) -> Vec<String> {
        self.narrative_frameworks.keys().cloned().collect()
    }

    // ========================================================================
    // THEMES
    // ========================================================================

    /// Register a theme provider plugin
    pub fn register_theme(&mut self, id: &str, theme: Arc<dyn ThemeProvider>) {
        self.themes.insert(id.to_string(), theme);
        log::info!("Registered theme: {}", id);
    }

    /// Get a theme by ID
    pub fn get_theme(&self, id: &str) -> Option<Arc<dyn ThemeProvider>> {
        self.themes.get(id).cloned()
    }

    /// List all registered themes
    pub fn list_themes(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }

    // ========================================================================
    // ASSESSMENTS
    // ========================================================================

    /// Register an assessment plugin
    pub fn register_assessment(&mut self, id: &str, assessment: Arc<dyn AssessmentPlugin>) {
        self.assessments.insert(id.to_string(), assessment);
        log::info!("Registered assessment: {}", id);
    }

    /// Get an assessment plugin by ID
    pub fn get_assessment(&self, id: &str) -> Option<Arc<dyn AssessmentPlugin>> {
        self.assessments.get(id).cloned()
    }

    /// List all registered assessments
    pub fn list_assessments(&self) -> Vec<String> {
        self.assessments.keys().cloned().collect()
    }

    // ========================================================================
    // NODE EXTENSIONS
    // ========================================================================

    /// Register a node type extension
    pub fn register_node_extension(&mut self, id: &str, extension: Arc<dyn NodeTypeExtension>) {
        self.node_extensions.insert(id.to_string(), extension);
        log::info!("Registered node extension: {}", id);
    }

    /// Get a node extension by type ID
    pub fn get_node_extension(&self, type_id: &str) -> Option<Arc<dyn NodeTypeExtension>> {
        self.node_extensions.get(type_id).cloned()
    }

    /// List all registered node extensions
    pub fn list_node_extensions(&self) -> Vec<String> {
        self.node_extensions.keys().cloned().collect()
    }

    // ========================================================================
    // UTILITIES
    // ========================================================================

    /// Load all built-in plugins
    ///
    /// This will be expanded as we create built-in plugins:
    /// - Hero's Journey framework
    /// - Freytag's Pyramid framework
    /// - Blank framework
    /// - Anime theme
    /// - Military theme
    /// - Fantasy theme
    fn load_builtin_plugins(&mut self) {
        log::info!("Loading built-in plugins...");

        // TODO: Register built-in plugins here as they're implemented
        // Example:
        // self.register_framework("heros_journey", Arc::new(HerosJourneyFramework::new()));
        // self.register_theme("anime", Arc::new(AnimeTheme::new()));

        log::info!("Built-in plugins loaded");
    }

    /// Get total count of all registered plugins
    pub fn plugin_count(&self) -> usize {
        self.narrative_frameworks.len()
            + self.themes.len()
            + self.assessments.len()
            + self.node_extensions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = PluginRegistry::new();
        assert_eq!(registry.plugin_count(), 0);
    }

    #[test]
    fn test_list_empty() {
        let registry = PluginRegistry::new();
        assert!(registry.list_frameworks().is_empty());
        assert!(registry.list_themes().is_empty());
        assert!(registry.list_assessments().is_empty());
    }
}
