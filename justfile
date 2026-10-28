# Install pinned tools and Git hooks.
setup:
    mise install
    mise exec -- prek install

# Run the content checks and the tests, as CI does.
check: content test

# Run the repository-owned content checks.
content:
    prek run --all-files --stage manual

# Use the same checks on the staged files without calling the Git hook again.
pre-commit:
    prek run --stage manual

# Run the tests, which exercise contributor commands in disposable Git indexes.
test:
    python3 -m unittest discover -s tests
