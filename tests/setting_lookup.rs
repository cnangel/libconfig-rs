mod common;

use common::*;

#[cfg(libconfig_1_5)]
#[test]
fn test_setting_lookup_by_path() {
    let cfg = new_config_with_data();
    let app = cfg.lookup("application").unwrap();
    let window = app.lookup("window");
    assert!(window.is_some());
    assert_eq!(window.unwrap().get_member("title").unwrap().get_string(), "MyApp");
}

#[cfg(libconfig_1_5)]
#[test]
fn test_setting_lookup_deep_path() {
    let cfg = new_config_with_data();
    let app = cfg.lookup("application").unwrap();
    let title = app.lookup("window.title");
    assert!(title.is_some());
    assert_eq!(title.unwrap().get_string(), "MyApp");
}

#[cfg(libconfig_1_4)]
#[test]
fn test_setting_lookup_int() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    assert_eq!(window.lookup_int("width"), Some(800));
    assert_eq!(window.lookup_int("height"), Some(600));
    assert_eq!(window.lookup_int("nonexistent"), None);
}

#[cfg(libconfig_1_4)]
#[test]
fn test_setting_lookup_float() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    let val = window.lookup_float("opacity").unwrap();
    assert!((val - 0.95).abs() < 0.001);
}

#[cfg(libconfig_1_4)]
#[test]
fn test_setting_lookup_bool() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    assert_eq!(window.lookup_bool("fullscreen"), Some(false));
}

#[cfg(libconfig_1_4)]
#[test]
fn test_setting_lookup_string() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    assert_eq!(window.lookup_string("title"), Some("MyApp"));
}

#[cfg(libconfig_1_4)]
#[test]
fn test_setting_lookup_int64() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    assert_eq!(window.lookup_int64("width"), Some(800));
}