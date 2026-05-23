# Security Policy

## Reporting a vulnerability

If you believe you've found a security issue in `dx-auth`, please report
it privately. Public issues on the tracker should be reserved for
non-security bug reports.

- **Preferred:** open a GitHub Security Advisory at
  <https://github.com/tonybierman/dx-auth/security/advisories/new> — this
  keeps the report private until a fix lands.
- **Fallback:** email `tonybierman@gmail.com` with `[dx-auth security]`
  in the subject.

This is a personal project with no SLA; expect best-effort response
times. Please include in your report:

- a clear description of the issue and its impact,
- a minimal reproduction (commit SHA + repro steps or PoC),
- whether you intend to disclose publicly and on what timeline.

Researchers will be credited in the changelog and the resolved advisory
unless they request otherwise.

## Supported versions

The crate is pre-1.0 and ships from `main`. Only the latest commit on
`main` receives fixes. There is no release-branch backport policy yet.

## Threat model

`dx-auth` is a reusable authentication and authorization library for
Dioxus fullstack apps. It is intended to defend against:

- credential stuffing and brute-force sign-in (per-IP rate limiting),
- offline password cracking after database exfiltration (Argon2id
  hashing),
- session theft over the wire (cookie flags + TLS expected at the
  reverse proxy),
- OAuth callback CSRF (`state` verification stored in the session and
  checked on callback — see `crates/dx-auth/src/oauth.rs`),
- TOTP recovery-code reuse (single-use enforced at the DB layer:
  `mfa_recovery_codes.used_at`),
- SQL injection (queries built with SQLx parameter binding — no string
  interpolation),
- secret leakage into the repo (gitleaks + trufflehog scans, see
  below).

It is **not** intended to defend against:

- a compromised host or database operator,
- a malicious dependency injected before the advisory database flags
  it,
- application-level authorization bugs in code that *consumes* this
  crate (it provides building blocks; correct use is the consumer's
  responsibility),
- side-channel attacks on Argon2 outside the documented parameter
  envelope.

## Hardening practices in place

### Authentication design

- Passwords hashed with **Argon2id** via the `argon2` crate.
- Sessions backed by `axum_session` / `axum_session_auth`.
- **TOTP** enrolment generates recovery codes whose Argon2 hashes are
  stored in `mfa_recovery_codes`; consumption sets `used_at` so a code
  can never be replayed.
- **OAuth** flows store the CSRF `state` per provider in the session
  before redirecting and compare on callback.
- **Per-IP rate limiting** on the entire router via `tower_governor`.
- **Append-only `audit_events` table** records authentication and
  admin state changes.
- **Bootstrap admin** is gated by the `BOOTSTRAP_ADMIN_EMAIL` env var —
  the first signup matching that email is auto-promoted, after which
  the env var no longer grants privileges to anyone else.

### Dependency hygiene (CI-enforced, advisory)

Two GitHub Actions workflows run security tooling:

**Per push / PR — `.github/workflows/ci.yml`**

- `cargo audit` — RustSec advisory database against `Cargo.lock`.
- `cargo deny check` — license / source / bans policy
  (see [`deny.toml`](deny.toml)).
- `cargo machete` — unused dependency detection.
- `gitleaks` — secret scan over the diff and history.
- `cargo clippy` with security-leaning lints
  (`unwrap_used`, `expect_used`, `panic`, `indexing_slicing`,
  `integer_arithmetic`).
- `cargo test` and a `cargo check` matrix across realistic feature
  combinations.

**Nightly — `.github/workflows/nightly.yml`**

- `cargo audit` again, so newly-published advisories surface within
  24 hours even when no PR is open.
- `cargo outdated` — surfaces new compatible releases.
- `cargo geiger` — counts `unsafe` reachable in the dep tree.
- `trufflehog` — full-history secret scan with `--only-verified`.

All security jobs are currently `continue-on-error: true` and run as
advisory. They are flipped to gating individually as each becomes
clean.

### Policy files

- [`deny.toml`](deny.toml) — license allow-list with per-entry
  rationale; explicit `allow-git` per source (no wildcard org trust);
  `yanked = "deny"`; `allow-wildcard-paths = true` for internal
  workspace members only.
- [`.gitignore`](.gitignore) — excludes dev SQLite databases, the
  dev-fallback `emails/` directory, and `.env` files.

### Cryptography and randomness

- **Password hashing:** Argon2id (`argon2` crate, default parameters).
  Verification uses the `password-hash` ecosystem; do not reach into
  raw hash bytes.
- **OAuth:** `oauth2` 5.x over `reqwest` + `rustls-tls`.
- **TOTP:** `totp-rs` 5.x.
- **Random secrets:** seeded from the OS via `argon2::password_hash::
  rand_core::OsRng` and `getrandom` — never a thread-local PRNG.

## Known limitations

- TLS termination is expected at a reverse proxy in front of the app;
  the example does not terminate TLS itself.
- The development-mode email backend writes `.eml` files to disk for
  inspection — never enable it in production (the `MAIL_*` env vars
  drive the production SMTP backend instead).
- An advisory (`RUSTSEC-2023-0071`, `rsa 0.9.10`) is reachable only via
  `sqlx-macros-core`'s compile-time backend support; it is not
  reachable from a deployed SQLite-only or Postgres-only build. Triage
  is in progress.
