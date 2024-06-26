FROM messense/rust-musl-cross:x86_64-musl AS chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
WORKDIR /hatchet

FROM chef AS planner
# Copy source code
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /hatchet/recipe.json recipe.json
# Build and cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /hatchet/target/x86_64-unknown-linux-musl/release/hatchet /hatchet
EXPOSE 8000
ENTRYPOINT ["/hatchet"]
