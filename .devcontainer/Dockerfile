FROM ubuntu:22.04
ARG USERNAME=sumcol

RUN apt update -y && apt upgrade -y && apt install -y \
    build-essential \
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
RUN . $HOME/.cargo/env && cargo install cargo-quickinstall
RUN . $HOME/.cargo/env && cargo quickinstall ripgrep fd-find

# Install Homebrew
RUN NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
RUN NONINTERACTIVE=1 /home/linuxbrew/.linuxbrew/bin/brew install neovim fish
RUN echo "eval \"\$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)\"" >> ~/.bashrc