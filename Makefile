.PHONY: install

test:
	cargo test

install:
	cargo build --release
	mkdir -p $(HOME)/bin
	cp target/release/markdeck $(HOME)/bin/
