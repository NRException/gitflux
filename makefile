build:
	clear
	cargo build

tag:
	clear
	@echo Running unit tests...
	cargo test
	./target/debug/gitflux tag -vvvv
	@echo Cleaning up tags
	git tag -d 0.0.2 0.1.2 1.1.2
