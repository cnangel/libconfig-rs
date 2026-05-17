# ChangeLog

## 0.1.0 — 2026-05-16

- Initial Rust binding for libconfig with full API surface
- Support libconfig 1.1.x ~ 1.8.x with compile-time version gating
- Safe `Config` and `Setting<'config>` wrappers with lifetime management
- Full Setting API: navigation, type checks, value getters/setters, elements
- Type-safe enums: `SettingType`, `SettingFormat`, `ConfigOptions`, `ErrorType`
- Config options: auto_convert, float_precision, tab_width, include_dir
- Safe getters (1.8+): `get_int_safe`, `get_float_safe`, `get_string_safe`, etc.
- Setting format control (1.5+): hex, binary, octal
- Setting-level lookup (1.4+): `lookup_int`, `lookup_float`, `lookup_bool`, `lookup_string`
- Comprehensive test suite with version-gated test coverage
- DEB and RPM packaging support