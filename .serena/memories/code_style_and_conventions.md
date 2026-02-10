# Code Style and Conventions

## Swift Code Style
- **Indentation:** 2 spaces (configured in .swiftformat and .editorconfig)
- **Trailing commas:** Always use for multiline collections/parameters
- **Line endings:** LF
- **Charset:** UTF-8
- **Final newline:** Always insert

## Naming Conventions
- Swift standard naming: camelCase for variables/functions, PascalCase for types
- Enum cases: camelCase
- File names match primary type name (e.g., InputSource.swift for struct InputSource)

## Code Patterns
- Use `enum` for stateless manager types (e.g., `InputSourceManager`)
- Use `struct` for value types (e.g., `InputSource`)
- Use `@main` attribute for CLI entry point
- Leverage swift-argument-parser decorators (@Argument, @Option, @Flag)
- Safe property access with fallback to "Unknown" for CF type conversions

## Linting (SwiftLint - Strict Mode)
Warnings are treated as errors. Key opt-in rules:
- Performance: contains_over_filter_count, empty_count, first_where, last_where, sorted_first_last, reduce_into
- Code quality: force_unwrapping, identical_operands, yoda_condition
- Modern Swift: shorthand_optional_binding, implicit_return, toggle_bool
- Style: sorted_imports, vertical_whitespace_closing_braces/opening_braces

Disabled rules: trailing_comma (handled by SwiftFormat)

## Formatting (SwiftFormat)
Configuration in `.swiftformat`:
- 2-space indent
- Always trailing commas
- Exclude .build directory

## Commit Convention
Conventional commits with gitmoji:
- `feat: :sparkles: description` for new features
- `fix: :bug: description` for bug fixes
- `ci: :green_heart: description` for CI changes
- Japanese descriptions are used

## Documentation
- Dual language: English (README.md) and Japanese (README-ja.md)
- Code comments in English or Japanese as appropriate
