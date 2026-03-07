# micromamba

`ghcr.io/mwognicki/micromamba` is a Rocky-based image built on `ghcr.io/mwognicki/python:3.12-redquartz` with `micromamba` preinstalled.

## Build

From the repository root:

```bash
docker build -f micromamba/Dockerfile -t ghcr.io/mwognicki/micromamba:3.12-redquartz .
```

## Verify

```bash
docker run --rm ghcr.io/mwognicki/micromamba:3.12-redquartz /bin/sh -lc "python --version && micromamba --version"
```

## Published Image Tags

The CI publish workflow pushes `ghcr.io/mwognicki/micromamba` with the following tags:

- `latest`
- `3.12-redquartz`
- `3.12`
- `3-redquartz`
- `3`
- `3.12-rocky`
- `3-rocky`
- `3.12-rhel`
- `3-rhel`
- `redquartz`
- `rocky`
- `rhel`
