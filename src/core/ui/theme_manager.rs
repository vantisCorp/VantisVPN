// Theme Manager - Dark/Light Mode with Haptics
// Phase 6: UX/UI & Additional Features
// Manages UI themes and haptic feedback

use crate::error::VantisError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

/// Theme mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// Auto (follows system)
    Auto,
}

/// Haptic feedback type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HapticType {
    /// Light tap
    Light,
    /// Medium tap
    Medium,
    /// Heavy tap
    Heavy,
    /// Success vibration
    Success,
    /// Error vibration
    Error,
    /// Warning vibration
    Warning,
    /// Custom pattern
    Custom,
}

/// Haptic pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticPattern {
    /// Pattern ID
    pub pattern_id: String,
    /// Pattern name
    pub name: String,
    /// Durations in milliseconds
    pub durations: Vec<u32>,
    /// Intensities (0-1)
    pub intensities: Vec<f64>,
}

/// Theme configuration
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    /// Current theme mode
    pub theme_mode: ThemeMode,
    /// Enable haptic feedback
    pub enable_haptics: bool,
    /// Haptic intensity (0-1)
    pub haptic_intensity: f64,
    /// Enable animations
    pub enable_animations: bool,
    /// Animation duration in milliseconds
    pub animation_duration: u32,
    /// Enable transitions
    pub enable_transitions: bool,
    /// Transition duration in milliseconds
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    /// Primary color
    pub primary: String,
    /// Secondary color
    pub secondary: String,
    /// Background color
    pub background: String,
    /// Surface color
    pub surface: String,
    /// Text color
    pub text: String,
    /// Error color
    pub error: String,
    /// Success color
    pub success: String,
    /// Warning color
    pub warning: String,
}

/// Theme Manager - Dark/Light Mode with Haptics
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
        haptic_patterns.insert("success".to_string(), HapticPattern {
            pattern_id: "success".to_string(),
            name: "Success".to_string(),
            durations: vec![50, 50, 50],
            intensities: vec![0.5, 0.7, 0.5],
        });
        haptic_patterns.insert("error".to_string(), HapticPattern {
            pattern_id: "error".to_string(),
            name: "Error".to_string(),
            durations: vec![100, 50, 100],
            intensities: vec![0.8, 0.3, 0.8],
        });
        haptic_patterns.insert("warning".to_string(), HapticPattern {
            pattern_id: "warning".to_string(),
            name: "Warning".to_string(),
            durations: vec![75],
            intensities: vec![0.6],
        });

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
            }
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
                patterns.get("success").cloned().unwrap_or_else(|| HapticPattern {
                    pattern_id: "success".to_string(),
                    name: "Success".to_string(),
                    durations: vec![50, 50, 50],
                    intensities: vec![0.5, 0.7, 0.5],
                })
            },
            HapticType::Error => {
                let patterns = self.haptic_patterns.lock().await;
                patterns.get("error").cloned().unwrap_or_else(|| HapticPattern {
                    pattern_id: "error".to_string(),
                    name: "Error".to_string(),
                    durations: vec![100, 50, 100],
                    intensities: vec![0.8, 0.3, 0.8],
                })
            },
            HapticType::Warning => {
                let patterns = self.haptic_patterns.lock().await;
                patterns.get("warning").cloned().unwrap_or_else(|| HapticPattern {
                    pattern_id: "warning".to_string(),
                    name: "Warning".to_string(),
                    durations: vec![75],
                    intensities: vec![0.6],
                })
            },
            HapticType::Custom => {
                return Err(VantisError::InvalidData("Custom haptic pattern not specified".to_string()));
            }
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