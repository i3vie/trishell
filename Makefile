run:
	cargo run

release:
	cargo build --release
	upx --best --lzma target/release/trishell

release-aarch64:
	cargo build --release --target aarch64-unknown-linux-gnu
	upx --best --lzma target/aarch64-unknown-linux-gnu/release/trishell

