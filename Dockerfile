
# syntax=docker/dockerfile:experimental
#
# Copyright 2021 ChainSafe Systems
# SPDX-License-Identifier: LGPL-3.0-only
#
# Building layer
FROM paritytech/ci-linux:production as builder
COPY . .
ENV CARGO_TERM_COLOR=always
RUN --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,sharing=private,target=target \
    cargo +nightly build --release && \
    mv target/release/server /server

# Release
FROM debian:buster-slim
ENV DEBIAN_FRONTEND=noninteractive
LABEL description="The docker image of polywrap-substrate"
COPY --from=builder /server /usr/local/bin/
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -m -u 1000 -U -s /bin/sh -d /polywrap-substrate polywrap-substrate && \
    mkdir -p /polywrap-substrate/.local/share && \
    mkdir /data && \
    chown -R polywrap-substrate:polywrap-substrate /data && \
    ln -s /data /polywrap-substrate/.local/share/polywrap-substrate && \
    rm -rf /usr/bin /usr/sbin

USER polywrap-substrate
# 30333 for p2p traffic
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
# 8000 for graphql playground
EXPOSE 30333 9933 9944 9615 8000
VOLUME [ "/data" ]
ENTRYPOINT [ "/usr/local/bin/server" ]
