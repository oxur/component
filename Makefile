default: all

all: build test examples

build:
	@cargo build

test:
	@cargo test

examples:
	@cargo run --example=deps

.PHONY: examples
