# Use a Rust base image
FROM rust:1.56 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin hello_world_server
WORKDIR /hello_world_server

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependency is built, copy your source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/hello_world_server*
RUN cargo build --release

# Final base
FROM debian:buster-slim
COPY --from=builder /hello_world_server/target/release/hello_world_server .

# Run the binary
CMD ["./hello_world_server"]
