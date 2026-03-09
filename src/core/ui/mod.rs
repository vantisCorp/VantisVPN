// UI Module - UX/UI & Additional Features
// Phase 6: UX/UI & Additional Features

/// Biometric Auth module.
pub mod biometric_auth;
/// Devtunnel module.
pub mod devtunnel;
/// Family Shield module.
pub mod family_shield;
/// Theme Manager module.
pub mod theme_manager;

// Re-exports
pub use biometric_auth::{
    AuthResult, BiometricAuth, BiometricConfig, BiometricTemplate, BiometricType,
};
pub use devtunnel::{
    DevTunnel, TunnelConfig, TunnelProtocol, TunnelSession, TunnelStats, TunnelStatus,
};
pub use family_shield::{
    FamilyShield, ShieldAction, ShieldCategory, ShieldConfig, ShieldRule, ShieldStats,
};
pub use theme_manager::{
    HapticPattern, HapticType, ThemeColors, ThemeConfig, ThemeManager, ThemeMode,
};

#[cfg(test)]
mod comprehensive_tests;
