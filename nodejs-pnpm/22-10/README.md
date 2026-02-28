# nodejs-pnpm/22-10

`ghcr.io/mwognicki/nodejs-pnpm:22-10` is a Rocky Linux 10 image with Node.js 22 and `pnpm` preinstalled.

This image is intended for Node.js workspaces that use `pnpm` and need a simple, reproducible base for CI or local builds.

## Build

From the repository root:

```bash
docker build -f nodejs-pnpm/22-10/Dockerfile -t ghcr.io/mwognicki/nodejs-pnpm:22-10 .
```

## Verify

```bash
docker run --rm ghcr.io/mwognicki/nodejs-pnpm:22-10 /bin/sh -lc "node --version && npm --version && pnpm --version"
```

## Notes

- Node.js is installed from the NodeSource `setup_22.x` bootstrap script.
- `pnpm` is installed using the official install script with `PNPM_VERSION=10.26.0`.
