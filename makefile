build:
	clear
	cargo build
	./target/debug/gitflux bump -vvvv

test:
	clear
	@echo Running unit tests...
	cargo test
	@echo Cleaning up tags
	git tag -d 0.0.2 0.1.2 1.1.2
