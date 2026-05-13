-- Migration 026: Document canonical audit table contract
--
-- Closes:
--   - GitHub issue CIRISAI/CIRISRegistry#6 (audit_log vs audit_logs:
--     clarify which table is canonical).
--
-- Background. CIRISRegistry has two audit tables that look like a
-- typo'd pair:
--
--   - `audit_log`  (singular) — canonical, runtime-written. Every
--     `create_audit_entry` call in `src/db/audit.rs` writes here.
--     This is the table operators should query.
--
--   - `audit_logs` (plural) — historical trigger target. Migration 005
--     created it as a view; mig 007 promoted it to a real table to
--     accept the wider trigger schema; mig 009 dropped every trigger
--     that wrote to it. Since 009 it has been empty fleet-wide
--     (verified across US + EU on 2026-05-03). Retained as a
--     trigger-target dead-letter in case operator-added triggers
--     reappear; never queried by the runtime.
--
-- This migration attaches PostgreSQL `COMMENT ON TABLE` strings so
-- `\d+ audit_log` / `\d+ audit_logs` in psql tells the operator which
-- one to query. Non-destructive — audit_logs is left in place so any
-- inadvertent trigger that resumes writing to it doesn't error out
-- silently. A future migration may DROP audit_logs once we're
-- confident no replication node carries operator-added triggers.
--
-- Replication: COMMENT statements are DDL — Spock excludes
-- _sqlx_migrations from replication, and each node executes its own
-- migrations. No repset_add_table call needed (this migration adds
-- no tables).
--
-- Idempotency: COMMENT ON TABLE is naturally idempotent (replaces the
-- existing comment string).

COMMENT ON TABLE audit_log IS
    'Canonical runtime audit table. All db::audit::create_audit_entry writes land here. Query this table for incident response and compliance. See migrations 001, 004, 006 for schema evolution.';

COMMENT ON TABLE audit_logs IS
    'DEAD-LETTER trigger target. Empty fleet-wide since migration 009 dropped the legacy table-level audit triggers. Retained as a landing pad in case operator-added triggers resume writing — runtime code does NOT read or write this table. For audit queries, use audit_log (singular).';
