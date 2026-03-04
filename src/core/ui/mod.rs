// UI Module - UX/UI & Additional Features
// Phase 6: UX/UI & Additional Features

pub mod devtunnel;
pub mod family_shield;
pub mod biometric_auth;
pub mod theme_manager;

// Re-exports
pub use devtunnel::{DevTunnel, TunnelConfig, TunnelSession, TunnelStats};
pub use family_shield::{FamilyShield, ShieldConfig, ShieldRule, ShieldStats};
pub use biometric_auth::{BiometricAuth, BiometricConfig, BiometricType, AuthResult};
pub use theme_manager::{ThemeManager, ThemeConfig, ThemeMode, HapticType, HapticPattern, ThemeColors};

#[cfg(test)]
mod comprehensive_tests;