// Theme Manager - Dark/Light Mode with Haptics
// Phase 6: UX/UI & Additional Features
// Manages UI themes and haptic feedback

use crate::error::VantisError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Theme mode
///
/// Available theme modes for the user interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    /// Light theme
    ///
    /// Light color scheme for the interface.
    Light,
    /// Dark theme
    ///
    /// Dark color scheme for the interface.
    Dark,
    /// Auto (follows system)
    ///
    /// Automatically switch between light and dark based on system preferences.
    Auto,
}

/// Haptic feedback type
///
/// Types of haptic feedback available for user interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HapticType {
    /// Light tap
    ///
    /// Light vibration for subtle feedback.
    Light,
    /// Medium tap
    ///
    /// Medium vibration for standard feedback.
    Medium,
    /// Heavy tap
    ///
    /// Strong vibration for important feedback.
    Heavy,
    /// Success vibration
    ///
    /// Vibration pattern indicating success.
    Success,
    /// Error vibration
    ///
    /// Vibration pattern indicating error.
    Error,
    /// Warning vibration
    ///
    /// Vibration pattern indicating warning.
    Warning,
    /// Custom pattern
    ///
    /// Custom haptic pattern defined by the user.
    Custom,
}

/// Haptic pattern
///
/// Defines a custom haptic feedback pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticPattern {
    /// Pattern ID
    ///
    /// Unique identifier for this pattern.
    pub pattern_id: String,
    /// Pattern name
    ///
    /// Human-readable name for this pattern.
    pub name: String,
    /// Durations in milliseconds
    ///
    /// Duration of each vibration in the pattern.
    pub durations: Vec<u32>,
    /// Intensities (0-1)
    ///
    /// Intensity of each vibration (0.0 to 1.0).
    pub intensities: Vec<f64>,
}

/// Theme configuration
///
/// Configuration settings for the theme manager.
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    /// Current theme mode (Light, Dark, Auto)
    ///
    /// The currently selected theme mode.
    pub theme_mode: ThemeMode,
    /// Enable haptic feedback
    ///
    /// Whether haptic feedback is enabled.
    pub enable_haptics: bool,
    /// Haptic intensity (0.0-1.0)
    ///
    /// Global intensity for haptic feedback (0.0 to 1.0).
    pub haptic_intensity: f64,
    /// Enable animations
    ///
    /// Whether UI animations are enabled.
    pub enable_animations: bool,
    /// Animation duration in milliseconds
    ///
    /// Default duration for UI animations.
    pub animation_duration: u32,
    /// Enable transitions
    ///
    /// Whether UI transitions are enabled.
    pub enable_transitions: bool,
    /// Transition duration in milliseconds
    ///
    /// Default duration for UI transitions.
    pub transition_duration: u32,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Auto,
            enable_haptics: true,
            haptic_intensity: 0.7,
            enable_animations: true,
            animation_duration: 300,
            enable_transitions: true,
            transition_duration: 200,
        }
    }
}

/// Theme colors
///
/// Color scheme for a theme mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    /// Primary color
    ///
    /// Primary brand color used for buttons and highlights.
    pub primary: String,
    /// Secondary color
    ///
    /// Secondary color for accents and secondary elements.
    pub secondary: String,
    /// Background color
    ///
    /// Background color for the interface.
    pub background: String,
    /// Surface color
    ///
    /// Surface color for cards and elevated elements.
    pub surface: String,
    /// Text color
    ///
    /// Primary text color.
    pub text: String,
    /// Error color
    ///
    /// Color for error messages and indicators.
    pub error: String,
    /// Success color
    ///
    /// Color for success messages and indicators.
    pub success: String,
    /// Warning color
    ///
    /// Color for warning messages and indicators.
    pub warning: String,
}

/// Theme Manager - Dark/Light Mode with Haptics
///
/// Manages UI themes (light/dark/auto) and haptic feedback patterns.
pub struct ThemeManager {
    config: ThemeConfig,
    light_theme: ThemeColors,
    dark_theme: ThemeColors,
    haptic_patterns: Arc<Mutex<HashMap<String, HapticPattern>>>,
    current_theme: Arc<Mutex<ThemeMode>>,
}

impl ThemeManager {
    /// Create a new Theme Manager instance
    pub fn new(config: ThemeConfig) -> Self {
        let light_theme = ThemeColors {
            primary: "#007AFF".to_string(),
            secondary: "#5856D6".to_string(),
            background: "#FFFFFF".to_string(),
            surface: "#F2F2F7".to_string(),
            text: "#000000".to_string(),
            error: "#FF3B30".to_string(),
            success: "#34C759".to_string(),
            warning: "#FF9500".to_string(),
        };

        let dark_theme = ThemeColors {
            primary: "#0A84FF".to_string(),
            secondary: "#5E5CE6".to_string(),
            background: "#000000".to_string(),
            surface: "#1C1C1E".to_string(),
            text: "#FFFFFF".to_string(),
            error: "#FF453A".to_string(),
            success: "#30D158".to_string(),
            warning: "#FF9F0A".to_string(),
        };

        let mut haptic_patterns = HashMap::new();
        haptic_patterns.insert(
            "success".to_string(),
            HapticPattern {
                pattern_id: "success".to_string(),
                name: "Success".to_string(),
                durations: vec![50, 50, 50],
                intensities: vec![0.5, 0.7, 0.5],
            },
        );
        haptic_patterns.insert(
            "error".to_string(),
            HapticPattern {
                pattern_id: "error".to_string(),
                name: "Error".to_string(),
                durations: vec![100, 50, 100],
                intensities: vec![0.8, 0.3, 0.8],
            },
        );
        haptic_patterns.insert(
            "warning".to_string(),
            HapticPattern {
                pattern_id: "warning".to_string(),
                name: "Warning".to_string(),
                durations: vec![75],
                intensities: vec![0.6],
            },
        );

        Self {
            config,
            light_theme,
            dark_theme,
            haptic_patterns: Arc::new(Mutex::new(haptic_patterns)),
            current_theme: Arc::new(Mutex::new(ThemeMode::Auto)),
        }
    }

    /// Set theme mode
    pub async fn set_theme_mode(&self, mode: ThemeMode) {
        let mut current = self.current_theme.lock().await;
        *current = mode;
    }

    /// Get current theme mode
    pub async fn get_theme_mode(&self) -> ThemeMode {
        let current = self.current_theme.lock().await;
        *current
    }

    /// Get current theme colors
    pub async fn get_theme_colors(&self) -> ThemeColors {
        let mode = self.get_theme_mode().await;
        match mode {
            ThemeMode::Light => self.light_theme.clone(),
            ThemeMode::Dark => self.dark_theme.clone(),
            ThemeMode::Auto => {
                // In production, check system theme
                // For now, default to dark
                self.dark_theme.clone()
            },
        }
    }

    /// Trigger haptic feedback
    pub async fn trigger_haptic(&self, haptic_type: HapticType) -> Result<(), VantisError> {
        if !self.config.enable_haptics {
            return Ok(());
        }

        let pattern = match haptic_type {
            HapticType::Light => HapticPattern {
                pattern_id: "light".to_string(),
                name: "Light".to_string(),
                durations: vec![30],
                intensities: vec![0.4 * self.config.haptic_intensity],
            },
            HapticType::Medium => HapticPattern {
                pattern_id: "medium".to_string(),
                name: "Medium".to_string(),
                durations: vec![50],
                intensities: vec![0.6 * self.config.haptic_intensity],
            },
            HapticType::Heavy => HapticPattern {
                pattern_id: "heavy".to_string(),
                name: "Heavy".to_string(),
                durations: vec![100],
                intensities: vec![0.8 * self.config.haptic_intensity],
            },
            HapticType::Success => {
                let patterns = self.haptic_patterns.lock().await;
                patterns
                    .get("success")
                    .cloned()
                    .unwrap_or_else(|| HapticPattern {
                        pattern_id: "success".to_string(),
                        name: "Success".to_string(),
                        durations: vec![50, 50, 50],
                        intensities: vec![0.5, 0.7, 0.5],
                    })
            },
            HapticType::Error => {
                let patterns = self.haptic_patterns.lock().await;
                patterns
                    .get("error")
                    .cloned()
                    .unwrap_or_else(|| HapticPattern {
                        pattern_id: "error".to_string(),
                        name: "Error".to_string(),
                        durations: vec![100, 50, 100],
                        intensities: vec![0.8, 0.3, 0.8],
                    })
            },
            HapticType::Warning => {
                let patterns = self.haptic_patterns.lock().await;
                patterns
                    .get("warning")
                    .cloned()
                    .unwrap_or_else(|| HapticPattern {
                        pattern_id: "warning".to_string(),
                        name: "Warning".to_string(),
                        durations: vec![75],
                        intensities: vec![0.6],
                    })
            },
            HapticType::Custom => {
                return Err(VantisError::InvalidData(
                    "Custom haptic pattern not specified".to_string(),
                ));
            },
        };

        // In production, this would trigger actual haptic feedback
        // For now, just log the pattern
        println!("Haptic feedback: {:?}", pattern);

        Ok(())
    }

    /// Add custom haptic pattern
    pub async fn add_haptic_pattern(&self, pattern: HapticPattern) -> Result<(), VantisError> {
        let mut patterns = self.haptic_patterns.lock().await;
        patterns.insert(pattern.pattern_id.clone(), pattern);
        Ok(())
    }

    /// Remove haptic pattern
    pub async fn remove_haptic_pattern(&self, pattern_id: &str) -> Result<(), VantisError> {
        let mut patterns = self.haptic_patterns.lock().await;
        patterns.remove(pattern_id);
        Ok(())
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: ThemeConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &ThemeConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager_creation() {
        let config = ThemeConfig::default();
        let manager = ThemeManager::new(config);
        assert_eq!(manager.config.theme_mode, ThemeMode::Auto);
    }

    #[test]
    fn test_theme_colors() {
        let config = ThemeConfig::default();
        let manager = ThemeManager::new(config);

        let light_colors = manager.light_theme.clone();
        assert_eq!(light_colors.background, "#FFFFFF");

        let dark_colors = manager.dark_theme.clone();
        assert_eq!(dark_colors.background, "#000000");
    }
}
