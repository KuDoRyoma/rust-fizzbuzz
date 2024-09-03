
FROM mcr.microsoft.com/devcontainers/base:ubuntu

RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get upgrade -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

ARG USERNAME=vscode

USER $USERNAME

WORKDIR /home/$USERNAME/

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y