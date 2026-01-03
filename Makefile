.PHONY: install test

test:
	cargo test

install:
	cargo build --release
	mkdir -p $(HOME)/bin
	cp target/release/markdeck $(HOME)/bin/
	mkdir -p $(HOME)/.config/markdeck
	@if [ ! -f $(HOME)/.config/markdeck/config.toml ]; then \
		cp examples/config.toml $(HOME)/.config/markdeck/config.toml; \
		echo "Installed example config to $(HOME)/.config/markdeck/config.toml"; \
	else \
		echo "Config already exists at $(HOME)/.config/markdeck/config.toml, skipping"; \
	fi
