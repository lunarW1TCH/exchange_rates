# Setup

- Clone repo
- Go to [Alpha Vantage](https://www.alphavantage.co/support/#api-key) docs to generate free API_KEY
- Create `.env` file at the root of project and put the following contents inside:

```
VANTAGE_KEY=<your_api_key>
```

## Local

```sh
cargo build -r
```

### Help

```sh
./target/release/exchange_rates -h
```

### List of currency codes

```sh
./target/release/exchange_rates -l
```

### Example conversion of 7.50PLN to USD

```sh
./target/release/exchange_rates -t USD -f PLN -a 7.50
```

## Docker

### Build

```sh
docker build exchange .
```

### Help

```sh
docker run exchange -h
```

### List of currency codes

```sh
docker run exchange -l
```

### Example conversion of 7.50PLN to USD

```sh
docker run exchange -t USD -f PLN -a 7.50
```

# Example result

```
Exchange from Polish Zloty into United States Dollar:
Exchange rate: 0.25090000,
7PLN = 1.7563USD
```

# Usage / Help

```
Usage: exchange_rates [OPTIONS]

Options:
  -l, --list             Prints a list of available country codes with their corresponding name
  -t, --to <TO>          The destination currency for the exchange rate. For example: `--to=USD`
  -f, --from <FROM>      The currency you would like to get the exchange rate for. For example: `--from=PLN`
  -a, --amount <AMOUNT>  Amount you would like to convert. For example: `--amount=7.52`
  -h, --help             Print help
  -V, --version          Print version
```

# Tests

Unit tests run automatically during building a docker image. You can also run them locally using `cargo test`
