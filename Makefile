release-x86_64:
	mkdir -p output
	cargo build --release --target x86_64-unknown-linux-gnu
	upx --best --lzma target/x86_64-unknown-linux-gnu/release/trishell
	mv target/x86_64-unknown-linux-gnu/release/trishell output/trishell-x86_64

release-aarch64:
	mkdir -p output
	cargo build --release --target aarch64-unknown-linux-gnu
	upx --best --lzma target/aarch64-unknown-linux-gnu/release/trishell
	mv target/aarch64-unknown-linux-gnu/release/trishell output/trishell-aarch64

release-all: release-x86_64 release-aarch64