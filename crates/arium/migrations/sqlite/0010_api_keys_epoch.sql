-- SQLite mirror of the Postgres 0010. SQLite columns are NUMERIC-affinity, so
-- the bound i64 epochs already store and read back as integers — the table
-- never broke here the way Postgres did. The one inconsistency: the old
-- `authenticate_token` bump wrote `last_used_at = CURRENT_TIMESTAMP` (a TEXT
-- datetime), which then failed to decode as i64 in `list_for_user`. Normalize
-- any such legacy TEXT values to unix-epoch seconds. No-op on fresh DBs and on
-- rows already holding integers.
UPDATE api_keys SET created_at   = CAST(strftime('%s', created_at)   AS INTEGER) WHERE typeof(created_at)   = 'text';
UPDATE api_keys SET last_used_at = CAST(strftime('%s', last_used_at) AS INTEGER) WHERE typeof(last_used_at) = 'text';
UPDATE api_keys SET revoked_at   = CAST(strftime('%s', revoked_at)   AS INTEGER) WHERE typeof(revoked_at)   = 'text';
