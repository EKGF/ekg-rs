ifdef GIT_ROOT
else
GIT_ROOT := $(shell git rev-parse --show-toplevel 2>/dev/null)
endif

MK_DIR := $(GIT_ROOT)/.make

include ekgf-make.mk


.PHONY: build
build: cargo-build-workspace

.PHONY: build-release
build-release: cargo-build-workspace-release

.PHONY: install
install: cargo-install-components cog-install
