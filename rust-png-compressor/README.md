# png-compressor

`ghcr.io/mwognicki/png-compressor:latest` is a minimal container image that ships the `png-compressor` Rust CLI utility.

The image builds a static MUSL binary from `src/png-compressor` and copies only the resulting executable into the runtime image.
The utility scans a given directory, compresses `.png` files to stay under a size limit, and creates backup copies for files it modifies.

## Build

From the repository root:

```bash
docker build -f rust-png-compressor/Dockerfile -t ghcr.io/mwognicki/png-compressor:latest .
```

## CLI Usage

```bash
png-compressor <path> <max_size>
```

Arguments:

- `<path>`: Directory containing PNG files to process.
- `<max_size>`: Maximum per-file size (human-readable), for example `2MB`, `200KB`, or raw bytes like `1048576`.

Container example:

```bash
docker run --rm -v "$(pwd)/images:/work/images" ghcr.io/mwognicki/png-compressor:latest png-compressor /work/images 2MB
```

Behavior summary:

- Skips files already within limit.
- Creates `<name>.backup.png` next to each file it modifies.
- First tries lossless optimization; if still too large, progressively rescales until it fits or reaches the minimum attempt.

## Notes

- Build target: `x86_64-unknown-linux-musl`
- Source location: `rust-png-compressor/src/png-compressor`
