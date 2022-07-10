TARGET=$(shell cat target.txt)

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

.PHONY: target-add
target-add:
	@rustup target add $(TARGET)

.PHONY: target-build
target-build: target-add
	@for target in $(TARGET); do echo $$target; cargo zigbuild --release --target $$target; done
