mod common;

use common::*;
use libconfig::SettingType;

#[test]
fn test_get_type() {
    let cfg = new_config_with_data();
    let root = cfg.get_root();
    assert_eq!(root.get_type(), SettingType::Group);

    let title = cfg.lookup("application.window.title").unwrap();
    assert_eq!(title.get_type(), SettingType::String);

    let width = cfg.lookup("application.window.width").unwrap();
    assert_eq!(width.get_type(), SettingType::Int);

    let opacity = cfg.lookup("application.window.opacity").unwrap();
    assert_eq!(opacity.get_type(), SettingType::Float);

    let fullscreen = cfg.lookup("application.window.fullscreen").unwrap();
    assert_eq!(fullscreen.get_type(), SettingType::Bool);

    let features = cfg.lookup("application.features").unwrap();
    assert_eq!(features.get_type(), SettingType::Array);

    let misc = cfg.lookup("application.misc").unwrap();
    assert_eq!(misc.get_type(), SettingType::List);
}

#[test]
fn test_type_predicates() {
    let cfg = new_config_with_data();

    let root = cfg.get_root();
    assert!(root.is_group());
    assert!(root.is_aggregate());
    assert!(!root.is_scalar());
    assert!(!root.is_number());

    let width = cfg.lookup("application.window.width").unwrap();
    assert!(width.is_scalar());
    assert!(width.is_number());
    assert!(!width.is_aggregate());

    let title = cfg.lookup("application.window.title").unwrap();
    assert!(title.is_string());
    assert!(title.is_scalar());
}

#[test]
fn test_get_name() {
    let cfg = new_config_with_data();
    let root = cfg.get_root();
    assert_eq!(root.get_name(), None);

    let window = cfg.lookup("application.window").unwrap();
    assert_eq!(window.get_name(), Some("window"));

    let title = cfg.lookup("application.window.title").unwrap();
    assert_eq!(title.get_name(), Some("title"));
}

#[test]
fn test_parent_and_is_root() {
    let cfg = new_config_with_data();
    let root = cfg.get_root();
    assert!(root.is_root());
    assert!(root.parent().is_none());

    let window = cfg.lookup("application.window").unwrap();
    assert!(!window.is_root());
    let parent = window.parent().unwrap();
    assert_eq!(parent.get_name(), Some("application"));
}

#[test]
fn test_get_index() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    // Index should be >= 0 within its parent
    assert!(window.get_index() >= 0);

    let title = cfg.lookup("application.window.title").unwrap();
    assert!(title.get_index() >= 0);
}

#[test]
fn test_source_line() {
    let cfg = new_config_with_data();
    let title = cfg.lookup("application.window.title").unwrap();
    // source_line should be > 0 for parsed settings
    assert!(title.source_line() > 0);
}

#[test]
fn test_source_file() {
    let (cfg, _file) = temp_config_file("x = 1;");
    let x = cfg.lookup("x").unwrap();
    #[cfg(libconfig_1_4)]
    {
        let file = x.source_file();
        assert!(file.is_some());
    }
    #[cfg(not(libconfig_1_4))]
    {
        // source_file not available — just verify no crash
        let _ = x;
    }
}

#[test]
fn test_len_and_is_empty() {
    let cfg = new_config_with_data();
    let root = cfg.get_root();
    // Root has version and application
    assert!(root.len() >= 2);
    assert!(!root.is_empty());

    let window = cfg.lookup("application.window").unwrap();
    assert!(window.len() >= 4);
    assert!(!window.is_empty());
}