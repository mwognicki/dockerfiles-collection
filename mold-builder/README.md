# mold-builder

`ghcr.io/mwognicki/mold-builder` is a Rocky Linux 10-based build image that installs:

- `mold` (built from `rui314/mold` stable branch)
- LLVM/Clang toolchain
- Common native build dependencies (`cmake`, `make`, `gcc`, `gcc-c++`, `git`, `curl`)
- Rust via `rustup` (stable toolchain)

The image configures Cargo to use `clang` as the linker and enables `mold` via `-fuse-ld=mold`.

## Build

From the repository root:

```bash
docker build -f mold-builder/Dockerfile -t ghcr.io/mwognicki/mold-builder:latest .
```

## Verify

```bash
docker run --rm ghcr.io/mwognicki/mold-builder:latest /bin/sh -lc "mold --version && rustc --version && clang --version"
```

## Usage Example

Use the image as a base for Rust projects that benefit from faster link times:

```Dockerfile
FROM ghcr.io/mwognicki/mold-builder:latest
WORKDIR /workspace
COPY . .
RUN cargo build --release
```

## Notes

- The Dockerfile clones the `stable` branch of `mold` with `--depth 1`.
- Cargo linker settings are written to `/root/.cargo/config.toml`.
