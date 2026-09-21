FROM rust:1.98

WORKDIR /app

# Copy Cargo files zuerst (für besseres Caching)
COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/rapi"]
