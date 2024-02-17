FROM docker.io/library/rust:latest as builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
COPY --from=builder /app/target/release/imdl /imdl
ENTRYPOINT ["/imdl"]