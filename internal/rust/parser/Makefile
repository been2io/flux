export CC=clang

# Default target, build
build:
	wasm-pack build --dev --scope influxdata
	$(CARGO) build --release

clean:
	$(CARGO) clean

test:
	$(CARGO) test

.PHONY: build clean test
