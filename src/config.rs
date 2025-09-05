use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::env_vars;
use crate::error::{Error, Result};
use std::collections::HashMap;

/// Individual profile configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Profile {
    /// Output directory for this profile
    pub output_dir: PathBuf,

    /// Log level for this profile (error, warning, info, debug, trace)
    pub log_level: String,

    /// Number of parallel jobs to run
    pub parallel_jobs: u32,
}

/// Main configuration structure for the CLI.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    /// Default profile to use
    pub default_profile: String,

    /// Profile configurations
    pub profiles: HashMap<String, Profile>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./output"),
            log_level: String::from("info"),
            parallel_jobs: 4,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut profiles = HashMap::new();

        // Add default profiles
        profiles.insert(
            String::from("local"),
            Profile {
                output_dir: PathBuf::from("./output"),
                log_level: String::from("debug"),
                parallel_jobs: 4,
            },
        );

        profiles.insert(
            String::from("ci"),
            Profile {
                output_dir: PathBuf::from("/tmp/ci-output"),
                log_level: String::from("error"),
                parallel_jobs: 1,
            },
        );

        profiles.insert(
            String::from("release"),
            Profile {
                output_dir: PathBuf::from("./dist"),
                log_level: String::from("warning"),
                parallel_jobs: 8,
            },
        );

        Self {
            default_profile: String::from("local"),
            profiles,
        }
    }
}

impl Config {
    /// Loads configuration from the specified file.
    ///
    /// # Arguments
    /// * `path` - Path to configuration file
    ///
    /// # Returns
    /// * `Ok(Config)` - Loaded and validated configuration
    /// * `Err` - If loading or validation fails
    pub fn load(path: &str) -> Result<Self> {
        let config = Self::load_from_file(path)?;
        config.validate()?;
        Ok(config)
    }

    /// Loads configuration from a specific file.
    /// Automatically detects format based on file extension (.json, .yaml, .yml).
    fn load_from_file(path: &str) -> Result<Self> {
        let path = Path::new(path);

        if !path.exists() {
            debug!(
                "Configuration file not found: {}, using defaults",
                path.display()
            );
            return Ok(Self::default());
        }

        info!("Loading configuration from: {}", path.display());

        let contents = fs::read_to_string(path).map_err(Error::Io)?;

        // Detect format based on extension
        let config = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str(&contents).map_err(Error::Json)?,
            Some("yaml") | Some("yml") => serde_yaml::from_str(&contents)
                .map_err(|e| Error::Other(format!("Failed to parse YAML: {}", e)))?,
            _ => {
                // Default to JSON for backward compatibility
                serde_json::from_str(&contents).map_err(Error::Json)?
            }
        };

        debug!("Configuration loaded successfully");
        Ok(config)
    }

    /// Merge environment variables onto configuration.
    pub fn merge_env(&mut self) -> Result<()> {
        // Check for profile override
        if let Ok(profile) = std::env::var(env_vars::PROFILE) {
            self.default_profile = profile;
        }

        // Apply profile-specific overrides if active profile exists
        if let Some(profile) = self.profiles.get_mut(&self.default_profile) {
            if let Ok(val) = std::env::var(env_vars::OUTPUT_DIR) {
                profile.output_dir = PathBuf::from(val);
            }

            if let Ok(val) = std::env::var(env_vars::LOG_LEVEL) {
                profile.log_level = val;
            }

            if let Ok(val) = std::env::var(env_vars::PARALLEL_JOBS) {
                if let Ok(parsed) = val.parse() {
                    profile.parallel_jobs = parsed;
                }
            }
        }

        Ok(())
    }

    /// Validates the configuration.
    fn validate(&self) -> Result<()> {
        // Validate that default profile exists
        if !self.profiles.contains_key(&self.default_profile) {
            return Err(Error::Other(format!(
                "Default profile '{}' not found in profiles",
                self.default_profile
            )));
        }

        // Validate each profile
        for (name, profile) in &self.profiles {
            if profile.output_dir.as_os_str().is_empty() {
                return Err(Error::Other(format!(
                    "Output directory cannot be empty in profile '{}'",
                    name
                )));
            }
            if profile.parallel_jobs == 0 {
                return Err(Error::Other(format!(
                    "Parallel jobs must be at least 1 in profile '{}'",
                    name
                )));
            }
            // Validate log level
            let valid_levels = ["error", "warn", "warning", "info", "debug", "trace"];
            if !valid_levels.contains(&profile.log_level.to_lowercase().as_str()) {
                return Err(Error::Other(format!(
                    "Invalid log level '{}' in profile '{}'. Valid levels: error, warn, info, debug, trace",
                    profile.log_level, name
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_profile, "local");
        assert!(config.profiles.contains_key("local"));
        assert!(config.profiles.contains_key("ci"));
        assert!(config.profiles.contains_key("release"));
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());

        // Test invalid profile reference
        config.default_profile = String::from("nonexistent");
        assert!(config.validate().is_err());

        // Test invalid parallel jobs
        config.default_profile = String::from("local");
        config.profiles.get_mut("local").unwrap().parallel_jobs = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        let config_path_str = config_path.to_str().unwrap();

        let mut config = Config::default();
        if let Some(profile) = config.profiles.get_mut("local") {
            profile.log_level = String::from("trace");
        }

        let json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(config_path_str, json).unwrap();

        let loaded = Config::load(config_path_str).unwrap();
        assert_eq!(loaded.profiles["local"].log_level, "trace");
    }

    #[test]
    fn test_env_override() {
        std::env::set_var(env_vars::PROFILE, "ci");
        std::env::set_var(env_vars::OUTPUT_DIR, "/custom/output");

        let mut config = Config::default();
        config.merge_env().unwrap();

        assert_eq!(config.default_profile, "ci");
        assert_eq!(
            config.profiles["ci"].output_dir,
            PathBuf::from("/custom/output")
        );

        // Clean up
        std::env::remove_var(env_vars::PROFILE);
        std::env::remove_var(env_vars::OUTPUT_DIR);
    }

    #[test]
    fn test_yaml_config_loading() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.yaml");
        let config_path_str = config_path.to_str().unwrap();

        let yaml = r#"
default_profile: production
profiles:
  production:
    output_dir: "/var/output"
    log_level: "error"
    parallel_jobs: 16
  dev:
    output_dir: "./dev-out"
    log_level: "trace"
    parallel_jobs: 2
"#;

        fs::write(config_path_str, yaml).unwrap();

        let loaded = Config::load(config_path_str).unwrap();
        assert_eq!(loaded.default_profile, "production");
        assert_eq!(loaded.profiles.len(), 2);
        assert_eq!(loaded.profiles["production"].parallel_jobs, 16);
        assert_eq!(loaded.profiles["dev"].log_level, "trace");
    }

    #[test]
    fn test_yml_extension() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.yml");
        let config_path_str = config_path.to_str().unwrap();

        let yaml = r#"
default_profile: local
profiles:
  local:
    output_dir: "./out"
    log_level: "info"
    parallel_jobs: 2
"#;

        fs::write(config_path_str, yaml).unwrap();

        let loaded = Config::load(config_path_str).unwrap();
        assert_eq!(loaded.default_profile, "local");
        assert_eq!(loaded.profiles["local"].parallel_jobs, 2);
    }
}
