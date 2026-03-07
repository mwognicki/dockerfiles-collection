# python/3.12/red-quartz

`ghcr.io/mwognicki/python` is a Python 3.12 container image based on Rocky Linux 10 with `pip` preinstalled.

## Build

From the repository root:

```bash
docker build -f python/3.12/red-quartz/Dockerfile -t ghcr.io/mwognicki/python:3.12-redquartz .
```

## Verify

```bash
docker run --rm ghcr.io/mwognicki/python:3.12-redquartz /bin/sh -lc "python --version && pip --version"
```

## Published Image Tags

The CI publish workflow pushes `ghcr.io/mwognicki/python` with the following tags:

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
