// Privacy Module - Privacy & Identity Management
// Phase 5: Privacy & Identity Management

pub mod zk_login;
pub mod avantis_id;
pub mod ip_rotator;
pub mod anonymous_payments;
pub mod gdpr_compliance;

#[cfg(test)]
mod comprehensive_tests;

// Re-exports
pub use zk_login::{
    ZkLoginManager, ZkLoginConfig, ZkChallenge, ZkResponse, ZkAuthResult,
    UserCredentials, AuthState, ZkProofType,
};
pub use avantis_id::{
    AvantisIdManager, AvantisIdConfig, DigitalIdentity, IdentityProof,
    IdentityType,
};
pub use ip_rotator::{
    IpRotator, RotatorConfig, RotationStrategy, IpPool, IpEndpoint,
};
pub use anonymous_payments::{
    AnonymousPaymentManager, PaymentConfig, PaymentMethod,
    MoneroPayment, LightningPayment, CashPayment, PaymentStatus,
};
pub use gdpr_compliance::{
    GdprCompliance, GdprConfig, DataSubject, DataRequest, ConsentRecord,
    RightToBeForgotten, DataPortability, ConsentType,
};