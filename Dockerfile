FROM ubuntu:22.04


RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y curl build-essential librust-atk-dev libcairo2-dev libpango1.0-dev libgtk-4-dev libgdk-pixbuf2.0-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR /app
COPY . /app

RUN /root/.cargo/bin/cargo build

