set working-directory := 'backend'

run *FLAGS:
    cargo run {{FLAGS}}

test:
    cargo test --release

lint:
    cargo fmt --check
    cargo clippy

fmt:
    cargo fmt

fix:
    cargo clippy --fix
