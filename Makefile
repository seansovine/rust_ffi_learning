rustlint:
	@cargo fmt && cargo clippy

rustfix:
	@cargo fmt && cargo clippy --fix

valgrind:
	@valgrind --leak-check=yes target/release/rust_ffi_learning

build:
	@cargo build --release

run:
	@cargo run --release --quiet
