FROM ubuntu:20.04 AS base

ARG USER_NAME=dev

#
# Configurations for LLVM.
# It is preferable to set LLVM_BUILD_CONFIG RelWithDebInfo.
#
ARG LLVM_VERSION=14.0.6
ARG LLVM_BUILD_CONFIG=Release
ARG LLVM_INSTALLATION_DIR=/llvm
ENV LLVM_SYS_140_PREFIX=$LLVM_INSTALLATION_DIR
ENV LLVM_DIRECTORY=llvm-project-$LLVM_VERSION.src

ENV DEBIAN_FRONTEND=noninteractive
ENV TERM=xterm-256color
ENV PATH=$PATH:$LLVM_INSTALLATION_DIR/bin

RUN \
    set -eux; \
    ########################################################
    #
    # Install fundamental tools
    #
    ########################################################
    apt update; \
    apt install -y \
        build-essential \
        cmake \
        curl \
        git \
        libffi-dev \
        ninja-build \
        python3 \
        sudo \
        wget; \
    ########################################################
    #
    # Install LLVM
    #
    ########################################################
    mkdir $LLVM_INSTALLATION_DIR; \
    mkdir /tmp/llvm; \
    cd /tmp/llvm; \
    wget \
        https://github.com/llvm/llvm-project/releases/download/llvmorg-$LLVM_VERSION/$LLVM_DIRECTORY.tar.xz; \
    tar xJf $LLVM_DIRECTORY.tar.xz; \
    mkdir $LLVM_DIRECTORY/llvm/build; \
    cd $LLVM_DIRECTORY/llvm/build; \
    cmake .. \
        -DCMAKE_INSTALL_PREFIX=$LLVM_INSTALLATION_DIR \
        -DCMAKE_BUILD_TYPE=$LLVM_BUILD_CONFIG \
        -DLLVM_INCLUDE_EXAMPLES=OFF \
        -DLLVM_INCLUDE_TESTS=OFF \
        -G Ninja; \
    cmake --build .; \
    cmake --build . --target install; \
    ########################################################
    #
    # Clean waste
    #
    ########################################################
    cd /; \
    rm -rf /tmp/llvm; \
    ########################################################
    #
    # Add a user
    #
    ########################################################
    adduser --disabled-password --gecos '' $USER_NAME; \
    adduser $USER_NAME sudo; \
    echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers;

FROM base as non-root

USER $USER_NAME

FROM non-root AS with-rust

RUN \
    set -eux; \
    ########################################################
    #
    # Install Rust
    #
    ########################################################
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
    ########################################################
    #
    # Clean waste
    #
    ########################################################
    sudo apt clean; \
    sudo rm -rf /var/lib/apt/lists/*;

ENV DEBIAN_FRONTEND=newt

SHELL ["bash", "-l"]
