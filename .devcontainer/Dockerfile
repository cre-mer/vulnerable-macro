# Use the official Rust image as a base
FROM rust:1.92.0

# Install additional tools (optional)
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential

# Set the working directory
WORKDIR /workspaces

# Install stable x86_64 toolchain and components
RUN rustup toolchain install 1.92.0-x86_64-unknown-linux-gnu && \
    rustup component add --toolchain 1.92.0-x86_64-unknown-linux-gnu rustfmt clippy