-- Bring `api_keys` time columns in line with every other arium `*_at`
-- column (expires_at, occurred_at, mfa_enabled_at, deleted_at, …): BIGINT
-- unix-epoch seconds, which is what the code has always bound and read
-- (`unix_now()` / `format_unix_date` / `i64` columns in
-- `auth::tokens::{create_for_user,list_for_user,revoke_for_user}`).
--
-- 0007 declared these TIMESTAMPTZ, so on Postgres every token write failed
-- (`column "created_at" is of type timestamp with time zone but expression
-- is of type bigint`) and every list failed to decode timestamptz into i64.
-- SQLite slipped through because its column affinity stored the bound i64
-- as-is; CI never exercises the Postgres query path. Convert in place,
-- preserving each row's instant via EXTRACT(EPOCH …).
ALTER TABLE api_keys ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE api_keys ALTER COLUMN created_at   TYPE BIGINT USING EXTRACT(EPOCH FROM created_at)::BIGINT;
ALTER TABLE api_keys ALTER COLUMN last_used_at TYPE BIGINT USING EXTRACT(EPOCH FROM last_used_at)::BIGINT;
ALTER TABLE api_keys ALTER COLUMN revoked_at   TYPE BIGINT USING EXTRACT(EPOCH FROM revoked_at)::BIGINT;
