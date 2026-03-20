# Stage 1: Build
# Uses Rust stable with cargo-leptos to compile both the server binary and WASM client bundle.
FROM rust:1-bookworm AS builder

RUN cargo install cargo-binstall
RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown

# Install Tailwind CSS v4 CLI (required by cargo-leptos for CSS compilation)
# Install binaryen (wasm-opt) for additional WASM size optimization (10-20% reduction)
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y nodejs binaryen \
    && npm install -g @tailwindcss/cli

WORKDIR /app
COPY . .

# Install npm dependencies (tailwindcss v4 needs the package for @import resolution)
RUN npm install

# Build the release binary + WASM bundle + CSS
RUN cargo leptos build --release -vv

# Optimize WASM with wasm-opt (binaryen) for additional size reduction
RUN find target/site/pkg -name "*.wasm" -exec wasm-opt -Oz -o {}.opt {} \; \
    && find target/site/pkg -name "*.wasm.opt" -exec sh -c 'mv "$1" "${1%.opt}"' _ {} \;

# Stage 2: Runtime
# Minimal Debian image with only the compiled binary and assets.
FROM debian:trixie-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gabriel-osemberg /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/migrations /app/migrations

WORKDIR /app

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"

EXPOSE 8080

CMD ["/app/gabriel-osemberg"]
