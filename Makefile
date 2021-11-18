default:
	cargo build --release --locked --target x86_64-apple-darwin
	cargo build --release --locked --target aarch64-apple-darwin
	cargo build --release --locked --target x86_64-pc-windows-gnu
	cargo build --release --locked --target x86_64-unknown-linux-gnu
	cargo build --release --locked --target aarch64-unknown-linux-gnu

run:
	cargo run

format:
	cargo fmt --all -- --check