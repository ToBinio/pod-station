mod backend "backend/justfile"
mod frontend "frontend/justfile"

test-container:
    podman run --rm --name test-container alpine sh -c "trap 'exit 0' TERM; while true; do sleep 1; done"
