# markdeck

View markdown content as a slide deck in your terminal.

## How it works

`H1` and `H2` elements mark the beginning of a new slide.

## Installation

Run `make install` to build the binary and install it to `$HOME/bin`.
This will also copy `examples/config.toml` to `$HOME/.config/markdeck/config.toml`.

## Usage

Run `markdeck README.md`.

See the TUI controls at the bottom of your terminal.
Edit `config.toml` to change them.

```shell
Usage: markdeck [OPTIONS] <FILE>

Arguments:
  <FILE>  Path to the markdown file to present

Options:
  -c, --config <CONFIG>  Path to config file (defaults to ~/.config/markdeck/config.toml)
  -h, --help             Print help
```

## Demo

![demo](./demo.gif)
