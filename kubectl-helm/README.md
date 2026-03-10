# kubectl-helm

`ghcr.io/mwognicki/kubectl-helm:latest` is a Rocky Linux-based image with `kubectl` and `helm` preinstalled for Kubernetes and Helm-driven automation.

The image includes a non-root `ciuser`, pre-created Helm and kube config directories, and smoke-tests both tools during build.

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
