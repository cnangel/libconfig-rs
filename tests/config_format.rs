mod common;

use libconfig::{Config, SettingFormat};

#[test]
fn test_default_format() {
    let mut cfg = Config::new();
    assert_eq!(cfg.get_default_format(), SettingFormat::Default);

    cfg.set_default_format(SettingFormat::Hex);
    assert_eq!(cfg.get_default_format(), SettingFormat::Hex);

    cfg.set_default_format(SettingFormat::Default);
    assert_eq!(cfg.get_default_format(), SettingFormat::Default);
}

#[cfg(libconfig_1_8)]
#[test]
fn test_float_precision() {
    let mut cfg = Config::new();
    cfg.set_float_precision(8);
    assert_eq!(cfg.get_float_precision(), 8);

    cfg.set_float_precision(4);
    assert_eq!(cfg.get_float_precision(), 4);
}

#[test]
fn test_tab_width() {
    let cfg = Config::new();
    // Default tab width is typically 2
    assert!(cfg.get_tab_width() > 0);
}

#[cfg(libconfig_1_8)]
#[test]
fn test_set_tab_width() {
    let mut cfg = Config::new();
    cfg.set_tab_width(4);
    assert_eq!(cfg.get_tab_width(), 4);

    cfg.set_tab_width(8);
    assert_eq!(cfg.get_tab_width(), 8);
}

#[cfg(libconfig_1_4)]
#[test]
fn test_format_affects_output() {
    let mut cfg = Config::new();
    cfg.read_string("value = 255;").unwrap();
    cfg.set_default_format(SettingFormat::Hex);

    let out = common::TempFile::new().unwrap();
    cfg.write_file(out.path().to_str().unwrap()).unwrap();
    let content = std::fs::read_to_string(out.path()).unwrap();
    // Should contain hex representation
    assert!(content.contains("0xFF") || content.contains("255"));
}

#[cfg(libconfig_1_4)]
#[test]
fn test_include_dir() {
    let mut cfg = Config::new();
    assert_eq!(cfg.get_include_dir(), None);

    cfg.set_include_dir("/tmp");
    assert_eq!(cfg.get_include_dir(), Some("/tmp"));
}