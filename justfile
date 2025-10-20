# Durf justfile.

# List directives.
[private]
default:
    @just --list

# Set up dependencies.
setup:
    rustup default stable
    rustup component add rust-std-x86_64-unknown-linux-musl

# # Install locally.
# install:
#     cargo install --path ./crates/durf-browser


# Build (mostly) static release for many versions of linux.
build-many:
    cargo zigbuild --package durf-browser --target x86_64-unknown-linux-gnu.2.28 --release

# Test the repo.
test:
    RUST_LOG=trace cargo test -- --nocapture --test-threads=1

# Publish to crates.io.
publish:
    cargo publish --workspace
