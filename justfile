backend *FLAGS:
    cargo run {{FLAGS}}

test:
    cargo test --release

test-container:
    podman run --rm --name test-container alpine sh -c "trap 'exit 0' TERM; while true; do sleep 1; done"
