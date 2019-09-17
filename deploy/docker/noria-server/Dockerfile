FROM rustlang/rust:nightly
RUN apt-get update && apt-get install -y clang libclang-dev libssl-dev liblz4-dev git
WORKDIR /src
RUN git clone https://github.com/mit-pdos/noria.git && cd noria && cargo build
