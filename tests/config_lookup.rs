mod common;

use common::*;

#[test]
fn test_lookup_path() {
    let cfg = new_config_with_data();
    let setting = cfg.lookup("application.window.title");
    assert!(setting.is_some());
    assert_eq!(setting.unwrap().get_string(), "MyApp");
}

#[test]
fn test_lookup_nonexistent() {
    let cfg = new_config_with_data();
    assert!(cfg.lookup("nonexistent.path").is_none());
}

#[test]
fn test_get_root() {
    let cfg = new_config_with_data();
    let root = cfg.get_root();
    assert!(root.is_root());
    assert!(root.is_group());
}

#[test]
fn test_lookup_int() {
    let cfg = new_config_with_data();
    assert_eq!(cfg.lookup_int("application.window.width"), Some(800));
    assert_eq!(cfg.lookup_int("application.window.height"), Some(600));
    assert_eq!(cfg.lookup_int("nonexistent"), None);
}

#[test]
fn test_lookup_int64() {
    let cfg = new_config_with_data();
    assert_eq!(cfg.lookup_int64("application.window.width"), Some(800));
    assert_eq!(cfg.lookup_int64("nonexistent"), None);
}

#[test]
fn test_lookup_float() {
    let cfg = new_config_with_data();
    let val = cfg.lookup_float("application.window.opacity");
    assert!(val.is_some());
    assert!((val.unwrap() - 0.95).abs() < 0.001);
    assert_eq!(cfg.lookup_float("nonexistent"), None);
}

#[test]
fn test_lookup_bool() {
    let cfg = new_config_with_data();
    assert_eq!(cfg.lookup_bool("application.window.fullscreen"), Some(false));
    assert_eq!(cfg.lookup_bool("nonexistent"), None);
}

#[test]
fn test_lookup_string() {
    let cfg = new_config_with_data();
    assert_eq!(cfg.lookup_string("version"), Some("1.0"));
    assert_eq!(cfg.lookup_string("application.window.title"), Some("MyApp"));
    assert_eq!(cfg.lookup_string("nonexistent"), None);
}

#[test]
fn test_exists() {
    let cfg = new_config_with_data();
    assert!(cfg.exists("application.window"));
    assert!(cfg.exists("version"));
    assert!(!cfg.exists("nope.nope"));
}