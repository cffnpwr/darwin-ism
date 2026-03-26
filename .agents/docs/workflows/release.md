# Release Workflow

## How Releases Work

This project uses [release-please](https://github.com/googleapis/release-please) for automated versioning and releases.

## Process

1. **Commit with conventional commits** — `feat:`, `fix:`, `chore:`, etc.
2. **release-please creates a PR** — automatically bumps version and generates changelog
3. **PR merges** — a draft GitHub release is created
4. **publish.yaml runs** — builds binaries for arm64 and amd64, creates universal binary via `lipo`, uploads to release
5. **Release is published** — draft status removed

## Configuration

| File | Purpose |
|------|---------|
| `.github/release-please/config.json` | Release type, PR title pattern, extra files |
| `.github/release-please/manifest.json` | Current version tracking |
| `.github/workflows/release-please.yaml` | GitHub Actions workflow |
| `.github/workflows/publish.yaml` | Build and upload artifacts |

## Version in Code

The version string in `Cargo.toml` is updated automatically by release-please via marker comments:

```toml
# x-release-please-start-version
version = "0.1.0"
# x-release-please-end
```

## Artifacts

Release artifacts include:
- `darwin-ism-arm64` — Apple Silicon binary
- `darwin-ism-amd64` — Intel binary
- `darwin-ism-universal` — Universal binary (both architectures)
