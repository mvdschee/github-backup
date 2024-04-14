
build:
	cargo build --release
test:
	cargo test -- --nocapture
dev: 
	cargo watch -q -c -w src/ -x run 
	