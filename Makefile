all: regenerate

.PHONY: regenerate
regenerate: src/main.rs

src/main.rs: pre-src/main.rs generate.py
	./generate.py

.PHONY: test
test: src/main.rs
	cargo run -q | diff -su src/main.rs -

CARGO_TARGETS=check run build

.PHONY: ${CARGO_TARGETS}
${CARGO_TARGETS}: %: src/main.rs
	cargo $@
