# dockerfiles-collection

Collection of Dockerfiles for various use cases.

Each image lives in its own directory with:

- `Dockerfile`
- `README.md` describing purpose, tooling, and usage

## Included images

- [mold-builder](mold-builder/README.md): Rocky Linux 10 image with `mold`, `clang`, and Rust toolchain configured for fast linking.
- [nodejs-pnpm/22-10](nodejs-pnpm/22-10/README.md): Rocky Linux 10 image with Node.js 22 and `pnpm` 10.
