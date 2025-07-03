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

An example crate was added in `examples/simulator` to restore functionaly of the feature.

### Type of `EmbeddedBackendConfig::font_bold` is now `Option<MonoFont<'static>>` ([#])

[#]: https://github.com/j-g00da/mousefood/pull/

### `EmbeddedBackendConfig` now requires providing `font_italic` ([#])

[#]: https://github.com/j-g00da/mousefood/pull/

### `ratatui` is no longer re-exported ([#60])

[#60]: https://github.com/j-g00da/mousefood/pull/60

Mousefood now depends on `ratatui-core` crate instead of `ratatui` and doesn't
re-export it. Downstream crates should now depend on `ratatui` directly.

```diff
- mousefood = "0.2.1"
+ mousefood = "0.3.0"
+ ratatui = { version = "0.30.0", default-features = false }
```

```diff
- use mousefood::ratatui::Terminal;
+ use ratatui::Terminal;
```

### `EmbeddedBackend` now uses `mousefood::error::Error` instead of `std::io::Error` for error handling ([#60])

[#60]: https://github.com/j-g00da/mousefood/pull/60

### The MSRV is now 1.85.0 ([#])

[#]: https://github.com/j-g00da/mousefood/pull/

## [v0.2.0](https://github.com/j-g00da/mousefood/releases/tag/0.2.0)

### `EmbeddedBackend::with_font` constructor removed ([#])

[#]: https://github.com/j-g00da/mousefood/pull/

### `EmbeddedBackend::new` now requires a `config` parameter ([#])

[#]: https://github.com/j-g00da/mousefood/pull/

### `fonts::BASIC_6X10` renamed to `fonts::MONO_6X10` ([#])

[#]: https://github.com/j-g00da/mousefood/pull/

## [v0.1.0](https://github.com/j-g00da/mousefood/releases/tag/0.1.0)

### `EmbeddedBackend::new` now takes different arguments ([#])

[#]: https://github.com/j-g00da/mousefood/pull/

## [v0.0.1](https://github.com/j-g00da/mousefood/releases/tag/0.0.1) - initial release

