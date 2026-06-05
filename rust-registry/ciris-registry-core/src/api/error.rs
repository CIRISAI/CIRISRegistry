//! CEG 0.2 §10.0 / §10.0.1 — unified HTTP error envelope + request-context middleware.
//!
//! Every error response serializes to the normative §10.0.1 shape:
//! ```json
//! { "error": { "code": "<ENUM>", "http_status": <int>,
//!              "message": "<human>", "request_id": "<uuid>",
//!              "details": { ... } } }
//! ```
//! Every response (success or error) carries the §10.0 headers
//! `CEG-Version`, `X-CEG-Server-Time`, and `X-Request-Id` via
//! [`request_context_mw`]. The request-id is generated once per request,
//! made available to error construction through a task-local, and echoed
//! in both the error body and the `X-Request-Id` header.

use axum::{
    extract::Request,
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

/// Current CEG spec version this Registry conforms to (tracks
/// `FSD/CEG/README.md`). Emitted as the `CEG-Version` header per §10.0.
pub const CEG_VERSION: &str = "0.10";

tokio::task_local! {
    /// Per-request id, set by [`request_context_mw`] and read by
    /// [`ApiError::into_response`] so every error body carries it without
    /// threading it through handler signatures.
    pub static REQUEST_ID: String;
}

/// The §10.0.1 error-code enum (closed set).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    MalformedRequest,
    CanonicalBytesViolation,
    Unauthenticated,
    ReservedPrefixViolation,
    UnknownWitness,
    NotFound,
    IdempotentConflict,
    SignatureVerificationFailed,
    ClockSkewViolation,
    WitnessQuorumNotMet,
    RateLimited,
    InternalError,
    WitnessDirectoryUnavailable,
}

impl ErrorCode {
    /// The wire string per §10.0.1.
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCode::MalformedRequest => "MALFORMED_REQUEST",
            ErrorCode::CanonicalBytesViolation => "CANONICAL_BYTES_VIOLATION",
            ErrorCode::Unauthenticated => "UNAUTHENTICATED",
            ErrorCode::ReservedPrefixViolation => "RESERVED_PREFIX_VIOLATION",
            ErrorCode::UnknownWitness => "UNKNOWN_WITNESS",
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::IdempotentConflict => "IDEMPOTENT_CONFLICT",
            ErrorCode::SignatureVerificationFailed => "SIGNATURE_VERIFICATION_FAILED",
            ErrorCode::ClockSkewViolation => "CLOCK_SKEW_VIOLATION",
            ErrorCode::WitnessQuorumNotMet => "WITNESS_QUORUM_NOT_MET",
            ErrorCode::RateLimited => "RATE_LIMITED",
            ErrorCode::InternalError => "INTERNAL_ERROR",
            ErrorCode::WitnessDirectoryUnavailable => "WITNESS_DIRECTORY_UNAVAILABLE",
        }
    }

    /// The default HTTP status for this code per the §10.0.1 table.
    pub fn http_status(self) -> StatusCode {
        match self {
            ErrorCode::MalformedRequest | ErrorCode::CanonicalBytesViolation => {
                StatusCode::BAD_REQUEST
            }
            ErrorCode::Unauthenticated => StatusCode::UNAUTHORIZED,
            ErrorCode::ReservedPrefixViolation => StatusCode::FORBIDDEN,
            ErrorCode::UnknownWitness | ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::IdempotentConflict => StatusCode::CONFLICT,
            ErrorCode::SignatureVerificationFailed
            | ErrorCode::ClockSkewViolation
            | ErrorCode::WitnessQuorumNotMet => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorCode::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            ErrorCode::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::WitnessDirectoryUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// The default code for a bare HTTP status — used by
    /// [`ApiError::from_status`] so existing `(StatusCode, ...)` error
    /// sites convert mechanically while still emitting a conformant code.
    /// Sites needing a more specific code (e.g. `CANONICAL_BYTES_VIOLATION`
    /// for a 400, `UNKNOWN_WITNESS` for a 404) use the typed constructors.
    pub fn from_status(status: StatusCode) -> Self {
        match status {
            StatusCode::BAD_REQUEST => ErrorCode::MalformedRequest,
            StatusCode::UNAUTHORIZED => ErrorCode::Unauthenticated,
            StatusCode::FORBIDDEN => ErrorCode::ReservedPrefixViolation,
            StatusCode::NOT_FOUND => ErrorCode::NotFound,
            StatusCode::CONFLICT => ErrorCode::IdempotentConflict,
            StatusCode::UNPROCESSABLE_ENTITY => ErrorCode::SignatureVerificationFailed,
            StatusCode::TOO_MANY_REQUESTS => ErrorCode::RateLimited,
            StatusCode::SERVICE_UNAVAILABLE => ErrorCode::WitnessDirectoryUnavailable,
            _ => ErrorCode::InternalError,
        }
    }
}

/// A §10.0.1-conformant HTTP error. Construct via the typed helpers or
/// [`ApiError::from_status`]; serializes through [`IntoResponse`].
#[derive(Debug)]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Value,
    /// Override the code's default status (rare; e.g. a code reused at a
    /// non-default status). `None` → `code.http_status()`.
    pub status_override: Option<StatusCode>,
}

impl ApiError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        ApiError {
            code,
            message: message.into(),
            details: Value::Null,
            status_override: None,
        }
    }

    /// Convert a legacy `(StatusCode, message)` error site to a conformant
    /// `ApiError`, inferring the §10.0.1 code from the status. Preserves the
    /// original status so behavior is byte-for-byte on the status line.
    pub fn from_status(status: StatusCode, message: impl Into<String>) -> Self {
        ApiError {
            code: ErrorCode::from_status(status),
            message: message.into(),
            details: Value::Null,
            status_override: Some(status),
        }
    }

    pub fn with_details(mut self, details: Value) -> Self {
        self.details = details;
        self
    }

    // ── typed convenience constructors ───────────────────────────────
    pub fn malformed(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::MalformedRequest, message)
    }
    pub fn canonical_bytes(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::CanonicalBytesViolation, message)
    }
    pub fn unauthenticated(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Unauthenticated, message)
    }
    pub fn reserved_prefix(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ReservedPrefixViolation, message)
    }
    pub fn unknown_witness(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::UnknownWitness, message)
    }
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::NotFound, message)
    }
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::IdempotentConflict, message)
    }
    pub fn signature_failed(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::SignatureVerificationFailed, message)
    }
    pub fn rate_limited(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::RateLimited, message)
    }
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InternalError, message)
    }
    pub fn witness_directory_unavailable(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::WitnessDirectoryUnavailable, message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_override.unwrap_or_else(|| self.code.http_status());
        let request_id = REQUEST_ID
            .try_with(|id| id.clone())
            .unwrap_or_default();
        let body = json!({
            "error": {
                "code": self.code.as_str(),
                "http_status": status.as_u16(),
                "message": self.message,
                "request_id": request_id,
                "details": self.details,
            }
        });
        (status, Json(body)).into_response()
    }
}

/// Current wall-clock UTC in RFC 3339 millisecond form per §0.5
/// (`YYYY-MM-DDTHH:MM:SS.sssZ`). Used for the `X-CEG-Server-Time` header.
fn server_time_rfc3339_millis() -> String {
    let now = time::OffsetDateTime::now_utc();
    // Millisecond precision, uppercase Z, per §0.5.
    let ms = now.millisecond();
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        now.year(),
        now.month() as u8,
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        ms,
    )
}

/// Request-context middleware (§10.0): generate a per-request id, expose
/// it to handlers/errors via the [`REQUEST_ID`] task-local, and set the
/// `X-Request-Id`, `CEG-Version`, and `X-CEG-Server-Time` response headers
/// on every response (success or error).
pub async fn request_context_mw(req: Request, next: Next) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();
    let rid_for_scope = request_id.clone();
    let mut resp = REQUEST_ID
        .scope(rid_for_scope, next.run(req))
        .await;

    let headers = resp.headers_mut();
    if let Ok(v) = HeaderValue::from_str(&request_id) {
        headers.insert("X-Request-Id", v);
    }
    headers.insert("CEG-Version", HeaderValue::from_static(CEG_VERSION));
    if let Ok(v) = HeaderValue::from_str(&server_time_rfc3339_millis()) {
        headers.insert("X-CEG-Server-Time", v);
    }
    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_strings_match_spec() {
        assert_eq!(ErrorCode::MalformedRequest.as_str(), "MALFORMED_REQUEST");
        assert_eq!(
            ErrorCode::WitnessDirectoryUnavailable.as_str(),
            "WITNESS_DIRECTORY_UNAVAILABLE"
        );
        assert_eq!(
            ErrorCode::SignatureVerificationFailed.as_str(),
            "SIGNATURE_VERIFICATION_FAILED"
        );
    }

    #[test]
    fn status_mapping_matches_spec() {
        assert_eq!(ErrorCode::MalformedRequest.http_status(), StatusCode::BAD_REQUEST);
        assert_eq!(ErrorCode::Unauthenticated.http_status(), StatusCode::UNAUTHORIZED);
        assert_eq!(ErrorCode::ReservedPrefixViolation.http_status(), StatusCode::FORBIDDEN);
        assert_eq!(ErrorCode::NotFound.http_status(), StatusCode::NOT_FOUND);
        assert_eq!(ErrorCode::IdempotentConflict.http_status(), StatusCode::CONFLICT);
        assert_eq!(
            ErrorCode::ClockSkewViolation.http_status(),
            StatusCode::UNPROCESSABLE_ENTITY
        );
        assert_eq!(ErrorCode::RateLimited.http_status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(ErrorCode::InternalError.http_status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            ErrorCode::WitnessDirectoryUnavailable.http_status(),
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[test]
    fn from_status_roundtrips_default_codes() {
        assert_eq!(ErrorCode::from_status(StatusCode::BAD_REQUEST), ErrorCode::MalformedRequest);
        assert_eq!(ErrorCode::from_status(StatusCode::NOT_FOUND), ErrorCode::NotFound);
        assert_eq!(ErrorCode::from_status(StatusCode::CONFLICT), ErrorCode::IdempotentConflict);
        assert_eq!(
            ErrorCode::from_status(StatusCode::IM_A_TEAPOT),
            ErrorCode::InternalError
        );
    }

    #[test]
    fn server_time_is_millisecond_rfc3339() {
        let t = server_time_rfc3339_millis();
        // YYYY-MM-DDTHH:MM:SS.sssZ — 24 chars, ends in Z, '.' at index 19
        assert_eq!(t.len(), 24, "got {t}");
        assert!(t.ends_with('Z'));
        assert_eq!(&t[19..20], ".");
    }
}
