FROM ubuntu:22.04
ARG USERNAME=sumcol

RUN apt update -y && apt upgrade -y && apt install -y \
    build-essential \
    zsh \
    fish \
    neovim \
    sudo \
    curl \
    git \
    less

RUN useradd -m $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
USER $USERNAME

# Install Rust
ENV SHELL /bin/bash
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . $HOME/.cargo/env && cargo install ripgrep fd-find