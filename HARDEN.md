# dx-auth hardening checklist

Security hardening tasks for the `dx-auth` crate and the `examples/basic`
app. Ordered roughly by cost/signal — start at the top.

Mirrored on standup board #4 ("dx-auth hardening").

Items marked **Automated** run in CI (see `.github/workflows/ci.yml` and
`.github/workflows/nightly.yml`). All security jobs are
`continue-on-error: true` for now — flip to gating once each is green.

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

**Automated.** CI runs `cargo audit`, `cargo deny`, and `cargo machete`
on every PR (advisory). The nightly workflow re-runs `cargo audit` plus
`cargo outdated` and `cargo geiger`. Policy lives in `deny.toml`.

- [x] Install the tools above (handled by CI runners)
- [x] `cargo audit` — wired into both `ci.yml` (PRs) and `nightly.yml`
- [x] Write `deny.toml` and run `cargo deny check` — `deny.toml` checked
      in; `deny` job runs on every PR
- [x] `cargo geiger` — nightly job
- [x] `cargo outdated --depth 1` — nightly job
- [x] `cargo machete` — `unused-deps` job on every PR
- [ ] Review the first run's findings and tighten `deny.toml` (license
      allow-list, skip/deny entries) before flipping any job to gating

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

**Automated.** The existing `clippy` job in `ci.yml` was extended with
the security lints above (advisory).

- [x] Run the clippy command above — `clippy` job, every PR
- [ ] Fix or `#[allow]` with justification each hit on request paths

## 4. Secret and config hygiene scan

```bash
gitleaks detect --source .
trufflehog filesystem .
```

The example has `auth.db` checked in — verify it's a throwaway and
confirm no `.env` / SMTP creds are tracked.

**Automated.** `gitleaks` runs on every PR (`secrets` job, fetches full
history). `trufflehog` runs nightly across the full repo with
`--only-verified`.

- [x] `gitleaks detect --source .` — `secrets` job, every PR
- [x] `trufflehog filesystem .` — nightly job
- [ ] Audit `examples/basic/auth.db` — confirm empty or remove from git
- [ ] Confirm `.env` / SMTP creds are `.gitignore`d

## 5. Fuzz the wire layer and OAuth callback

`src/wire.rs` and OAuth callback handling are good fuzz targets.

```bash
cargo install cargo-fuzz
cargo fuzz init
cargo fuzz add wire_parse
```

A matrix job is already drafted (commented out) in `nightly.yml` under
`fuzz:` — uncomment once the targets below exist.

- [ ] Install `cargo-fuzz`
- [ ] `cargo fuzz init && cargo fuzz add wire_parse`
- [ ] Add fuzz target for OAuth `state` / callback parsing
- [ ] Run each target for at least a few hours
- [ ] Uncomment the `fuzz` job in `nightly.yml`

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

A ZAP baseline job is drafted (commented out) in `nightly.yml` under
`zap:` — needs a CI boot harness for `examples/basic` (cargo run +
readiness probe) before it can run. The brute-force tools stay local.

- [ ] Boot `examples/basic` locally
- [ ] ZAP baseline scan (local now; uncomment `zap:` in `nightly.yml`
      once the example can boot in CI)
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
