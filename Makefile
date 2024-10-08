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

release-riscv64gc:
	mkdir -p output
	cargo build --release --target riscv64gc-unknown-linux-gnu
	# upx --best --lzma target/riscv64gc-unknown-linux-gnu/release/trishell
	# UPX can't actually pack RISCV64GC executables yet apparently
	mv target/riscv64gc-unknown-linux-gnu/release/trishell output/trishell-riscv64gc

release-all: release-x86_64 release-aarch64 release-riscv64gc