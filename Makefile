.PHONY: lint
lint:
	@cargo clippy

.PHONY: fix
fix:
	@cargo clippy --fix

.PHONY: test
test:
	@cargo test

.PHONY: install
install:
	@cargo install --path .

.PHONY: check
check:
	@cargo check
