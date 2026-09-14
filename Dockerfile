FROM docker.io/library/ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    qemu-user-static \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
