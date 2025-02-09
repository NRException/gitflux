.PHONY: clean
clean:
	rm -rf target

.PHONT: fmt
fmt:
	cargo fmt

.PHONY: lint
lint: fmt
	cargo clippy

.PHONY: test
test:
	cargo test

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release
