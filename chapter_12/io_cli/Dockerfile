from ubuntu:latest as builder

RUN apt-get update && \
apt-get upgrade -y && \
apt-get install build-essential curl -y

# RUN export CARGO_HOME=/cargo && \
# export RUSTUP_HOME=/rustup

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN chmod 777 -R $HOME/.cargo

RUN ["/bin/bash", "-c", "source $HOME/.cargo/env", "rustup target add x86_64-unknown-linux-musl"]

CMD /bin/sh
