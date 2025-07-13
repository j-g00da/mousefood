# Breaking Changes

This document lists all breaking changes along with tips to help you migrate smoothly.

## Summary

- [v0.3.0](#v030---unreleased) - unreleased
  - Feature `simulator` is removed
  - Type of `EmbeddedBackendConfig::font_bold` is now `Option<MonoFont<'static>>`
  - `EmbeddedBackendConfig` now requires providing `font_italic`
  - `ratatui` is no longer re-exported
  - `EmbeddedBackend` now uses `mousefood::error::Error` instead of
    `std::io::Error` for error handling
  - The MSRV is now 1.85.0
- [v0.2.0](#v020)
  - `EmbeddedBackend::with_font` constructor removed
  - `EmbeddedBackend::new` now requires a `config` parameter
  - `fonts::BASIC_6X10` renamed to `fonts::MONO_6X10`
- [v0.1.0](#v010)
  - `EmbeddedBackend::new` now takes different arguments
- [v0.0.1](#v001---initial-release) - initial release

## v0.3.0 - unreleased

### Feature `simulator` is removed ([#83])

[#83]: https://github.com/j-g00da/mousefood/pull/83

The feature `simulator` is removed to simplify code of the crate.

An example crate was added in `examples/simulator` to restore functionality of the feature.

**Migration guide:**

If you were using the `simulator` feature:

```diff
[dependencies]
- mousefood = { version = "0.2.1", features = ["simulator"] }
+ mousefood = "0.3.0"
+ embedded-graphics-simulator = "0.6.0"
```

```diff
- use mousefood::simulator::SimulatorDisplay;
+ use embedded_graphics_simulator::SimulatorDisplay;
```

See the example in `examples/simulator/` for a complete migration example.

### Type of `EmbeddedBackendConfig::font_bold` is now `Option<MonoFont<'static>>` ([#57])

[#57]: https://github.com/j-g00da/mousefood/pull/57

Previously, `font_bold` was a required `MonoFont<'static>`. Now it's optional to allow
configurations where bold text is not needed or uses the same font as regular text.

**Migration guide:**

```diff
let config = EmbeddedBackendConfig {
-   font_bold: mousefood::fonts::MONO_6X13_BOLD,
+   font_bold: Some(mousefood::fonts::MONO_6X13_BOLD),
    // ...other fields
};
```

If you don't need bold text support, you can set it to `None`:

```rust
let config = EmbeddedBackendConfig {
    font_bold: None,
    // ...other fields
};
```

### `EmbeddedBackendConfig` now requires providing `font_italic` ([#57])

The `font_italic` field was added to `EmbeddedBackendConfig` to support italic text rendering. This
field is optional and can be set to `None` if italic text support is not needed.

**Migration guide:**

```diff
let config = EmbeddedBackendConfig {
    font_regular: mousefood::fonts::MONO_6X13,
    font_bold: Some(mousefood::fonts::MONO_6X13_BOLD),
+   font_italic: None, // or Some(your_italic_font),
    // ...other fields
};
```

If you have an italic font available:

```rust
let config = EmbeddedBackendConfig {
    font_regular: mousefood::fonts::MONO_6X13,
    font_bold: Some(mousefood::fonts::MONO_6X13_BOLD),
+   font_italic: Some(mousefood::fonts::MONO_6X13_ITALIC),
    // ...other fields,
};
```

### `ratatui` is no longer re-exported ([#60])

[#60]: https://github.com/j-g00da/mousefood/pull/60

Mousefood now depends on `ratatui-core` crate instead of `ratatui` and doesn't
re-export it. Downstream crates should now depend on `ratatui` directly.

**Migration guide:**

```diff
[dependencies]
- mousefood = "0.2.1"
+ mousefood = "0.3.0"
+ ratatui = { version = "0.30.0", default-features = false }
```

```diff
- use mousefood::ratatui::Terminal;
+ use ratatui::Terminal;
```

### `EmbeddedBackend` now uses `mousefood::error::Error` instead of `std::io::Error` for error handling ([#60])

The backend now uses a custom error type that better represents the kinds of errors that can occur
in embedded graphics contexts.

**Migration guide:**

```diff
- use std::io::Error
+ use mousefood::error::Error
```

### The MSRV is now 1.85.0 ([#65])

[#65]: https://github.com/j-g00da/mousefood/pull/65

The minimum supported Rust version has been updated to 1.85.0 to support Rust 2024 edition and
latest language features.

**Migration guide:**

Ensure your Rust toolchain is at least version 1.85.0:

```bash
rustup update
rustc --version  # should show 1.85.0 or higher
```

## [v0.2.0](https://github.com/j-g00da/mousefood/releases/tag/0.2.0)

### `EmbeddedBackend::with_font` constructor removed ([#48])

[#48]: https://github.com/j-g00da/mousefood/pull/48

The `with_font` constructor was removed in favor of the more flexible config-based approach.

**Migration guide:**

```diff
- let backend = EmbeddedBackend::with_font(&mut display, font);
+ let config = EmbeddedBackendConfig {
+     font_regular: font,
+     ..Default::default()
+ };
+ let backend = EmbeddedBackend::new(&mut display, config);
```

### `EmbeddedBackend::new` now requires a `config` parameter ([#48])

The constructor now takes a configuration struct for better extensibility.

**Migration guide:**

```diff
- let backend = EmbeddedBackend::new(&mut display);
+ let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());
```

For custom configurations:

```rust
let config = EmbeddedBackendConfig {
    font_regular: your_font,
    ..Default::default()
};
let backend = EmbeddedBackend::new(&mut display, config);
```

### `fonts::BASIC_6X10` renamed to `fonts::MONO_6X10` ([#26])

[#26]: https://github.com/j-g00da/mousefood/pull/26

The font constant was renamed to better reflect its monospace nature.

**Migration guide:**

```diff
- use mousefood::fonts::BASIC_6X10;
+ use mousefood::fonts::MONO_6X10;
```

```diff
let config = EmbeddedBackendConfig {
-   font_regular: BASIC_6X10,
+   font_regular: MONO_6X10,
    // ...other fields
};
```

## [v0.1.0](https://github.com/j-g00da/mousefood/releases/tag/0.1.0)

### `EmbeddedBackend::new` now takes different arguments

The `new` constructor was simplified to take fewer parameters and use sensible defaults.

**Migration guide:**

```diff
- let backend = EmbeddedBackend::new(&mut display, font_regular, font_bold);
+ let backend = EmbeddedBackend::new(&mut display);
```

Or if you need to specify custom fonts:

```diff
- let backend = EmbeddedBackend::new(&mut display, font_regular, font_bold);
+ let backend = EmbeddedBackend::with_font(&mut display, font_regular, font_bold);
```

## [v0.0.1](https://github.com/j-g00da/mousefood/releases/tag/0.0.1) - initial release
