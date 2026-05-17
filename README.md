# libconfig-rs

Safe Rust bindings for the [libconfig](https://hyperrealm.github.io/libconfig/) C configuration library.

## Features

- Full libconfig 1.1.x ~ 1.8.x API coverage
- Compile-time lifetime safety — `Setting<'config>` borrows from `Config`
- Type-safe enums for all libconfig constants (`SettingType`, `SettingFormat`, `ConfigOptions`)
- Version-gated features via `build.rs` pkg-config detection
- Comprehensive test suite (84 tests)

## Requirements

- libconfig >= 1.1 (1.8.x recommended for full API)
- Rust >= 1.70
- pkg-config

### RHEL/CentOS

```bash
yum install libconfig-devel
```

Or

```bash
dnf install libconfig-devel
```

### Debian/Ubuntu

```bash
apt install libconfig-dev
```

## Usage

```rust
use libconfig::Config;

let mut cfg = Config::new();

// Read from file
cfg.read_file("example.cfg").expect("failed to read config");

// Read from string
cfg.read_string(r#"
    version = "1.0";
    database: {
        host = "localhost";
        port = 5432;
    };
"#).unwrap();

// Lookup values
let host = cfg.lookup_string("database.host").unwrap();
assert_eq!(host, "localhost");

// Navigate settings
let db = cfg.lookup("database").unwrap();
for child in &db {
    println!("{} = {:?}", child.get_name().unwrap(), child.get_type());
}

// Write to file
cfg.write_file("output.cfg").unwrap();
```

## Version Compatibility

| Feature | libconfig |
|---------|-----------|
| Basic read/write/lookup | 1.1+ |
| read_string, include_dir, error_file/type, default_format | 1.4+ |
| setting lookup(path), setting format | 1.5+ |
| options, float_precision, tab_width, safe getters, clear | 1.8+ |

## COPYRIGHT AND LICENCE

* Copyright (C) 2026 cnangel

* This program is released under the following license: bsd
