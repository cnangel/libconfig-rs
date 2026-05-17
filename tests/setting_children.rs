mod common;

use common::*;
use libconfig::SettingType;

#[test]
fn test_get_elem_by_index() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    // Should be able to access children by index
    let child = window.get_elem(0);
    assert!(child.is_some());
}

#[test]
fn test_get_elem_out_of_bounds() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    let len = window.len();
    assert!(window.get_elem(len).is_none());
}

#[test]
fn test_get_member() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    let title = window.get_member("title");
    assert!(title.is_some());
    assert_eq!(title.unwrap().get_string(), "MyApp");

    assert!(window.get_member("nonexistent").is_none());
}

#[test]
fn test_exists_on_setting() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    assert!(window.exists("title"));
    assert!(!window.exists("nonexistent"));
}

#[test]
fn test_iterate_children() {
    let cfg = new_config_with_data();
    let root = cfg.get_root();
    let mut count = 0;
    for child in &root {
        count += 1;
        assert!(child.get_name().is_some());
    }
    assert!(count >= 2); // version + application
    assert_eq!(count as u32, root.len());
}

#[test]
fn test_add_child() {
    let cfg = new_config_with_data();
    let window = cfg.lookup("application.window").unwrap();
    assert!(window.exists("title"));
}

#[cfg(libconfig_1_4)]
#[test]
fn test_add_and_remove() {
    let mut cfg = libconfig::Config::new();
    cfg.read_string("app: { name = \"test\"; };")
        .unwrap();

    {
        let app = cfg.lookup("app").unwrap();
        let num_children = app.len();
        let mut app_mut: libconfig::Setting<'_> = unsafe { std::mem::transmute(app) };
        let new_setting = app_mut.add("version", SettingType::String);
        assert!(new_setting.is_some());
        drop(app_mut);

        let app_after = cfg.lookup("app").unwrap();
        assert_eq!(app_after.len(), num_children + 1);
        assert!(app_after.exists("version"));
    }

    // Remove it
    {
        let app = cfg.lookup("app").unwrap();
        let mut app_mut: libconfig::Setting<'_> = unsafe { std::mem::transmute(app) };
        app_mut.remove("version").unwrap();
        drop(app_mut);
    }

    let app_final = cfg.lookup("app").unwrap();
    assert!(!app_final.exists("version"));
}

#[test]
fn test_array_iteration() {
    let cfg = new_config_with_data();
    let features = cfg.lookup("application.features").unwrap();
    assert!(features.is_array());
    let mut items = vec![];
    for child in &features {
        items.push(child.get_string().to_string());
    }
    assert_eq!(items.len(), 3);
    assert!(items.contains(&"saving".to_string()));
    assert!(items.contains(&"printing".to_string()));
}

#[test]
fn test_list_iteration() {
    let cfg = new_config_with_data();
    let misc = cfg.lookup("application.misc").unwrap();
    assert!(misc.is_list());
    let children: Vec<_> = misc.into_iter().collect();
    assert_eq!(children.len(), 4);
    assert_eq!(children[0].get_int(), 1);
    assert_eq!(children[1].get_string(), "two");
    assert_eq!(children[2].get_bool(), true);
    assert!((children[3].get_float() - 3.14).abs() < 0.001);
}