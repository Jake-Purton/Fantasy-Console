FROM ianpurton/rust-fullstack-devcontainer:latest AS development

RUN sudo npm install -g wasm4

RUN rustup target add wasm32-unknown-unknown

ARG USERNAME=vscode
RUN sudo mkdir -p /vscode/target && sudo chown $USERNAME:$USERNAME /vscode/target