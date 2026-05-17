mod common;

use common::*;

#[test]
fn test_get_int_elem() {
    let cfg = new_config_with_data();
    let misc = cfg.lookup("application.misc").unwrap();
    assert_eq!(misc.get_int_elem(0), Some(1));
    assert_eq!(misc.get_int_elem(10), None); // out of bounds
}

#[test]
fn test_get_string_elem() {
    let cfg = new_config_with_data();
    let misc = cfg.lookup("application.misc").unwrap();
    assert_eq!(misc.get_string_elem(1), Some("two"));
    assert_eq!(misc.get_string_elem(10), None);
}

#[test]
fn test_get_bool_elem() {
    let cfg = new_config_with_data();
    let misc = cfg.lookup("application.misc").unwrap();
    assert_eq!(misc.get_bool_elem(2), Some(true));
}

#[test]
fn test_get_float_elem() {
    let cfg = new_config_with_data();
    let misc = cfg.lookup("application.misc").unwrap();
    let val = misc.get_float_elem(3).unwrap();
    assert!((val - 3.14).abs() < 0.001);
}

#[test]
fn test_get_int64_elem() {
    let cfg = new_config_with_data();
    let misc = cfg.lookup("application.misc").unwrap();
    assert_eq!(misc.get_int64_elem(0), Some(1));
}

#[test]
fn test_array_element_types() {
    let cfg = new_config_with_data();
    let features = cfg.lookup("application.features").unwrap();
    assert_eq!(features.get_string_elem(0), Some("saving"));
    assert_eq!(features.get_string_elem(1), Some("printing"));
    assert_eq!(features.get_string_elem(2), Some("export"));
}

#[cfg(libconfig_1_4)]
#[test]
fn test_remove_elem() {
    let mut cfg = libconfig::Config::new();
    cfg.read_string("list = (1, 2, 3, 4);").unwrap();

    let orig_len = cfg.lookup("list").unwrap().len();

    let s = cfg.lookup("list").unwrap();
    let mut s_mut: libconfig::Setting<'_> = unsafe { std::mem::transmute(s) };
    s_mut.remove_elem(0).unwrap();
    drop(s_mut);

    let list = cfg.lookup("list").unwrap();
    assert_eq!(list.len(), orig_len - 1);
    assert_eq!(list.get_int_elem(0), Some(2));
}