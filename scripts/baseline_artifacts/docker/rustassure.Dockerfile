FROM ubuntu:22.04

ARG DEBIAN_FRONTEND=noninteractive
ARG RUSTASSURE_COMMIT=39618406bd0c6acb1c5ca2cd697c3781a8e4c296
ARG RUSTIFY_KLEE_COMMIT=d880ceb4a521126a243e6126430e39a215abed06
ARG NLOHMANN_JSON_COMMIT=d33ecd3f3bd11e30aa8bbabb00e0a9cd3f2456d8

RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        clang-14 \
        cmake \
        curl \
        git \
        graphviz \
        libgraphviz-dev \
        libsqlite3-dev \
        libz3-dev \
        llvm-14 \
        llvm-14-dev \
        llvm-14-tools \
        ninja-build \
        pkg-config \
        python3 \
        python3-dev \
        python3-pip \
        z3 \
        zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

ENV PATH=/root/.cargo/bin:/usr/lib/llvm-14/bin:/opt/rustify-klee/bin:${PATH}
ENV LLVM_DIR=/usr/lib/llvm-14/lib/cmake/llvm

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --profile minimal --default-toolchain 1.64.0 \
    && rustc --version \
    && rustc -vV | grep 'LLVM version: 14\.'

RUN git clone https://github.com/davsec-lab/rustify-klee.git /opt/src/rustify-klee \
    && git -C /opt/src/rustify-klee checkout --detach "${RUSTIFY_KLEE_COMMIT}" \
    && test "$(git -C /opt/src/rustify-klee rev-parse HEAD)" = "${RUSTIFY_KLEE_COMMIT}" \
    && cmake -S /opt/src/rustify-klee -B /opt/build/rustify-klee -G Ninja \
        -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_INSTALL_PREFIX=/opt/rustify-klee \
        -DLLVM_DIR=/usr/lib/llvm-14/lib/cmake/llvm \
        -DLLVMCC=/usr/bin/clang-14 \
        -DLLVMCXX=/usr/bin/clang++-14 \
        -DENABLE_TCMALLOC=OFF \
        -DENABLE_SOLVER_STP=OFF \
        -DENABLE_SOLVER_Z3=ON \
        -DENABLE_SOLVER_METASMT=OFF \
        -DENABLE_POSIX_RUNTIME=OFF \
        -DENABLE_UNIT_TESTS=OFF \
        -DENABLE_SYSTEM_TESTS=OFF \
        -DENABLE_DOCS=OFF \
    && cmake --build /opt/build/rustify-klee -j4 \
    && cmake --install /opt/build/rustify-klee

RUN git clone https://github.com/davsec-lab/rustassure.git /opt/rustassure \
    && git -C /opt/rustassure checkout --detach "${RUSTASSURE_COMMIT}" \
    && test "$(git -C /opt/rustassure rev-parse HEAD)" = "${RUSTASSURE_COMMIT}" \
    && git clone https://github.com/nlohmann/json.git /opt/rustassure/src/Symbolizer/json \
    && git -C /opt/rustassure/src/Symbolizer/json checkout --detach "${NLOHMANN_JSON_COMMIT}" \
    && test "$(git -C /opt/rustassure/src/Symbolizer/json rev-parse HEAD)" = "${NLOHMANN_JSON_COMMIT}"

RUN cmake -S /opt/rustassure/src/Symbolizer -B /opt/rustassure/src/Symbolizer/build -G Ninja \
        -DCMAKE_BUILD_TYPE=RelWithDebInfo \
        -DCMAKE_C_COMPILER=/usr/bin/clang-14 \
        -DCMAKE_CXX_COMPILER=/usr/bin/clang++-14 \
        -DLLVM_DIR=/usr/lib/llvm-14/lib/cmake/llvm \
    && cmake --build /opt/rustassure/src/Symbolizer/build -j4

RUN apt-get update \
    && apt-get install -y --no-install-recommends fdupes \
    && rm -rf /var/lib/apt/lists/* \
    && rustup toolchain install 1.70.0 --profile minimal \
    && cargo +1.70.0 install rustfilt --version 0.2.1 \
    && rustup toolchain uninstall 1.70.0

RUN python3 -m pip install --no-cache-dir \
        antlr4-python3-runtime==4.13.2 \
        llvmlite==0.40.1 \
        more-itertools==10.7.0 \
        networkx==3.1 \
        numpy==1.24.4 \
        openai==1.109.1 \
        pandas==2.0.3 \
        pydot==3.0.4 \
        pygraphviz==1.11 \
        pycparser==2.23 \
        scipy==1.10.1 \
        sympy==1.12 \
        tabulate==0.9.0 \
        tiktoken==0.11.0 \
        tree-sitter==0.25.2 \
        tree-sitter-c==0.23.6 \
        tree-sitter-rust==0.23.2

RUN klee --version \
    && klee --help | grep -- '--target-function-name' \
    && clang --version | grep 'version 14\.' \
    && opt --version | grep 'LLVM version 14\.' \
    && rustfilt --version \
    && fdupes --version \
    && python3 -c 'from tree_sitter import Language, Parser; import llvmlite, networkx, tree_sitter_c, tree_sitter_rust; Parser(Language(tree_sitter_c.language())); Parser(Language(tree_sitter_rust.language()))' \
    && cd /opt/rustassure/src/python/kquery-parser \
    && python3 -c 'import KqueryGrapher; assert KqueryGrapher.logging.__name__ == "logging"'

WORKDIR /opt/rustassure/src/python
