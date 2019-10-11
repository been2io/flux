export CC=clang

# Default target, build
build:
	wasm-pack build --dev --scope influxdata

rust-bench:
	$(CARGO) build --release

clean:
	$(CARGO) clean

test:
	$(CARGO) test

.PHONY: build clean test
