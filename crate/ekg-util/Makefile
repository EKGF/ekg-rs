ifndef GIT_ROOT
GIT_ROOT := $(shell git rev-parse --show-toplevel 2>/dev/null)
endif

MK_DIR := $(GIT_ROOT)/.make

-include $(GIT_ROOT)/ekgf-make.mk

.PHONY: build
build: cargo-check
	$(CARGO_BIN) build