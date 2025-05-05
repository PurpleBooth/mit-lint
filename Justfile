# This help screen
show-help:
	just --list

# Test it was built ok
test:
	RUST_BACKTRACE=1 cargo test

# Build release version
build:
	cargo build --release

# Check performance
bench:
	cargo bench

# Lint the whole project
lint:
	cargo fmt --all -- --check
	cargo clippy --all-features
	cargo check
	cargo audit

# Lint specific Rust files
lint-file file:
	cargo clippy --lib -- -W clippy::pedantic -W clippy::nursery --message-format=json | jq -r 'select(.file == "{{file}}")'

# Lint model/lint.rs
lint-lintrs:
	just lint-file src/model/lint.rs

# Lint model/problem.rs
lint-problemrs:
	just lint-file src/model/problem.rs

# Format what can be formatted
fmt:
	cargo fix --allow-dirty --allow-staged
	cargo clippy --allow-dirty --allow-staged --fix --all-features
	cargo fmt --all
	npx prettier --write **.yml **.json

# Clean the build directory
clean:
	cargo clean

# Check tests
mutate:
  cargo mutants
