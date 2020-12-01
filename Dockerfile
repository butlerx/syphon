FROM rust as builder

RUN rustup component add rustfmt
WORKDIR /usr/src/syphon
COPY . .
RUN cargo install --path .

# Build the actual container
FROM scratch
LABEL maintainer="Cian Butler<butlerx@notthe.cloud>"

COPY --from=builder /usr/local/cargo/bin/syphon /usr/local/bin/syphon
ADD configs/config.toml /etc/syphon/config.toml

EXPOSE 2003
ENTRYPOINT ["/usr/local/bin/syphon"]
CMD ["-vv"]
