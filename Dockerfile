# Use the official Rust image as a base
FROM rust:latest

# Install additional tools (optional)
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential

# Set the working directory
WORKDIR /workspaces

# Install rustup toolchain
RUN rustup toolchain install 1.83.0-aarch64-unknown-linux-gnu

# Install Rust tools (optional)
RUN rustup component add rustfmt clippy

# Install zsh and oh-my-zsh
RUN apt install zsh
RUN sh -c "$(wget https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh -O -)"

# Set the default command
CMD ["bash"]