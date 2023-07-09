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
    Color(u8, u8, u8),
    Image(String),
}

impl Setting{
    // #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        Setting {
            mode: Mode::Light,
            background: Background::Transparent
        }
    }
}