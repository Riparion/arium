# Runtime-only image — no Rust/wasm toolchain inside. Build on the host first,
# from this example's directory:
#
#   cargo leptos build --release
#
# That produces the SSR server binary at `target/release/leptos-fullstack-example`
# and the hashed client bundle under `target/site/` (both in the shared
# workspace `target/`). The build context is the workspace root, so compose
# sets `context: ../..`. Then, from this directory:
#
#   docker compose up -d --build

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates libssl3 sqlite3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY target/release/leptos-fullstack-example /app/server
COPY target/site /app/site

# arium's file-backed mailer writes to a CWD-relative "./emails" dir; symlink
# it into the bind-mounted volume so the .eml files are inspectable from the
# host. The SQLite path is set via DATABASE_URL in docker-compose.yml and
# points straight at /app/data/auth-leptos.db — don't symlink the .db file
# (it trips SQLite's DBMOVED check, error 1032, when the file is recreated).
RUN mkdir -p /app/data/emails \
    && ln -s data/emails /app/emails

# At runtime there's no Cargo.toml, so Leptos reads its config from LEPTOS_*
# env (get_configuration(None)). These mirror [package.metadata.leptos] but
# point site-root at the copied bundle and bind all interfaces on 8080.
ENV LEPTOS_OUTPUT_NAME=leptos-fullstack-example \
    LEPTOS_SITE_ROOT=/app/site \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:8080 \
    LEPTOS_ENV=PROD
EXPOSE 8080
VOLUME ["/app/data"]
# mkdir runs AFTER the /app/data bind mount, so /app/data/emails exists before
# arium's `create_dir_all("./emails")` follows the symlink on first send.
CMD ["sh", "-c", "mkdir -p /app/data/emails && exec /app/server"]
