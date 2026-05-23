# dx-auth hardening checklist

Security hardening tasks for the `dx-auth` crate and the `examples/basic`
app. Ordered roughly by cost/signal — start at the top.

Mirrored on standup board #4 ("dx-auth hardening").

## 1. Run `/security-review` on the branch

Use the built-in `/security-review` slash command to scan the current
diff/branch for OWASP-style issues across the whole crate.

- [ ] Branch off `main` with no uncommitted changes
- [ ] Invoke `/security-review`
- [ ] Triage findings into follow-up cards

## 2. Rust supply chain audit

`cargo deny` is the highest-value here — write a `deny.toml` that bans
yanked crates and untrusted git sources (`dioxus-primitives` is a git
dep).

```bash
cargo install cargo-audit cargo-deny cargo-geiger cargo-outdated cargo-machete
cargo audit
cargo deny check
cargo geiger
cargo outdated --depth 1
cargo machete
```

- [ ] Install the tools above
- [ ] `cargo audit` — RustSec advisories on `Cargo.lock`
- [ ] Write `deny.toml` and run `cargo deny check`
- [ ] `cargo geiger` — surface `unsafe` in deps (argon2, sqlx, etc.)
- [ ] `cargo outdated --depth 1`
- [ ] `cargo machete` — drop unused deps

## 3. Clippy with security-leaning lints

For an auth crate, hits on `unwrap` / `panic` in request paths are real
DoS findings.

```bash
cargo clippy --all-features --all-targets -- \
  -D warnings \
  -W clippy::unwrap_used -W clippy::expect_used \
  -W clippy::panic -W clippy::indexing_slicing \
  -W clippy::integer_arithmetic
```

- [ ] Run the clippy command above
- [ ] Fix or `#[allow]` with justification each hit on request paths

## 4. Secret and config hygiene scan

```bash
gitleaks detect --source .
trufflehog filesystem .
```

The example has `auth.db` checked in — verify it's a throwaway and
confirm no `.env` / SMTP creds are tracked.

- [ ] `gitleaks detect --source .` (scans repo + history)
- [ ] `trufflehog filesystem .`
- [ ] Audit `examples/basic/auth.db` — confirm empty or remove from git
- [ ] Confirm `.env` / SMTP creds are `.gitignore`d

## 5. Fuzz the wire layer and OAuth callback

`src/wire.rs` and OAuth callback handling are good fuzz targets.

```bash
cargo install cargo-fuzz
cargo fuzz init
cargo fuzz add wire_parse
```

- [ ] Install `cargo-fuzz`
- [ ] `cargo fuzz init && cargo fuzz add wire_parse`
- [ ] Add fuzz target for OAuth `state` / callback parsing
- [ ] Run each target for at least a few hours

## 6. Dynamic scan against the running example

Start the example, then point web scanners + brute-force tools at it.
Verify `tower_governor` actually rate-limits `/login`, and audit the
session cookie flags in DevTools.

```bash
docker run -t zaproxy/zap-stable zap-baseline.py -t http://localhost:8080
nuclei -u http://localhost:8080 -t http/exposures -t http/misconfiguration
# gobuster / ffuf for hidden endpoints
# hydra against /login to confirm lockout fires
```

- [ ] Boot `examples/basic` locally
- [ ] ZAP baseline scan
- [ ] `nuclei` exposures + misconfiguration templates
- [ ] `gobuster` / `ffuf` for hidden endpoints
- [ ] `hydra` against `/login` — confirm rate-limit + lockout fires
- [ ] DevTools: session cookie has `HttpOnly` + `Secure` + `SameSite`

## 7. Dioxus MCP security checks

The Dioxus MCP tools are already loaded.
`server_fn_call_graph` + `repeated_auth_extractor` are the most relevant
for an auth library — every server fn must be guarded.

- [ ] `mcp__dioxus__insecure_set_cookie`
- [ ] `mcp__dioxus__lint_project`
- [ ] `mcp__dioxus__audit_feature_flags` (the feature matrix is non-trivial)
- [ ] `mcp__dioxus__server_fn_call_graph` — confirm auth guard on every server fn
- [ ] `mcp__dioxus__repeated_auth_extractor`

## 8. Auth-specific manual review

No tool replaces this. Walk the flows by hand against the threat model.

- [ ] CSRF on state-changing server fns
- [ ] Session rotation on login, MFA promotion, password change
- [ ] Argon2id params: memory cost ≥ 19 MiB, time cost ≥ 2 (`src/auth.rs`)
- [ ] TOTP recovery codes single-use enforced at the DB level
- [ ] OAuth `state` verified, callback URL pinned, account-link can't be
      cross-user-triggered
- [ ] Rate limiter keyed correctly behind a proxy (X-Forwarded-For trust)
- [ ] `audit_events` table is append-only at the DB layer
