generate:
	cd vdso-gen && \
		cargo build --release && \
		cargo run --release -- ..
