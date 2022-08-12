.PHONY: rapid-class-finder
rapid-class-finder:
	cargo build --release --frozen --offline
	cp target/release/rapid-class-finder .

.PHONY: revendor
revendor:
	rm -rf ./vendor
	cargo vendor ./vendor