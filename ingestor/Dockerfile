FROM rust:latest as builder
WORKDIR /app
# Copy Cargo files
COPY Cargo.toml ./
# Create a dummy main.rs to generate Cargo.lock and download dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
# Generate Cargo.lock and download dependencies (this layer will be cached)
RUN cargo build --release
# Remove dummy source and copy real source
RUN rm -rf src
COPY src ./src
# Build the actual application
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/ingestor /app/ingestor
CMD ["/app/ingestor"]
