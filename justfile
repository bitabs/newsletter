# watch
watch_and_run:
    cargo watch -x check -x run

# analyze the current package and report errors, but don't build object files (faster than 'build')
check:
    cargo check

# remove generated artifacts
clean:
    cargo clean

# code coverage
coverage:
    cargo tarpaulin --ignore-tests

# lint
clippy:
    cargo clippy -- -D warnings

# formatting
fmt:
    cargo fmt -- --check