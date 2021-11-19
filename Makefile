# works on macOS
default:
	cargo build --release --locked --target x86_64-apple-darwin
	cargo build --release --locked --target aarch64-apple-darwin
	cargo build --release --locked --target x86_64-pc-windows-gnu

	# brew tap SergioBenitez/osxct
	# brew install x86_64-unknown-linux-gnu
	cargo build --release --locked --target x86_64-unknown-linux-gnu
	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc cargo build --release --locked --target aarch64-unknown-linux-gnu

run:
	cargo run

format:
	cargo fmt --all -- --check