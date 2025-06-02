all: format test
	cargo build
test:
	cargo test
format:
	cargo fmt
run:
	cargo run -p galemind start