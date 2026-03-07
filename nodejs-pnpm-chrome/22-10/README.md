# nodejs-pnpm-chrome/22-10

`ghcr.io/mwognicki/nodejs-pnpm-chrome` is a Node.js + pnpm image with Google Chrome installed, based on `ghcr.io/mwognicki/nodejs-pnpm:22-10`.

This image is tailored for Puppeteer applications and browser-driven automation workloads, with runtime dependencies preinstalled for smooth Chrome execution.

## Build

From the repository root:

```bash
docker build -f nodejs-pnpm-chrome/22-10/Dockerfile -t ghcr.io/mwognicki/nodejs-pnpm-chrome:22-10 .
```

## Verify

```bash
docker run --rm ghcr.io/mwognicki/nodejs-pnpm-chrome:22-10 /bin/sh -lc "node --version && pnpm --version && google-chrome --version"
```

## Published Image Tags

The CI publish workflow pushes `ghcr.io/mwognicki/nodejs-pnpm-chrome` with:

- `22-10`
- `latest`
- `sha-<commit>`
