//! Environment variable names for the application.

// CLI arg overrides
pub const CONFIG: &str = concat!("__TEMPLATE_ENV_PREFIX__", "_CONFIG");
pub const PROFILE: &str = concat!("__TEMPLATE_ENV_PREFIX__", "_PROFILE");

// CLI config overrides
pub const OUTPUT_DIR: &str = concat!("__TEMPLATE_ENV_PREFIX__", "_OUTPUT_DIR");
pub const LOG_LEVEL: &str = concat!("__TEMPLATE_ENV_PREFIX__", "_LOG_LEVEL");
pub const PARALLEL_JOBS: &str = concat!("__TEMPLATE_ENV_PREFIX__", "_PARALLEL_JOBS");

// Other
