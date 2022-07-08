.PHONY: lint
lint:
	@cargo clippy

.PHONY: fix
fix:
	@cargo clippy --fix
