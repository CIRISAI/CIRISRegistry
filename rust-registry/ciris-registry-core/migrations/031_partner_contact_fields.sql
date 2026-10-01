-- 031: partners.responsible_party + partners.public_contact_email
--
-- Commit ebe637a added both fields to the proto and to every partner query
-- (db/partners.rs selects and inserts them), but put the ALTER only in the
-- legacy `database/migrations/008_public_contact_fields.sql` tree. This tree —
-- the one `sqlx::migrate!("./migrations")` actually runs — never got it. So on
-- any database the binary migrates itself, `lookup_partner` fails with
-- `column "responsible_party" does not exist`: gRPC LookupPartner errors, and
-- `GET /v1/partner/{key_id}` silently serves an empty composition because its
-- handler discards the error. Found by booting v3.1.0 against a fresh
-- Postgres 16 and seeding partners.
--
-- Idempotent (ADD COLUMN IF NOT EXISTS): a database that already received the
-- columns from the legacy tree or by hand is untouched; one that did not gets
-- them. Nullable, no default — matches `Option<String>` in PartnerRow, so
-- pre-existing rows decode.
--
-- Replication intent: `partners` is a TRANSITIONAL legacy cross-region table
-- (Spock-enrolled where Spock is still loaded; CEG-native target is the
-- `partner_record` operational envelope, CC 3.3.9). This migration adds no
-- enrollment and no new table. DDL is per-node sqlx execution, as always.

ALTER TABLE partners ADD COLUMN IF NOT EXISTS responsible_party TEXT;
ALTER TABLE partners ADD COLUMN IF NOT EXISTS public_contact_email TEXT;
