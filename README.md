# dockerfiles-collection

Collection of Dockerfiles for various use cases.

Each image lives in its own directory with:

- `Dockerfile`
- `README.md` describing purpose, tooling, and usage

## Included images

- [mold-builder](mold-builder/README.md): Rocky Linux 10 image with `mold`, `clang`, and Rust toolchain configured for fast linking.
- [nodejs-pnpm/22-10](nodejs-pnpm/22-10/README.md): Rocky Linux 10 image with Node.js 22 and `pnpm` 10.
- [nodejs-pnpm-chrome/22-10](nodejs-pnpm-chrome/22-10/README.md): `ghcr.io/mwognicki/nodejs-pnpm-chrome` image tailored for Puppeteer/browser automation.
- [rust-zigbuild](rust-zigbuild/README.md): `ghcr.io/mwognicki/rust-zigbuild` image with Rust, Zig, and `cargo-zigbuild` for cross-compilation.
- [python/3.12/red-quartz](python/3.12/red-quartz/README.md): `ghcr.io/mwognicki/python` image (Python 3.12 red-quartz variant) with multi-tag publishing.
- [micromamba](micromamba/README.md): `ghcr.io/mwognicki/micromamba` image with micromamba on top of the Python 3.12 red-quartz base.
