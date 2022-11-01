FROM ubuntu:20.04 AS base

ARG USER_NAME=dev

#
# Configurations for LLVM.
# It is preferable to set LLVM_BUILD_CONFIG RelWithDebInfo.
#
ARG LLVM_VERSION=14.0.6
ARG LLVM_BUILD_CONFIG=Release
ARG LLVM_INSTALLATION_DIR=/home/dev/llvm
ENV LLVM_SYS_140_PREFIX=$LLVM_INSTALLATION_DIR

ENV DEBIAN_FRONTEND=noninteractive
ENV TERM=xterm-256color

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
        ninja-build \
        python3 \
        sudo \
        wget; \
    ########################################################
    #
    # Add a user
    #
    ########################################################
    adduser --disabled-password --gecos '' $USER_NAME; \
    adduser $USER_NAME sudo; \
    echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers;

USER $USER_NAME

FROM base AS llvm

ENV LLVM_DIRECTORY=llvm-project-$LLVM_VERSION.src

RUN \
    set -eux; \
    ########################################################
    #
    # Install LLVM
    #
    ########################################################
    mkdir $LLVM_INSTALLATION_DIR; \
    mkdir $HOME/tmp; \
    cd $HOME/tmp; \
    wget https://github.com/llvm/llvm-project/releases/download/llvmorg-$LLVM_VERSION/$LLVM_DIRECTORY.tar.xz; \
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
    cmake --build . . --target install; \
    ########################################################
    #
    # Clean waste
    #
    ########################################################
    cd $HOME; \
    rm -rf $HOME/tmp;

FROM llvm AS ship

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

ENV PATH=$PATH:$LLVM_INSTALLATION_DIR
ENV DEBIAN_FRONTEND=newt

SHELL ["bash", "-l"]
