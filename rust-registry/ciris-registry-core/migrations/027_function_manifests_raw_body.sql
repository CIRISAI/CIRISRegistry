-- Migration 027: Store raw BuildManifest POST body for Path B verbatim GET
--
-- Closes:
--   - CIRISRegistry#5 §2 (Path B symmetric BuildManifest GET).
--
-- Background. The `POST /v1/verify/build-manifest` handler today parses
-- the BuildManifest JSON, verifies the hybrid signature against the
-- raw POST bytes, then re-serializes through serde for storage in
-- `function_manifests.manifest_json`. This re-serialization is NOT
-- byte-faithful — serde JSON output reorders some fields, normalizes
-- whitespace, and renders numbers differently from the producer's
-- canonical form. The stored value is enough for Path A
-- (FunctionManifestResponse, which is its own derived shape) but
-- destroys the byte fidelity Path B consumers need to re-verify
-- the original CI signature against canonical bytes.
--
-- Path B (per CIRISRegistry#5 §2) requires serving the BuildManifest
-- exactly as POSTed. New column `raw_manifest_body` holds the
-- verbatim request body for rows POSTed via `/v1/verify/build-manifest`
-- (Case (i) per docs/TRUST_CONTRACT.md §2.3). Rows POSTed via the
-- legacy `/v1/verify/function-manifest` endpoint (Case (ii), server-
-- resigned) leave this column NULL — Path B is meaningless for them
-- since the original CI signature was never stored.
--
-- Backfill is intentionally NULL — historical rows predate the column
-- and don't have the raw bytes available. New POSTs starting at v1.4.2
-- populate it.
--
-- Replication: `function_manifests` is already enrolled in the
-- default repset (migration 014 + project-namespace migration 021).
-- No new repset_add_table call. The new BYTEA column rides along
-- with normal row-level replication.
--
-- Idempotency: ADD COLUMN IF NOT EXISTS. Safe to re-run.

ALTER TABLE function_manifests
    ADD COLUMN IF NOT EXISTS raw_manifest_body BYTEA;

COMMENT ON COLUMN function_manifests.raw_manifest_body IS
    'Verbatim POST body bytes from /v1/verify/build-manifest. NULL for rows POSTed via the legacy /v1/verify/function-manifest endpoint (those rows carry the registry-re-signed signature in signature_*; the original CI body was never captured). Served by GET /v1/verify/build-manifest/{project}/{version}/{target} (Path B).';
