# Makefile for trivia-battle (Linera Buildathon project)

.PHONY: all build clean setup test run

all: build

build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test --workspace

clean:
	cargo clean

setup:
	rustup target add wasm32-unknown-unknown
	cargo install linera-sdk --git https://github.com/linera-io/linera-protocol --branch testnet_conway

run-local:
	docker compose up -d

stop-local:
	docker compose down

help:
	@echo "Available commands:"
	@echo "  make build       - Build all contracts to Wasm"
	@echo "  make test        - Run unit tests"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make setup       - Install Linera CLI and Wasm target"
	@echo "  make run-local   - Start local testnet with Docker"
	@echo "  make stop-local  - Stop local testnet"
