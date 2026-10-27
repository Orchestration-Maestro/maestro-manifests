# Install pinned tools and Git hooks.
setup:
    mise install
    mise exec -- prek install

# Run the repository-owned content checks used by CI.
check:
    prek run --all-files --stage manual

# Use the same checks on the staged files without calling the Git hook again.
pre-commit:
    prek run --stage manual

# Exercise contributor commands in disposable Git indexes.
test:
    python3 -m unittest discover -s tests
