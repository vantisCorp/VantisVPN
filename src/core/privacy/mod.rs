// Privacy Module - Privacy & Identity Management
// Phase 5: Privacy & Identity Management

pub mod anonymous_payments;
pub mod avantis_id;
pub mod gdpr_compliance;
pub mod ip_rotator;
pub mod zk_login;

#[cfg(test)]
mod comprehensive_tests;

// Re-exports
pub use anonymous_payments::{
    AnonymousPaymentManager, CashPayment, LightningPayment, MoneroPayment, PaymentConfig,
    PaymentMethod, PaymentStatus,
};
pub use avantis_id::{
    AvantisIdConfig, AvantisIdManager, DigitalIdentity, IdentityProof, IdentityType,
};
pub use gdpr_compliance::{
    ConsentRecord, ConsentType, DataPortability, DataRequest, DataSubject, GdprCompliance,
    GdprConfig, RightToBeForgotten,
};
pub use ip_rotator::{IpEndpoint, IpPool, IpRotator, RotationStrategy, RotatorConfig};
pub use zk_login::{
    AuthState, UserCredentials, ZkAuthResult, ZkChallenge, ZkLoginConfig, ZkLoginManager,
    ZkProofType, ZkResponse,
};
