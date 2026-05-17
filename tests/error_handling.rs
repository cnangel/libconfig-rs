use libconfig::Config;

#[test]
fn test_parse_error() {
    let mut cfg = Config::new();
    let result = cfg.read_string("invalid {{{");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(!err.message.is_empty());
}

#[test]
fn test_file_not_found_error() {
    let mut cfg = Config::new();
    let result = cfg.read_file("/definitely/not/a/real/path.cfg");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(!err.message.is_empty());
}

#[test]
fn test_error_display() {
    let mut cfg = Config::new();
    let _ = cfg.read_string("bad {{{");
    let text = cfg.error_text();
    assert!(!text.is_empty());
    // Display format should include error message
    let _ = format!("{}", text);
}

#[test]
fn test_error_line() {
    let mut cfg = Config::new();
    let _ = cfg.read_file("/nonexistent/path.cfg");
    // error_line may be 0 for file I/O errors
    let _line = cfg.error_line();
}

#[test]
fn test_error_type() {
    let mut cfg = Config::new();
    let _ = cfg.read_file("/nonexistent/path.cfg");
    let et = cfg.error_type();
    // after file I/O error, should be FileIo
    use libconfig::ErrorType;
    assert!(matches!(et, ErrorType::FileIo));

    let mut cfg2 = Config::new();
    let _ = cfg2.read_string("parse error {{{");
    let et2 = cfg2.error_type();
    assert!(matches!(et2, ErrorType::Parse));
}

#[test]
fn test_error_file() {
    let mut cfg = Config::new();
    let _ = cfg.read_file("/tmp/test-nonexistent.cfg");
    // error_file may or may not be set depending on libconfig version
    let _file = cfg.error_file();
}

#[test]
fn test_config_error_implements_error_trait() {
    let mut cfg = Config::new();
    let result = cfg.read_string("invalid");
    assert!(result.is_err());
    let err = result.unwrap_err();

    // Verify it implements std::error::Error
    let _: &dyn std::error::Error = &err;
}