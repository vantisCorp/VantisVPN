//! # Tunnel State Management
//! 
//! Defines tunnel states and state transitions.

use std::fmt;

/// Tunnel connection state
/// 
/// Represents the current state of a VPN tunnel connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TunnelState {
    /// Disconnected
    /// 
    /// Tunnel is not connected.
    #[default]
    Disconnected,
    /// Connecting
    /// 
    /// Tunnel is establishing a connection.
    Connecting,
    /// Connected
    /// 
    /// Tunnel is connected and operational.
    Connected,
    /// Disconnecting
    /// 
    /// Tunnel is in the process of disconnecting.
    Disconnecting,
    /// Reconnecting
    /// 
    /// Tunnel is reconnecting after a disconnection.
    Reconnecting,
    /// Error
    /// 
    /// Tunnel encountered an error.
    Error,
}

impl fmt::Display for TunnelState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disconnected => write!(f, "Disconnected"),
            Self::Connecting => write!(f, "Connecting"),
            Self::Connected => write!(f, "Connected"),
            Self::Disconnecting => write!(f, "Disconnecting"),
            Self::Reconnecting => write!(f, "Reconnecting"),
            Self::Error => write!(f, "Error"),
        }
    }
}

/// State transition
/// 
/// Represents a transition between tunnel states.
/// StateTransition validates that tunnel state changes follow the correct
/// state machine rules and prevents invalid transitions.
pub struct StateTransition {
    /// The starting state before the transition
    /// 
    /// The state before the transition.
    from: TunnelState,
    /// The target state after the transition
    /// 
    /// The state after the transition.
    to: TunnelState,
}

impl StateTransition {
    /// Check if transition is valid
    pub fn is_valid(&self) -> bool {
        matches!(
            (self.from, self.to),
            // Valid transitions
            (TunnelState::Disconnected, TunnelState::Connecting) |
            (TunnelState::Connecting, TunnelState::Connected) |
            (TunnelState::Connecting, TunnelState::Error) |
            (TunnelState::Connected, TunnelState::Disconnecting) |
            (TunnelState::Connected, TunnelState::Reconnecting) |
            (TunnelState::Disconnecting, TunnelState::Disconnected) |
            (TunnelState::Reconnecting, TunnelState::Connecting) |
            (TunnelState::Error, TunnelState::Disconnected) |
            (TunnelState::Error, TunnelState::Reconnecting)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_display() {
        assert_eq!(TunnelState::Disconnected.to_string(), "Disconnected");
        assert_eq!(TunnelState::Connecting.to_string(), "Connecting");
        assert_eq!(TunnelState::Connected.to_string(), "Connected");
    }

    #[test]
    fn test_valid_transitions() {
        let transition = StateTransition {
            from: TunnelState::Disconnected,
            to: TunnelState::Connecting,
        };
        assert!(transition.is_valid());
        
        let transition = StateTransition {
            from: TunnelState::Connected,
            to: TunnelState::Connected,
        };
        assert!(!transition.is_valid());
    }
}