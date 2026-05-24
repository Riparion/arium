# Runtime-only image — no Rust/wasm toolchain inside. Build the bundle on
# the host first, from the workspace root:
#
#   dx bundle --release --platform web --package dioxus-fullstack-example
#
# That drops `server` + `public/` under
# `target/dx/dioxus-fullstack-example/release/web/`. The build context is the
# workspace root (the example crate's bundle lives in the shared `target/`),
# so compose sets `context: ../..`. Then, from this directory:
#
#   docker compose up -d --build

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates libssl3 sqlite3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY target/dx/dioxus-fullstack-example/release/web/ /app/

# arium's file-backed mailer writes to a CWD-relative "./emails" dir; symlink
# it into the bind-mounted volume so the .eml files are inspectable from the
# host. The SQLite path is set via DATABASE_URL in docker-compose.yml and
# points straight at /app/data/auth.db — don't symlink the .db file itself
# (it trips SQLite's DBMOVED check, error 1032, when the file is recreated).
RUN mkdir -p /app/data/emails \
    && ln -s data/emails /app/emails

ENV IP=0.0.0.0 \
    PORT=8080
EXPOSE 8080
VOLUME ["/app/data"]
# mkdir runs AFTER the /app/data bind mount, so /app/data/emails exists before
# arium's `create_dir_all("./emails")` follows the symlink on first send.
CMD ["sh", "-c", "mkdir -p /app/data/emails && exec /app/server"]
