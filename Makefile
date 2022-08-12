.PHONY: rapid-class-finder
rapid-class-finder:
	cargo build --release --frozen --offline
	cp target/release/rapid-class-finder .

.PHONY: revendor
revendor:
	git config --local core.autocrlf false
	rm -rf ./vendor
	cargo vendor ./vendor

.PHONY: deploy
deploy:
	fly deploy --nixpacks