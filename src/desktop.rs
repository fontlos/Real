use dioxus::prelude::*;
use dioxus::desktop::{tao::dpi::LogicalSize, wry::application::window::Icon};
use anyhow::Result;
use sled::Batch;

use std::fs;
use std::path::Path;
use std::io::Cursor;

use crate::data::config::config_db::read_local_music_dir;
use crate::components;
use crate::page;
use crate::APP;

/// Desktop 端配置
pub fn start() -> Result<()> {
    // 初始化配置
    // 检测一些文件夹是否存在
    if !Path::new("./config").exists() {
        let database = sled::open("config").unwrap();
        let mut default_data = Batch::default();
        default_data.insert("mode", "light");
        database.apply_batch(default_data)?;
        // 初始化后卸载
        drop(database);
    };
    if !Path::new("./download").exists() {
        fs::create_dir("./download").unwrap();
    }

    let database = sled::open("config").unwrap();

    #[cfg(target_os = "windows")]
    read_local_music_dir(&database ,Some("default"), &default_music_lib())?;
    read_local_music_dir(&database ,Some("download"),"./download")?;

    let logo = include_bytes!("../icons/icon.png");
    let mut reader = image::io::Reader::new(Cursor::new(logo));
    reader.set_format(image::ImageFormat::Png);
    let icon = reader.decode().unwrap();
    let icon = Icon::from_rgba(icon.as_bytes().to_owned(), 512, 512).unwrap();

    dioxus::desktop::launch_with_props(app, (),|config| {
        config
            .with_window(|window| {
                window
                    .with_title("Solitude")
                    .with_window_icon(Some(icon))
                    .with_decorations(false)
                    .with_transparent(true)
                    .with_resizable(true)
                    .with_inner_size(LogicalSize::new(1240.0, 720.0))
                    .with_min_inner_size(LogicalSize::new(500.0, 500.0))
            })
            .with_custom_index(
                r#"
            <!DOCTYPE html>
            <html lang="zh-CN">
                <head>
                    <meta charset="utf-8" />
                    <link rel="stylesheet" href="assets/css/root.css" />
                    <link rel="stylesheet" href="assets/css/main.css" />
                </head>
                <body id="main"></body>
            </html>
        "#
                .into(),
            )
    });
    Ok(())
}

fn app(cx: Scope) -> Element {
    // 非 WASM 的 debug 模式下开启调试工具
    #[cfg(debug_assertions)]
    {
        let _window = dioxus::desktop::use_window(&cx);
        _window.devtool();
    }

    let app_mode = use_read(&cx, APP);

    rsx! (cx,
        div{
            class: "app {app_mode}",
            Router{
                components::header{}
                // Link 不能再 Router之外使用，nav 组件必须放入 Router 内
                components::nav{}
                Route{
                    to: "/",
                    page::home{}
                }
                Route{
                    to: "/ncm",
                    page::ncm{}
                }
                Route{
                    to: "/ncm/:id",
                    page::ncm{}
                }
                Route{
                    to: "/qm",
                    page::qm{}
                }
                Route{
                    to: "/qm/:id",
                    page::qm{}
                }
                Route{
                    to: "/playlist/:id",
                    page::playlist{}
                }
                Route{
                    to: "/local/:id",
                    page::local{}
                }
                Route{
                    to: "",
                    page::home{}
                }
                components::footer{}
            }
        }
    )
}

#[cfg(target_os = "windows")]
fn default_music_lib() -> String {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let sf = hkcu
        .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders")
        .unwrap();
    // let dt: String = sf.get_value("Desktop").unwrap();
    let music_lib: String = sf.get_value("My Music").unwrap();
    music_lib
}