build:
	@echo + $@
	cargo build

test:
	@echo + $@
	cargo test

fmt:
	@echo + $@
	cargo fmt --all -- --check
clippy:
	@echo + $@
	cargo clippy --all-features

release-%:
	@echo + $@
	cargo release --dev-version-ext next -- $*

dryrun-release-%:
	@echo + $@
	cargo release --dry-run -vv --dev-version-ext next -- $*
