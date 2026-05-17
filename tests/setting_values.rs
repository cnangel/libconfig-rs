mod common;

use common::*;

#[test]
fn test_get_int() {
    let cfg = new_config_with_data();
    let width = cfg.lookup("application.window.width").unwrap();
    assert_eq!(width.get_int(), 800);
    assert_eq!(width.get_int64(), 800);
}

#[test]
fn test_get_float() {
    let cfg = new_config_with_data();
    let opacity = cfg.lookup("application.window.opacity").unwrap();
    assert!((opacity.get_float() - 0.95).abs() < 0.001);
}

#[test]
fn test_get_bool() {
    let cfg = new_config_with_data();
    let fs = cfg.lookup("application.window.fullscreen").unwrap();
    assert!(!fs.get_bool());
}

#[test]
fn test_get_string() {
    let cfg = new_config_with_data();
    let title = cfg.lookup("application.window.title").unwrap();
    assert_eq!(title.get_string(), "MyApp");
}

#[test]
fn test_set_int() {
    let mut cfg = libconfig::Config::new();
    cfg.read_string("value = 42;").unwrap();

    let mut mut_val: libconfig::Setting<'_> =
        unsafe { std::mem::transmute(cfg.lookup("value").unwrap()) };
    mut_val.set_int(100).unwrap();
    drop(mut_val);

    assert_eq!(cfg.lookup_int("value"), Some(100));
}

#[test]
fn test_set_float() {
    let mut cfg = libconfig::Config::new();
    cfg.read_string("value = 1.0;").unwrap();

    let mut mut_val: libconfig::Setting<'_> =
        unsafe { std::mem::transmute(cfg.lookup("value").unwrap()) };
    mut_val.set_float(3.14).unwrap();
    drop(mut_val);

    let val = cfg.lookup_float("value").unwrap();
    assert!((val - 3.14).abs() < 0.001);
}

#[test]
fn test_set_bool() {
    let mut cfg = libconfig::Config::new();
    cfg.read_string("value = false;").unwrap();

    let mut mut_val: libconfig::Setting<'_> =
        unsafe { std::mem::transmute(cfg.lookup("value").unwrap()) };
    mut_val.set_bool(true).unwrap();
    drop(mut_val);

    assert_eq!(cfg.lookup_bool("value"), Some(true));
}

#[test]
fn test_set_string() {
    let mut cfg = libconfig::Config::new();
    cfg.read_string("value = \"old\";").unwrap();

    let mut mut_val: libconfig::Setting<'_> =
        unsafe { std::mem::transmute(cfg.lookup("value").unwrap()) };
    mut_val.set_string("new").unwrap();
    drop(mut_val);

    assert_eq!(cfg.lookup_string("value"), Some("new"));
}

#[test]
fn test_set_int64() {
    let mut cfg = libconfig::Config::new();
    cfg.read_string("value = 42L;").unwrap();

    let mut mut_val: libconfig::Setting<'_> =
        unsafe { std::mem::transmute(cfg.lookup("value").unwrap()) };
    mut_val.set_int64(9999999999i64).unwrap();
    drop(mut_val);

    assert_eq!(cfg.lookup_int64("value"), Some(9999999999i64));
}

#[cfg(libconfig_1_8)]
#[test]
fn test_safe_getters() {
    let cfg = new_config_with_data();
    let width = cfg.lookup("application.window.width").unwrap();
    assert_eq!(width.get_int_safe(), Some(800));
    assert_eq!(width.get_string_safe(), None); // not a string

    let title = cfg.lookup("application.window.title").unwrap();
    assert_eq!(title.get_string_safe(), Some("MyApp"));
    assert_eq!(title.get_int_safe(), None); // not an int

    let fs = cfg.lookup("application.window.fullscreen").unwrap();
    assert_eq!(fs.get_bool_safe(), Some(false));
    assert_eq!(fs.get_float_safe(), None); // not a float

    let opacity = cfg.lookup("application.window.opacity").unwrap();
    assert!(opacity.get_float_safe().is_some());
    assert!((opacity.get_float_safe().unwrap() - 0.95).abs() < 0.001);
    assert_eq!(opacity.get_int64_safe(), None); // not int64
}

#[cfg(libconfig_1_8)]
#[test]
fn test_setting_roundtrip() {
    let mut cfg = libconfig::Config::new();
    cfg.set_auto_convert(true);
    cfg.read_string("int_val = 0; float_val = 0.0;").unwrap();

    {
        let mut s: libconfig::Setting<'_> =
            unsafe { std::mem::transmute(cfg.lookup("int_val").unwrap()) };
        s.set_int(42).unwrap();
        drop(s);
    }
    assert_eq!(cfg.lookup_int("int_val"), Some(42));

    {
        let mut s: libconfig::Setting<'_> =
            unsafe { std::mem::transmute(cfg.lookup("float_val").unwrap()) };
        s.set_float(2.718).unwrap();
        drop(s);
    }
    let val = cfg.lookup_float("float_val").unwrap();
    assert!((val - 2.718).abs() < 0.001);
}