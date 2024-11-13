PROJECT_NAME := ukma_url_parser

.PHONY: run test format clippy check clean build

run:
	cargo run -- parse urls.txt

credits:
	cargo run -- credits

test:
	cargo test

format:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

check: format clippy test

build:
	cargo build --release

clean:
	cargo clean

doc:
	cargo doc --open

help:
	@echo Available commands:
	@echo   make run     - Run program with urls.txt file
	@echo   make test    - Start all tests
	@echo   make format  - Format code with cargo fmt
	@echo   make clippy  - Run static program analysis with cargo clippy
	@echo   make check   - Format code with fmt, analyse with clippy and run all tests
	@echo   make build   - Build a project
	@echo   make clean   - Clean build files
	@echo   make doc     - Generate docs and open it in a browser
	@echo   make help    - Print this help message
	@echo   make credits - Provide information about an author of a crate