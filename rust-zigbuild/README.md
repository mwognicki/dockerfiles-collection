# rust-zigbuild

`ghcr.io/mwognicki/rust-zigbuild:latest` is a Rust build image with Zig and `cargo-zigbuild` preinstalled for cross-compilation.

It includes common native build dependencies, Rust stable toolchain, `rustfmt` and `clippy`, Zig, and popular Linux targets for GNU and MUSL.

## Build

From the repository root:

```bash
docker build -f rust-zigbuild/Dockerfile -t ghcr.io/mwognicki/rust-zigbuild:latest .
```

## Verify

```bash
docker run --rm ghcr.io/mwognicki/rust-zigbuild:latest /bin/sh -lc "rustc --version && zig version && cargo zigbuild --version"
```

## Usage Example

```bash
docker run --rm -v "$(pwd):/workspace" -w /workspace ghcr.io/mwognicki/rust-zigbuild:latest \
  cargo zigbuild --release --target x86_64-unknown-linux-musl
```

## Notes

- Installs Zig `${ZIG_VERSION}` during build.
- Preinstalls targets:
  - `x86_64-unknown-linux-musl`
  - `x86_64-unknown-linux-gnu`
  - `aarch64-unknown-linux-musl`
  - `aarch64-unknown-linux-gnu`
