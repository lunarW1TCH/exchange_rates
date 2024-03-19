todo

- tests
- docker image

# Setup from source

- Go to [Alpha Vantage](https://www.alphavantage.co/support/#api-key) docs to generate free API_KEY
- create `.env` file at the root of project and put the following contents inside:

```env
VANTAGE_KEY=<your_api_key>
```

# Setup docker

## Requests

### Request

do zmiany na docker image or sth

```sh
cargo run -- -t USD -f PLN -a 7
```

### Example result

```
Exchange from Polish Zloty into United States Dollar:
Exchange rate: 0.25090000,
7PLN = 1.7563USD
```

## Usage / Help

```
Usage: exchange_rates [OPTIONS]

Options:
  -l, --list             Prints a list of available country codes with their corresponding name
  -t, --to <TO>          The destination currency for the exchange rate. For example: `--to=USD`
  -f, --from <FROM>      The currency you would like to get the exchange rate for. For example: `--from=PLN`
  -a, --amount <AMOUNT>  Amount you would like to convert. For example: `--amount=21.37`
  -h, --help             Print help
  -V, --version          Print version
```
