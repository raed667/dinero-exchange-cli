# 🦀 Dinero Exchange CLI


A very basic CLI that internally uses [Dinero](https://github.com/raed667/dinero) for currency handling.

## 📦 Install

```sh
cargo install dinero-exchange-cli
```

## ⚡️ Usage

```sh
dinero-exchange-cli --help
Apply exchange rates. Simple CLI project to demonstrate the `Dinero` crate capabilities.

Usage: dinero-exchange-cli [OPTIONS] [VALUE]

Arguments:
  [VALUE]  Amount to apply the exchange rate to [default: 1]

Options:
  -d, --date <DATE>  Date of exchange rate for historical values
  -f, --from <FROM>  Base currency (Example: EUR,USD,CAD) [default: EUR]
  -t, --to <TO>      Target currency (Example: EUR,USD,CAD) [default: TND]
  -k, --key <KEY>    API key for Fixer API
      --json         Output JSON instead of human readable messages
  -h, --help         Print help information
  -V, --version      Print version information
```

[![asciicast](https://asciinema.org/a/I0oIlxn557Pa2y1ZuzOYqmnwx.svg)](https://asciinema.org/a/I0oIlxn557Pa2y1ZuzOYqmnwx)

## API Key

You will need a (free) API key to access the data. You can get one [here](https://apilayer.com/marketplace/fixer-api).

## 📜 License

[MIT](LICENSE)
