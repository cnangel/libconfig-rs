mod common;

#[cfg(libconfig_1_8)]
use libconfig::{Config, ConfigOptions};

#[cfg(libconfig_1_8)]
#[test]
fn test_set_and_get_options() {
    let mut cfg = Config::new();
    let opts = ConfigOptions::AUTOCONVERT
        | ConfigOptions::SEMICOLON_SEPARATORS
        | ConfigOptions::FSYNC;
    cfg.set_options(opts);
    let got = cfg.get_options();
    assert!(got.contains(ConfigOptions::AUTOCONVERT));
    assert!(got.contains(ConfigOptions::SEMICOLON_SEPARATORS));
    assert!(got.contains(ConfigOptions::FSYNC));
    assert!(!got.contains(ConfigOptions::ALLOW_OVERRIDES));
}

#[cfg(libconfig_1_8)]
#[test]
fn test_set_and_get_option() {
    let mut cfg = Config::new();
    cfg.set_option(ConfigOptions::ALLOW_SCIENTIFIC_NOTATION, true);
    assert!(cfg.get_option(ConfigOptions::ALLOW_SCIENTIFIC_NOTATION));
    cfg.set_option(ConfigOptions::ALLOW_SCIENTIFIC_NOTATION, false);
    assert!(!cfg.get_option(ConfigOptions::ALLOW_SCIENTIFIC_NOTATION));
}

#[cfg(libconfig_1_8)]
#[test]
fn test_auto_convert() {
    let mut cfg = Config::new();
    cfg.set_auto_convert(true);
    assert!(cfg.get_auto_convert());
    cfg.set_auto_convert(false);
    assert!(!cfg.get_auto_convert());
}

#[cfg(libconfig_1_8)]
#[test]
fn test_options_persist_after_clear() {
    let mut cfg = Config::new();
    cfg.set_option(ConfigOptions::OPEN_BRACE_ON_SEPARATE_LINE, true);
    cfg.read_string("x = 1;").unwrap();
    cfg.clear();
    // Options persist after clear
    assert!(cfg.get_option(ConfigOptions::OPEN_BRACE_ON_SEPARATE_LINE));
}