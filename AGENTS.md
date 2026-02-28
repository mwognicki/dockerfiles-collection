# Repository Guidelines

## Project Structure & Module Organization
This repository is a small collection of Docker build definitions.

- `mold-builder/Dockerfile`: Builds a Rocky Linux-based image with `mold`, `clang`, and Rust toolchain setup.
- `README.md`: Short repository overview.
- `LICENSE`: Project license text.

When adding new images, use one directory per image (for example, `node-builder/`) and keep each Dockerfile self-contained. Each image directory must include a `README.md` next to its `Dockerfile` describing the image, purpose, and usage.

Example:

- `node-builder/Dockerfile`
- `node-builder/README.md`

## Build, Test, and Development Commands
Run commands from the repository root.

- `docker build -f mold-builder/Dockerfile -t mold-builder .`
Builds the `mold-builder` image.
- `docker run --rm mold-builder /bin/sh -lc "mold --version && rustc --version"`
Quick smoke test to confirm toolchain installation.
- `docker image ls | rg mold-builder`
Checks that the image was created locally.

## CI/CD Workflow Convention
The GitHub Actions setup used for `mold-builder` is the standard for all image directories in this repository.

- Each Docker image directory must have a corresponding workflow in `.github/workflows/`.
- Each workflow must publish its image to GHCR.
- Triggers must be limited to pushes on `main` and path changes to:
  - that image's `Dockerfile`
  - the workflow file itself
- Root `README.md` must list and link every nested image `README.md`; update this index whenever a new nested `README.md` is added.

## Coding Style & Naming Conventions
- Use uppercase Dockerfile instructions (`FROM`, `RUN`, `ENV`).
- Chain package/install steps with `&&` and clean caches in the same layer when possible.
- Keep continuation indentation consistent (4 spaces recommended under `RUN` blocks).
- Use directory names in kebab-case for image variants (`mold-builder`, `rust-builder`, etc.).
- Sort package names logically to keep diffs easy to review.

## Testing Guidelines
Testing is performed entirely by maintainers. Contributors are not required to run build or runtime verification locally before submitting changes.

## Commit & Pull Request Guidelines
Git history is currently minimal (`Initial commit`), so use clear, imperative commit messages going forward.

- Good example: `Add mold-builder smoke test command to docs`
- Keep subject lines concise; group related changes in one commit.

For pull requests:

- Explain what image was added/changed and why.
- Include exact build/test commands you ran.
- Link related issues when applicable.
- Include logs or screenshots only when they clarify failures or environment-specific behavior.
