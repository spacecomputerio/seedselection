test:
	@cargo test

fmt:
	@cargo fmt

fmt-check:
	@cargo fmt -- --check

clippy:
	@cargo clippy -- -D warnings

lint: clippy

build:
	@cargo build --release

help:
	@echo ""
	@echo "Usage: make [vars] <cmd>"
	@echo ""
	@echo "Available commands:"
	@echo "  test            Run unit tests"
	@echo "  fmt             Format code"
	@echo "  fmt-check       Check code formatting"
	@echo "  clippy          Run clippy linter"
	@echo "  lint            Run linters"
	@echo "  build           Build the project"
	@echo "  help            Show this help message"
	@echo ""

default: help
