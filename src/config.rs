use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for cosmic-eyes break reminder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Short break settings
    pub short_break: BreakConfig,

    /// Long break settings
    pub long_break: BreakConfig,

    /// Whether to enable idle detection
    pub idle_detection: bool,

    /// Idle time threshold in seconds before pausing timers
    pub idle_threshold: u64,

    /// Notification time before break starts (in seconds)
    pub notification_before_break: u64,

    /// Allow skipping breaks
    pub allow_skip: bool,

    /// Allow postponing breaks
    pub allow_postpone: bool,

    /// Postpone duration in minutes
    pub postpone_duration: u64,

    /// Enable strict mode (no skip/postpone)
    pub strict_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakConfig {
    /// Interval between breaks in minutes
    pub interval: u64,

    /// Duration of the break in seconds
    pub duration: u64,

    /// Whether this break type is enabled
    pub enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            short_break: BreakConfig {
                interval: 20,  // Every 20 minutes
                duration: 20,  // 20 seconds
                enabled: true,
            },
            long_break: BreakConfig {
                interval: 60,  // Every 60 minutes (1 hour)
                duration: 300, // 5 minutes
                enabled: true,
            },
            idle_detection: true,
            idle_threshold: 300, // 5 minutes
            notification_before_break: 10, // 10 seconds warning
            allow_skip: true,
            allow_postpone: true,
            postpone_duration: 5, // 5 minutes
            strict_mode: false,
        }
    }
}

impl Config {
    /// Get the config file path
    pub fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("cosmic-eyes");

        std::fs::create_dir_all(&config_dir).ok();
        config_dir.join("config.ron")
    }

    /// Load configuration from file
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::config_path();

        if path.exists() {
            let contents = std::fs::read_to_string(&path)?;
            let config = ron::from_str(&contents)?;
            Ok(config)
        } else {
            // Create default config
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path();
        let contents = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        std::fs::write(&path, contents)?;
        Ok(())
    }
}

// Additional dependency needed - add to Cargo.toml:
// dirs = "5.0"
