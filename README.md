# markdeck

View markdown content as a slide deck in your terminal.

A nice way to read a simple markdown file one section at a time.

## How it works

`H1` and `H2` elements form the boundaries of slides.

Other elements are rendered as content on a slide.

## Installation

```shell
git clone https://github.com/julian-computes/markdeck.git

# Build the binary and install it to $HOME/bin
# This will also copy examples/config.toml to $HOME/.config/markdeck/config.toml
make install
```

## Usage

Run `markdeck README.md` for an example.

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
