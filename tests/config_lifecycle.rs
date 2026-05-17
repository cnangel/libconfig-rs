mod common;

use common::*;
use libconfig::Config;

#[test]
fn test_new_and_default() {
    let _cfg = Config::new();
    let _cfg2 = Config::default();
}

#[cfg(libconfig_1_4)]
#[test]
fn test_read_string_sample() {
    let mut cfg = Config::new();
    cfg.read_string(sample_config_str())
        .expect("read_string should succeed");
    assert!(cfg.exists("application"));
    assert!(cfg.exists("version"));
}

#[test]
fn test_read_file_and_write_file() {
    let (cfg, file) = temp_config_file(sample_config_str());
    assert!(cfg.exists("application"));

    // Write back to a different file
    let out = tempfile::NamedTempFile::new().unwrap();
    cfg.write_file(out.path().to_str().unwrap())
        .expect("write_file should succeed");

    // Read it back
    let mut cfg2 = Config::new();
    cfg2.read_file(out.path().to_str().unwrap())
        .expect("read back should succeed");
    assert!(cfg2.exists("application"));
    assert_eq!(
        cfg2.lookup_string("application.window.title"),
        Some("MyApp")
    );

    drop(file); // keep temp file alive until here
}

#[test]
fn test_read_file_not_found() {
    let mut cfg = Config::new();
    let result = cfg.read_file("/nonexistent/path/to/config.cfg");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(!err.message.is_empty());
}

#[cfg(libconfig_1_4)]
#[test]
fn test_read_string_invalid() {
    let mut cfg = Config::new();
    let result = cfg.read_string("this is not a valid config {{{");
    assert!(result.is_err());
}

#[cfg(libconfig_1_4)]
#[test]
fn test_error_state() {
    let mut cfg = Config::new();
    let _ = cfg.read_string("invalid {{{");
    // After error, error_text should be set
    assert!(!cfg.error_text().is_empty());
}

#[cfg(libconfig_1_8)]
#[test]
fn test_clear_reuse() {
    let mut cfg = Config::new();
    cfg.read_string("version = \"1.0\";").unwrap();
    assert!(cfg.exists("version"));

    cfg.clear();

    // After clear, the old data should be gone
    assert!(!cfg.exists("version"));

    // Can reuse the config
    cfg.read_string("name = \"test\";").unwrap();
    assert_eq!(cfg.lookup_string("name"), Some("test"));
}

#[cfg(libconfig_1_4)]
#[test]
fn test_write_file_preserves_structure() {
    let mut cfg = Config::new();
    cfg.read_string(sample_config_str()).unwrap();

    let out = tempfile::NamedTempFile::new().unwrap();
    cfg.write_file(out.path().to_str().unwrap()).unwrap();

    let content = std::fs::read_to_string(out.path()).unwrap();
    assert!(content.contains("MyApp"));
    assert!(content.contains("800"));
}