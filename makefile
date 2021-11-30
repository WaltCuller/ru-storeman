

# Runs clippy on the sources
check:
	cargo clippy --locked -- -D warnings

# Runs unit tests
test:
	cargo test --locked


simple-run:
	cargo run


run: check test
	cargo run


build: check test
	cargo build