use std::io::Write;
use tempfile::NamedTempFile;

/// A representative config string exercising all types.
pub fn sample_config_str() -> &'static str {
    r#"
version = "1.0";
application:
{
    window:
    {
        title = "MyApp";
        width = 800;
        height = 600;
        fullscreen = false;
        opacity = 0.95;
    };
    features = ["saving", "printing", "export"];
    misc = (1, "two", true, 3.14);
};
"#
}

/// Create a Config and read the sample config string into it.
pub fn new_config_with_data() -> libconfig::Config {
    let mut cfg = libconfig::Config::new();
    cfg.read_string(sample_config_str())
        .expect("sample config string should parse");
    cfg
}

/// Create a temporary config file with the given content, read it, return (Config, tempfile).
pub fn temp_config_file(content: &str) -> (libconfig::Config, NamedTempFile) {
    let mut file = NamedTempFile::new().expect("temp file");
    write!(file, "{}", content).expect("write to temp file");
    let mut cfg = libconfig::Config::new();
    cfg.read_file(file.path().to_str().unwrap())
        .expect("read temp config file");
    (cfg, file)
}