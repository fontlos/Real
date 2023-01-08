use anyhow::Result;
use nom::{
    bytes::complete::{tag, take_until},
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use sled::Batch;
use walkdir::{DirEntry, WalkDir};

use std::path::Path;

const ID: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
];

#[cfg(not(target_arch = "wasm32"))]
pub fn read_local_music_dir(database: &sled::Db,title: Option<&str>, path: &str) -> Result<()> {
    // let database = sled::open("config")?;

    let tree = if let Some(t) = title {
        format!("local-{}",t)
    } else {
        // 拿到当前文件夹名字
        format!("local-{}",Path::new(path).file_name().unwrap().to_str().unwrap())
    };
    let name = &tree;

    // 用于储存 local 有多少个列表
    let local = database.open_tree("local")?;
    let tree_name = name.as_bytes();
    local.insert(tree_name, tree_name)?;

    // TODO 用同样的方法给作者，专辑，时间顺序建立索引
    // 用 文件名称 建立当前文件夹文件索引
    let local_name_index = database.open_tree(tree_name)?;

    // 将当前文件全部存入 music 类树下
    let music_name = database.open_tree("music-name")?;
    let music_artist = database.open_tree("music-artist")?;
    let music_album = database.open_tree("music_album")?;
    let music_source = database.open_tree("music-source")?;
    let music_url = database.open_tree("music-url")?;

    let mut local_tree_handler = Batch::default();
    let mut music_name_handler = Batch::default();
    let mut music_artist_handler = Batch::default();
    let mut music_album_handler = Batch::default();
    let mut music_source_handler = Batch::default();
    let mut music_url_handler = Batch::default();

    for item in WalkDir::new(path)
        .into_iter()
        // 跳过无法打开的文件
        .filter_map(|entry| entry.ok())
        // 过滤文件夹
        .filter(|entry| entry.path().is_file() && is_music(entry))
    {
        let path = item.path();
        let id = format!("{}-{}",name,path.file_name().unwrap().to_str().unwrap());
        let (name, artist, album) =
            parse_music_file_name(path.file_stem().unwrap().to_str().unwrap());

        local_tree_handler.insert(name, id.as_bytes());
        music_name_handler.insert(id.as_bytes(), name);
        music_artist_handler.insert(id.as_bytes(), artist);
        music_album_handler.insert(id.as_bytes(), album);
        music_source_handler.insert(id.as_bytes(),b"local");
        // 拿到文件路径
        music_url_handler.insert(id.as_bytes(), path.to_str().unwrap());
    }

    local_name_index.apply_batch(local_tree_handler)?;
    music_name.apply_batch(music_name_handler)?;
    music_artist.apply_batch(music_artist_handler)?;
    music_album.apply_batch(music_album_handler)?;
    music_source.apply_batch(music_source_handler)?;
    music_url.apply_batch(music_url_handler)?;

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn is_music(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            s.ends_with(".mp3")
                || s.ends_with(".flac")
                || s.ends_with(".mp4")
                || s.ends_with(".m4a")
                || s.ends_with(".acc")
                || s.ends_with(".wav")
                || s.ends_with(".ogg")
                || s.ends_with(".MP3")
                || s.ends_with(".FLAC")
                || s.ends_with(".MP4")
                || s.ends_with(".M4A")
                || s.ends_with(".ACC")
                || s.ends_with(".WAV")
                || s.ends_with(".OGG")
        })
        .unwrap_or(false)
}

fn parse_music_file_name_handler(input: &str) -> IResult<&str, Option<&str>> {
    map(tuple((opt(take_until(" - ")), opt(tag(" - ")))), |p| p.0)(input)
}

fn parse_music_file_name(input: &str) -> (&str, &str, &str) {
    let (remain1, name) = parse_music_file_name_handler(input).unwrap();
    let (remain2, artist) = parse_music_file_name_handler(remain1).unwrap();
    if name.is_none() {
        (input, "未知艺术家", "未知专辑")
    } else if artist.is_none() {
        (name.unwrap(), remain1, "未知专辑")
    } else {
        (name.unwrap(), artist.unwrap(), remain2)
    }
}

#[test]
fn init_db(){
    let path = "D:/Library/Music";
    let database = sled::open("config").unwrap();
    let mut default_data = Batch::default();
    default_data.insert("mode", "light");
    database.apply_batch(default_data).unwrap();

    #[cfg(target_os = "windows")]
    read_local_music_dir(&database,Some("default"), path).unwrap();
}

#[test]
fn test_print_db() {
    let path = "D:/Library/Music";
    let title = "default";

    let database = sled::open("config").unwrap();

    read_local_music_dir(&database,Some(title), path).unwrap();

    let data = sled::open("config").unwrap();

    let default = data.open_tree("local-default").unwrap();
    for key in default.iter().keys(){
        let value = &default.get(key.unwrap()).unwrap().unwrap();
        let refs = value.as_ref();
        let strs = std::str::from_utf8(refs).unwrap();
        println!("value: {}", strs)
    }
}