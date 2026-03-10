# kubectl-helm

`ghcr.io/mwognicki/kubectl-helm:latest` is a Rocky Linux-based image with `kubectl` and `helm` for Kubernetes and Helm-driven automation.

The image uses a multi-stage build, verifies downloaded binaries with checksums, and ships a lean runtime with a non-root `ciuser` plus pre-created Helm and kube config directories.

## Build

From the repository root:

```bash
docker build -f kubectl-helm/Dockerfile -t ghcr.io/mwognicki/kubectl-helm:latest .
```

## Verify

```bash
docker run --rm ghcr.io/mwognicki/kubectl-helm:latest /bin/sh -lc "kubectl version --client && helm version"
```

## Published Image Tags

The CI publish workflow pushes `ghcr.io/mwognicki/kubectl-helm` with:

- `latest`
- `sha-<commit>`
