all:
	cargo test
fmto:
	cargo fmt -- --write-mode overwrite
clip:
	cargo build --features dev
