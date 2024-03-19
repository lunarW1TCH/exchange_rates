# builder

FROM rust:latest as builder

WORKDIR /usr/src/exchange_rates

COPY . .

RUN mkdir -p api_data
COPY api_data/physical_currency_list.csv api_data/

RUN cargo install --path .

RUN apt-get update && \
  apt-get install -y ca-certificates && \
  update-ca-certificates

RUN cargo build --release

# release

FROM ubuntu:22.04

RUN apt-get update && \
  apt-get install -y libssl-dev && \
  apt-get install -y openssl && \
  apt-get install -y ca-certificates

WORKDIR /usr/src/exchange_rates

RUN mkdir -p api_data
COPY api_data/physical_currency_list.csv api_data/

COPY --from=builder /usr/src/exchange_rates/target/release/exchange_rates /usr/src/exchange_rates

COPY .env .

RUN export $(grep -v '^#' .env | xargs)

ENTRYPOINT ["./exchange_rates"]