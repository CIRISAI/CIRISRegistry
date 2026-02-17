-- Migration 008: Add public contact / responsible party fields
-- These fields support CIRISVerify "insurance" accountability:
-- Every licensed deployment must identify who is responsible and how to contact them.

-- Organization: public-facing contact email (defaults to primary_email via app logic)
ALTER TABLE organizations
  ADD COLUMN IF NOT EXISTS public_contact_email TEXT;

COMMENT ON COLUMN organizations.public_contact_email IS
  'Public-facing org contact email. Defaults to primary_email if empty. Embedded in license JWTs as responsible_party_contact.';

-- Partner: responsible party name and public contact
ALTER TABLE partners
  ADD COLUMN IF NOT EXISTS responsible_party TEXT DEFAULT '',
  ADD COLUMN IF NOT EXISTS public_contact_email TEXT;

COMMENT ON COLUMN partners.responsible_party IS
  'Name of the human accountable for deployments under this partner license.';
COMMENT ON COLUMN partners.public_contact_email IS
  'Public-facing contact email for this partner. Defaults to org primary_email if empty.';
