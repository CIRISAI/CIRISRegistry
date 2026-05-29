//! Input validators shared across HTTP and gRPC entry points.

/// Maximum length of a project name (CIRIS primitive identifier).
pub const MAX_PROJECT_NAME_LEN: usize = 64;

/// Validate a project name (e.g., "ciris-agent", "ciris-persist", "ciris-lens").
///
/// Empty is allowed and means "caller did not specify; use default" — the DB
/// layer substitutes `ciris-agent`. Non-empty values must match
/// `^[a-z][a-z0-9-]{0,63}$`.
///
/// Returns `Ok(())` on valid input (or empty), `Err(String)` with a
/// human-readable reason otherwise.
pub fn validate_project_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Ok(());
    }
    if name.len() > MAX_PROJECT_NAME_LEN {
        return Err(format!(
            "project name must be {} characters or fewer (got {})",
            MAX_PROJECT_NAME_LEN,
            name.len()
        ));
    }
    let bytes = name.as_bytes();
    if !bytes[0].is_ascii_lowercase() {
        return Err("project name must start with [a-z]".to_string());
    }
    for &b in bytes {
        if !(b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-') {
            return Err("project name must match [a-z][a-z0-9-]*".to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_ok() {
        assert!(validate_project_name("").is_ok());
    }

    #[test]
    fn canonical_names_pass() {
        for name in [
            "ciris-agent",
            "ciris-persist",
            "ciris-lens",
            "ciris-verify",
            "ciris-registry",
            "x",
            "abc-123",
        ] {
            assert!(validate_project_name(name).is_ok(), "expected ok for {}", name);
        }
    }

    #[test]
    fn rejects_uppercase() {
        assert!(validate_project_name("CIRIS-Agent").is_err());
    }

    #[test]
    fn rejects_leading_digit() {
        assert!(validate_project_name("1ciris").is_err());
    }

    #[test]
    fn rejects_underscore() {
        assert!(validate_project_name("ciris_agent").is_err());
    }

    #[test]
    fn rejects_dot() {
        assert!(validate_project_name("ciris.agent").is_err());
    }

    #[test]
    fn rejects_too_long() {
        let long = "a".repeat(MAX_PROJECT_NAME_LEN + 1);
        assert!(validate_project_name(&long).is_err());
    }

    #[test]
    fn accepts_max_length() {
        let max = "a".repeat(MAX_PROJECT_NAME_LEN);
        assert!(validate_project_name(&max).is_ok());
    }
}
