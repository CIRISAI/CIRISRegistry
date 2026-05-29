//! Edge transport substrate — CEG 0.2 §10.1 read-side helpers.
//!
//! As of v1.3.0-rc.1 (#33 Phase 3), this module ships the pure-logic
//! helpers for:
//!
//! - **§10.1.1** — Full-SHA verification of received content bytes before
//!   consumption (see [`verify::verify_content_body_sha256`]).
//! - **§10.1.2** — 24-hour TTL discipline on `holds_bytes:sha256:{prefix}`
//!   attestations + ContentMiss-feedback emission (see [`ttl`] +
//!   [`content_miss`]).
//! - **§8.1.6** — Three-layer `agent_files:*` trust composition (Canonical
//!   / Open / Vote-then-trust; see [`agent_files`]).
//!
//! What this module does NOT do (deferred to v1.3.0 final):
//!
//! - Engine construction at boot in `main.rs`. The helpers here take
//!   abstract inputs (`&[Attestation]`, `Arc<dyn FederationDirectory>`,
//!   etc.); they don't construct an Engine themselves.
//! - Lighting up `/v1/agent_files/{kind}` to query live data through
//!   these helpers. That requires Engine in AppState which is the
//!   Phase-2-follow-up.
//! - Implementing `ciris_edge::transport::reticulum::PeerResolver` to
//!   serve PeerResolver queries from Registry. Registry is the substrate
//!   that *publishes* attestations; the resolver implementation belongs
//!   to consumers (CIRISAgent, CIRISVerify clients) who fetch bytes.
//!
//! ## Design discipline
//!
//! - Pure-logic helpers (TTL, SHA verify, three-layer composition) take
//!   their inputs by reference and return decisions; no I/O.
//! - Helpers that need I/O (ContentMiss-feedback emission) take an
//!   `&dyn FederationDirectory` so they can be tested against any
//!   federation client implementation.
//! - The module surface is shaped so the Phase-2-follow-up's endpoint
//!   integration is a straight-line plumbing exercise — fetch
//!   attestations, call the pure helper, return the result.

pub mod agent_files;
pub mod content_miss;
pub mod ttl;
pub mod verify;

pub use agent_files::{
    compose_trust_layers, AgentFilesTrustComposition, OpenAttester, VoteThenTrustAttester,
};
pub use content_miss::{emit_content_miss_withdraws, ContentMissError};
pub use ttl::{filter_fresh_holders, HoldsBytesFreshness, DEFAULT_HOLDS_BYTES_TTL_SECONDS};
pub use verify::{verify_content_body_sha256, ContentBodyVerifyError};
