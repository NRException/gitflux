tag:
	clear
	@echo Running unit tests...
	cargo test
	./target/debug/gitflux tag -vvvv
