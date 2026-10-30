# Install pinned tools and Git hooks.
setup:
    mise install
    mise exec -- prek install

# Run the content checks, the Rust gate and the tests, as CI does.
check: content rust test

# Run the repository-owned content checks.
content:
    prek run --all-files --stage manual

# Use the same checks on the staged files without calling the Git hook again.
pre-commit:
    prek run --stage manual

# Check Rust formatting and lints, with warnings denied.
rust:
    cargo fmt --check
    cargo clippy --all-targets --locked -- -D warnings

# Run the Rust tests, which exercise contributor commands in disposable Git indexes.
test:
    cargo test --locked
