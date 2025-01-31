use incredible_config::IncredibleConfig;

/// Avs builder
#[derive(Debug)]
pub struct AvsBuilder {
    pub(crate) config: IncredibleConfig,
}

impl AvsBuilder {
    /// new
    pub fn new(config: IncredibleConfig) -> Self {
        Self { config }
    }
}
