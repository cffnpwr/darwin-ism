# Task Completion Checklist

When a coding task is completed, run the following checks:

## 1. Build
```bash
swift build
```
Ensure the project compiles without errors.

## 2. Format Check
```bash
treefmt --fail-on-change
```
Or format and apply:
```bash
treefmt
```

## 3. Lint Check
```bash
swiftlint lint --config .swiftlint.yaml --strict
```
All warnings are treated as errors in strict mode.

## 4. Manual Verification
Since there are no automated tests, verify behavior manually if the change affects CLI functionality.

## Notes
- Pre-commit hooks (lefthook) will run format and lint automatically on commit
- CI runs build (arm64 + amd64), swiftlint (strict), and format check
- Version consistency is checked by scripts/check-swift-version.sh
