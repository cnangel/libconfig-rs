use libconfig::{ConfigOptions, ErrorType, SettingFormat, SettingType};

#[test]
fn test_setting_type_values() {
    assert_eq!(SettingType::None as i32, 0);
    assert_eq!(SettingType::Group as i32, 1);
    assert_eq!(SettingType::Int as i32, 2);
    assert_eq!(SettingType::Int64 as i32, 3);
    assert_eq!(SettingType::Float as i32, 4);
    assert_eq!(SettingType::String as i32, 5);
    assert_eq!(SettingType::Bool as i32, 6);
    assert_eq!(SettingType::Array as i32, 7);
    assert_eq!(SettingType::List as i32, 8);
}

#[test]
fn test_setting_type_from_raw() {
    assert_eq!(SettingType::from_raw(0), Some(SettingType::None));
    assert_eq!(SettingType::from_raw(1), Some(SettingType::Group));
    assert_eq!(SettingType::from_raw(8), Some(SettingType::List));
    assert_eq!(SettingType::from_raw(99), None);
}

#[test]
fn test_setting_format_values() {
    assert_eq!(SettingFormat::Default as u16, 0);
    assert_eq!(SettingFormat::Hex as u16, 1);
    #[cfg(libconfig_1_8)]
    {
        assert_eq!(SettingFormat::Bin as u16, 2);
        assert_eq!(SettingFormat::Oct as u16, 3);
    }
}

#[test]
fn test_setting_format_from_raw() {
    assert_eq!(SettingFormat::from_raw(0), Some(SettingFormat::Default));
    assert_eq!(SettingFormat::from_raw(1), Some(SettingFormat::Hex));
    assert_eq!(SettingFormat::from_raw(99), None);
}

#[test]
fn test_config_options_bitflags() {
    assert_eq!(ConfigOptions::AUTOCONVERT.bits(), 0x01);
    assert_eq!(ConfigOptions::FSYNC.bits(), 0x40);

    let combined = ConfigOptions::AUTOCONVERT | ConfigOptions::FSYNC;
    assert_eq!(combined.bits(), 0x01 | 0x40);
}

#[test]
fn test_error_type_values() {
    assert_eq!(ErrorType::None as i32, 0);
    assert_eq!(ErrorType::FileIo as i32, 1);
    assert_eq!(ErrorType::Parse as i32, 2);
}

#[test]
fn test_error_type_from_raw() {
    assert_eq!(ErrorType::from_raw(0), Some(ErrorType::None));
    assert_eq!(ErrorType::from_raw(1), Some(ErrorType::FileIo));
    assert_eq!(ErrorType::from_raw(2), Some(ErrorType::Parse));
    assert_eq!(ErrorType::from_raw(99), None);
}

#[test]
fn test_version_string() {
    let v = libconfig::version();
    assert!(!v.is_empty());
}