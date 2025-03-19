mod backend "backend/justfile"
mod frontend "frontend/justfile"

run-test-container:
    podman run --rm -d --memory=1m --name test-container-1 alpine sh -c "trap 'exit 0' TERM; while true; do sleep 1; done"
    podman run --rm -d --memory=1m --name test-container-2 alpine sh -c "trap 'exit 0' TERM; while true; do sleep 1; done"
    podman run --rm -d --memory=1m --name test-container-3 alpine sh -c "trap 'exit 0' TERM; while true; do sleep 1; done"

stop-test-container:
    podman stop test-container-1 || true
    podman stop test-container-2 || true
    podman stop test-container-3 || true
