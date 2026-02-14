//! Error handling for CIRISRegistry
//!
//! Maps internal errors to gRPC status codes and v1.1.0 RegistryErrorCode values

use thiserror::Error;
use tonic::Status;

use crate::proto;

#[derive(Debug, Error)]
pub enum RegistryError {
    // Agent errors (1000-1999)
    #[error("Agent not found: {0}")]
    AgentNotFound(String),

    #[error("Agent revoked: {0}")]
    AgentRevoked(String),

    #[error("Agent deprecated: {0}")]
    AgentDeprecated(String),

    #[error("Invalid agent hash: {0}")]
    InvalidAgentHash(String),

    // Partner errors (2000-2999)
    #[error("Partner not licensed: {0}")]
    PartnerNotLicensed(String),

    #[error("Partner suspended: {0}")]
    PartnerSuspended(String),

    #[error("Partner license expired: {0}")]
    PartnerExpired(String),

    #[error("License revoked: {0}")]
    LicenseRevoked(String),

    #[error("Capability denied: {0}")]
    CapabilityDenied(String),

    #[error("Autonomy tier exceeded: requested {requested}, max {max}")]
    AutonomyTierExceeded { requested: i32, max: i32 },

    // Cryptographic errors (3000-3999)
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("Signature expired")]
    SignatureExpired,

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Key revoked: {0}")]
    KeyRevoked(String),

    #[error("Key pending activation: {0}")]
    KeyPending(String),

    #[error("No active key for organization: {0}")]
    NoActiveKey(String),

    // Organization errors (4000-4999)
    #[error("Organization not found: {0}")]
    OrgNotFound(String),

    #[error("Organization inactive: {0}")]
    OrgInactive(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("User inactive: {0}")]
    UserInactive(String),

    #[error("Insufficient role: required {required}, has {has}")]
    InsufficientRole { required: String, has: String },

    // Infrastructure errors (5000-5999)
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Merkle proof invalid")]
    MerkleProofInvalid,

    #[error("Snapshot stale: {0}")]
    SnapshotStale(String),

    #[error("HSM unavailable: {0}")]
    HsmUnavailable(String),

    #[error("Webhook delivery failed: {0}")]
    WebhookDeliveryFailed(String),

    // General errors
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl RegistryError {
    /// Convert to protobuf error code
    pub fn error_code(&self) -> i32 {
        use proto::RegistryErrorCode;

        match self {
            // Agent errors
            Self::AgentNotFound(_) => RegistryErrorCode::RegistryErrorAgentNotRegistered as i32,
            Self::AgentRevoked(_) => RegistryErrorCode::RegistryErrorAgentRevoked as i32,
            Self::AgentDeprecated(_) => RegistryErrorCode::RegistryErrorAgentDeprecated as i32,
            Self::InvalidAgentHash(_) => RegistryErrorCode::RegistryErrorAgentHashInvalid as i32,

            // Partner errors
            Self::PartnerNotLicensed(_) => {
                RegistryErrorCode::RegistryErrorPartnerNotLicensed as i32
            }
            Self::PartnerSuspended(_) => RegistryErrorCode::RegistryErrorPartnerSuspended as i32,
            Self::PartnerExpired(_) => RegistryErrorCode::RegistryErrorPartnerExpired as i32,
            Self::LicenseRevoked(_) => RegistryErrorCode::RegistryErrorLicenseRevoked as i32,
            Self::CapabilityDenied(_) => RegistryErrorCode::RegistryErrorCapabilityDenied as i32,
            Self::AutonomyTierExceeded { .. } => {
                RegistryErrorCode::RegistryErrorAutonomyTierExceeded as i32
            }

            // Cryptographic errors
            Self::InvalidSignature(_) => RegistryErrorCode::RegistryErrorInvalidSignature as i32,
            Self::SignatureExpired => RegistryErrorCode::RegistryErrorSignatureExpired as i32,
            Self::KeyNotFound(_) => RegistryErrorCode::RegistryErrorKeyNotFound as i32,
            Self::KeyRevoked(_) => RegistryErrorCode::RegistryErrorKeyRevoked as i32,
            Self::KeyPending(_) => RegistryErrorCode::RegistryErrorKeyPending as i32,
            Self::NoActiveKey(_) => RegistryErrorCode::RegistryErrorNoActiveKey as i32,

            // Organization errors
            Self::OrgNotFound(_) => RegistryErrorCode::RegistryErrorOrgNotFound as i32,
            Self::OrgInactive(_) => RegistryErrorCode::RegistryErrorOrgInactive as i32,
            Self::UserNotFound(_) => RegistryErrorCode::RegistryErrorUserNotFound as i32,
            Self::UserInactive(_) => RegistryErrorCode::RegistryErrorUserInactive as i32,
            Self::InsufficientRole { .. } => {
                RegistryErrorCode::RegistryErrorInsufficientRole as i32
            }

            // Infrastructure errors
            Self::Database(_) => RegistryErrorCode::RegistryErrorDatabaseError as i32,
            Self::MerkleProofInvalid => RegistryErrorCode::RegistryErrorMerkleProofInvalid as i32,
            Self::SnapshotStale(_) => RegistryErrorCode::RegistryErrorSnapshotStale as i32,
            Self::HsmUnavailable(_) => RegistryErrorCode::RegistryErrorHsmUnavailable as i32,
            Self::WebhookDeliveryFailed(_) => {
                RegistryErrorCode::RegistryErrorWebhookDeliveryFailed as i32
            }

            // General errors
            Self::InvalidArgument(_) => RegistryErrorCode::RegistryErrorInvalidArgument as i32,
            Self::Unauthorized(_) => RegistryErrorCode::RegistryErrorUnauthorized as i32,
            Self::Forbidden(_) => RegistryErrorCode::RegistryErrorForbidden as i32,
            Self::Conflict(_) => RegistryErrorCode::RegistryErrorConflict as i32,
            Self::RateLimited => RegistryErrorCode::RegistryErrorRateLimited as i32,
            Self::Internal(_) => RegistryErrorCode::RegistryErrorInternal as i32,
            Self::ServiceUnavailable(_) => {
                RegistryErrorCode::RegistryErrorServiceUnavailable as i32
            }
        }
    }

    /// Convert to retry guidance
    pub fn retry_status(&self) -> proto::Retryable {
        match self {
            // Never retry these
            Self::AgentNotFound(_)
            | Self::AgentRevoked(_)
            | Self::PartnerNotLicensed(_)
            | Self::PartnerSuspended(_)
            | Self::PartnerExpired(_)
            | Self::LicenseRevoked(_)
            | Self::KeyRevoked(_)
            | Self::OrgNotFound(_)
            | Self::UserNotFound(_)
            | Self::InvalidArgument(_)
            | Self::Unauthorized(_)
            | Self::Forbidden(_)
            | Self::Conflict(_)
            | Self::InvalidSignature(_) => proto::Retryable::RetryNo,

            // Retry with backoff
            Self::Database(_)
            | Self::HsmUnavailable(_)
            | Self::ServiceUnavailable(_)
            | Self::WebhookDeliveryFailed(_) => proto::Retryable::RetryBackoff,

            // Retry immediately (transient)
            Self::MerkleProofInvalid | Self::SnapshotStale(_) => proto::Retryable::RetryImmediate,

            // Rate limited - retry after
            Self::RateLimited => proto::Retryable::RetryAfter,

            // Others
            _ => proto::Retryable::RetryNo,
        }
    }

    /// Create ErrorDetail protobuf message
    pub fn to_error_detail(&self) -> proto::ErrorDetail {
        proto::ErrorDetail {
            code: self.error_code(),
            message: self.to_string(),
            retry_status: self.retry_status() as i32,
            retry_after_seconds: if matches!(self, Self::RateLimited) {
                60
            } else {
                0
            },
            metadata: Default::default(),
            cause: None,
        }
    }
}

impl From<RegistryError> for Status {
    fn from(err: RegistryError) -> Status {
        use tonic::Code;

        let code = match &err {
            RegistryError::AgentNotFound(_)
            | RegistryError::PartnerNotLicensed(_)
            | RegistryError::KeyNotFound(_)
            | RegistryError::OrgNotFound(_)
            | RegistryError::UserNotFound(_) => Code::NotFound,

            RegistryError::InvalidArgument(_)
            | RegistryError::InvalidAgentHash(_)
            | RegistryError::InvalidSignature(_) => Code::InvalidArgument,

            RegistryError::Unauthorized(_) => Code::Unauthenticated,

            RegistryError::Forbidden(_)
            | RegistryError::InsufficientRole { .. }
            | RegistryError::CapabilityDenied(_)
            | RegistryError::AutonomyTierExceeded { .. } => Code::PermissionDenied,

            RegistryError::AgentRevoked(_)
            | RegistryError::PartnerSuspended(_)
            | RegistryError::PartnerExpired(_)
            | RegistryError::LicenseRevoked(_)
            | RegistryError::KeyRevoked(_)
            | RegistryError::OrgInactive(_)
            | RegistryError::UserInactive(_)
            | RegistryError::KeyPending(_)
            | RegistryError::NoActiveKey(_) => Code::FailedPrecondition,

            RegistryError::Conflict(_) => Code::AlreadyExists,

            RegistryError::RateLimited => Code::ResourceExhausted,

            RegistryError::Database(_)
            | RegistryError::Internal(_)
            | RegistryError::MerkleProofInvalid
            | RegistryError::SnapshotStale(_) => Code::Internal,

            RegistryError::HsmUnavailable(_)
            | RegistryError::ServiceUnavailable(_)
            | RegistryError::WebhookDeliveryFailed(_) => Code::Unavailable,

            _ => Code::Unknown,
        };

        Status::new(code, err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, RegistryError>;
