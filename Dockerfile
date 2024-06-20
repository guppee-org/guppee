FROM alpine:3.10

# Install dependencies 
RUN apk add --no-cache cargo rust curl gcc
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup install nightly
RUN rustup default nightly


COPY dist /dist
COPY server /server

EXPOSE 8081
ENTRYPOINT cargo run --manifest-path=server/Cargo.toml --release
