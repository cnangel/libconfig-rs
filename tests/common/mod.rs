#![allow(dead_code)]

use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct TempFile {
    path: PathBuf,
    inner: std::fs::File,
}

impl TempFile {
    pub fn new() -> std::io::Result<Self> {
        let dir = std::env::temp_dir();
        let count = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let name = format!("libconfig-rs-{}-{}.cfg", std::process::id(), count);
        let path = dir.join(name);
        let inner = std::fs::File::create(&path)?;
        Ok(Self { path, inner })
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl std::io::Write for TempFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

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

/// Create a Config and read the sample config string via temp file (works on all versions).
pub fn new_config_with_data() -> libconfig::Config {
    let (cfg, _file) = temp_config_file(sample_config_str());
    cfg
}

/// Create a temporary config file with the given content, read it, return (Config, TempFile).
pub fn temp_config_file(content: &str) -> (libconfig::Config, TempFile) {
    let mut file = TempFile::new().expect("temp file");
    write!(file, "{}", content).expect("write to temp file");
    let mut cfg = libconfig::Config::new();
    cfg.read_file(file.path().to_str().unwrap())
        .expect("read temp config file");
    (cfg, file)
}