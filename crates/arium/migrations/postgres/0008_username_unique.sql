-- Postgres-flavored equivalent of the sqlite 0008 migration. See that file for
-- the full rationale: make `username` a unique, stable handle and fold the
-- provider-supplied `name` column into the user-editable `display_name`.

-- 1) Seed display_name from the provider name wherever the user hasn't set one.
UPDATE users SET display_name = name
    WHERE (display_name IS NULL OR display_name = '') AND name IS NOT NULL;

-- 2) Disambiguate any pre-existing case-insensitive username collisions before
--    the unique index goes on. Lowest id keeps the bare handle; the rest get
--    their row id appended (`alice` -> `alice-7`), which is always unique.
UPDATE users SET username = username || '-' || id
    WHERE id IN (
        SELECT u.id FROM users u
        JOIN (
            SELECT LOWER(username) AS lname, MIN(id) AS keep_id
            FROM users GROUP BY LOWER(username) HAVING COUNT(*) > 1
        ) dup ON LOWER(u.username) = dup.lname AND u.id <> dup.keep_id
    );

-- 3) Enforce a unique, case-insensitive handle from here on.
CREATE UNIQUE INDEX IF NOT EXISTS ux_users_username_lower ON users(LOWER(username));

-- 4) Drop the now-redundant provider `name` column (folded into display_name).
ALTER TABLE users DROP COLUMN name;
