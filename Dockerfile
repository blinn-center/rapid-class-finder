FROM rust:1.63-bullseye AS builder
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN make

FROM debian:bullseye
COPY --from=builder /app/rapid-class-finder /usr/local/bin/rapid-class-finder
CMD ["/usr/local/bin/rapid-class-finder"]
EXPOSE 3000