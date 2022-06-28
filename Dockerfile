# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --release

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
EXPOSE 8080
ENV PORT 8080
ENV HOST 0.0.0.0

# Run the web service on container startup.
ENTRYPOINT ["target/release/API-Server"]