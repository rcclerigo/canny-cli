.PHONY: build release install uninstall clean help

# Default target
all: install

# Build debug version
build:
	cargo build

# Build release version (optimized)
release:
	cargo build --release

# Install globally (builds release and links to /usr/local/bin)
install: release
	@echo "Installing canny to /usr/local/bin..."
	@sudo cp target/release/canny /usr/local/bin/canny
	@echo "✓ Installed! Run 'canny --help' to get started."

# Install without sudo (to ~/.cargo/bin which should be in PATH)
install-user: release
	@echo "Installing canny to ~/.cargo/bin..."
	@cp target/release/canny ~/.cargo/bin/canny
	@echo "✓ Installed! Run 'canny --help' to get started."

# Uninstall from /usr/local/bin
uninstall:
	@echo "Removing canny from /usr/local/bin..."
	@sudo rm -f /usr/local/bin/canny
	@echo "✓ Uninstalled."

# Uninstall from ~/.cargo/bin
uninstall-user:
	@echo "Removing canny from ~/.cargo/bin..."
	@rm -f ~/.cargo/bin/canny
	@echo "✓ Uninstalled."

# Clean build artifacts
clean:
	cargo clean

# Run tests
test:
	cargo test

# Check code without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Lint code
lint:
	cargo clippy

# Show help
help:
	@echo "Canny CLI Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make              Build and install globally (requires sudo)"
	@echo "  make build        Build debug version"
	@echo "  make release      Build release version"
	@echo "  make install      Install to /usr/local/bin (requires sudo)"
	@echo "  make install-user Install to ~/.cargo/bin (no sudo required)"
	@echo "  make uninstall    Remove from /usr/local/bin"
	@echo "  make uninstall-user Remove from ~/.cargo/bin"
	@echo "  make clean        Remove build artifacts"
	@echo "  make test         Run tests"
	@echo "  make check        Check code without building"
	@echo "  make fmt          Format code"
	@echo "  make lint         Run clippy linter"
	@echo "  make help         Show this help"
