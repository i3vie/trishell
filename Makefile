default:
	cargo build

run:
	cargo run

release:
	cargo build --release
	upx --best --lzma target/release/trishell

