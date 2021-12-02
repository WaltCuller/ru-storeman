

# Runs clippy on the sources
.PHONY: check
check:
	cargo clippy --locked -- -D warnings

# Runs unit tests
.PHONY: test
test:
	cargo test --locked

.PHONY: simple-run
simple-run:
	cargo run

.PHONY: run
run: check test
	cargo run

.PHONY: build
build: check test
	cargo build

cross_install:
	cargo install cross

.PHONY: release
release: cross_install
	cross build --verbose --release

.PHONY: upload
upload:
	cd && go install github.com/tcnksm/ghr
