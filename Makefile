dev-size:
	du ./target/debug/ort_lightgbm_ex -h

prod-size:
	du ./target/release/ort_lightgbm_ex -h

rel:
	cargo run --release

run:
	cargo run

fmt:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test