use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting{
    mode: Mode,
    background: Background
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mode{
    Light,
    Dark,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Background{
    Transparent,
    Image(String),
    Video(String),
}

impl Setting{
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        Setting {
            mode: Mode::Light,
            background: Background::Transparent
        }
    }
    #[cfg(target_os = "macos")]
    pub fn new() -> Self {
        Setting {
            mode: Mode::Light,
            background: Background::Video(String::from("./assets/video/background.mp4"))
        }
    }
    #[cfg(any(target_os = "linux",target_arch = "wasm32"))]
    pub fn new() -> Self {
        Setting {
            mode: Mode::Light,
            background: Background::Video(String::from("./assets/img/background.jpg"))
        }
    }
}