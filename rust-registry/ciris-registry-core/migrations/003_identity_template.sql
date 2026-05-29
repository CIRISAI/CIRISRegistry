-- Identity template fields for CIRISVerify enforcement (v1.2.0)

ALTER TABLE agents ADD COLUMN IF NOT EXISTS identity_template VARCHAR(50);
ALTER TABLE agents ADD COLUMN IF NOT EXISTS stewardship_tier INTEGER;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS permitted_actions TEXT[];
ALTER TABLE agents ADD COLUMN IF NOT EXISTS template_hash BYTEA;

ALTER TABLE partners ADD COLUMN IF NOT EXISTS allowed_identity_templates TEXT[];
