fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

build:
	cargo build	

test:
	cargo test

run:
	cargo run -- $(ARGS)

all: fmt clippy build test run
