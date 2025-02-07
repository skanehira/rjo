.PHONY: lint
lint:
	@cargo clippy

.PHONY: fix
fix:
	@cargo clippy --fix

.PHONY: test
test:
	@cargo test

.PHONY: shellspec
shellspec:
	@shellspec

.PHONY: install
install:
	@cargo install --path .
