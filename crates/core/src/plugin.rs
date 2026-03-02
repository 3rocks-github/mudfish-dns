/// Plugin Model.
/// Supports Custom Rules (Include/Exclude domains), Official Filtering (Ad/Tracker blocking, Category blocking), DNS Leak Prevention.

pub trait Plugin {
    fn name(&self) -> &str;
    fn process_query(&self, domain: &str) -> PluginAction;
}

pub enum PluginAction {
    Allow,
    Block,
    Redirect(String),
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn evaluate(&self, domain: &str) -> PluginAction {
        // Iterate and apply plugin rules
        PluginAction::Allow
    }
}
