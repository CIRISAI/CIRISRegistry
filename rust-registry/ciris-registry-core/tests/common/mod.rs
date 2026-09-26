//! Common test utilities and fixtures for CIRISRegistry
//!
//! This module provides:
//! - Test data generators (fixtures)
//! - Proptest strategies for property-based testing
//! - Database test helpers

#[cfg(feature = "standalone")]
pub mod auth_fixtures;
pub mod fixtures;
pub mod strategies;

pub use fixtures::*;
pub use strategies::*;
