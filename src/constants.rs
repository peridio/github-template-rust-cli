//! Compile-time constants for the application.

/// GitHub organization or username that owns this repository.
pub const GITHUB_OWNER: &str = "__TEMPLATE_REPO_OWNER__";

/// GitHub repository name.
pub const GITHUB_REPO: &str = "__TEMPLATE_REPO__";

/// Application name used in user agent strings.
pub const APP_NAME: &str = "__TEMPLATE_PACKAGE_NAME__";

/// Application version from Cargo.toml.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration file name.
pub const DEFAULT_CONFIG_FILE: &str = "config.json";
