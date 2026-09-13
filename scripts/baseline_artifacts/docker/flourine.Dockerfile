FROM ubuntu:22.04

ARG DEBIAN_FRONTEND=noninteractive
# Generated FLOURINE harnesses contain #![feature(min_specialization)], so the
# artifact requires a nightly compiler.  Pin a dated nightly new enough to read
# the currently resolved Edition-2024 transitive dependencies.
ARG RUST_VERSION=nightly-2026-07-16
ARG CARGO_BOLERO_VERSION=0.10.0

RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        clang \
        cmake \
        curl \
        jq \
        libboost-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

ENV PATH=/root/.cargo/bin:/opt/flourine-adapter/bin:${PATH}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --profile minimal --default-toolchain "${RUST_VERSION}" \
    && rustc --version \
    && cargo --version

RUN cargo install --locked cargo-bolero --version "${CARGO_BOLERO_VERSION}" \
    && cargo bolero --version

RUN apt-get update && apt-get install -y --no-install-recommends python3-pip \
    && python3 -m pip install --no-cache-dir cmake==3.27.7 \
    && rm -rf /var/lib/apt/lists/* \
    && cmake --version | grep '3\.27\.7'

RUN mkdir -p /opt/flourine-adapter/bin \
    && ln -s /usr/bin/g++ /opt/flourine-adapter/bin/gcc10-c++ \
    && gcc10-c++ --version | head -n 1

WORKDIR /artifact/Differential_Tester
