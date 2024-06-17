FROM alpine:3.10

# Install dependencies 
RUN apk add --no-cache cargo rust curl gcc
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup install nightly
RUN rustup default nightly

COPY dist /app
