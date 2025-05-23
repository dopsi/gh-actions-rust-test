FROM mcr.microsoft.com/devcontainers/base:alpine

USER root

RUN apk add rustup zlib-static openssl-libs-static

USER vscode

RUN rustup-init -y -c rls && \
    source "$HOME/.cargo/env" && \
    cargo install --locked cargo-mutants cargo-tarpaulin