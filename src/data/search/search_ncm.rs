use serde::Deserialize;
/// ## 搜索配置信息
/// 1. key 关键字
/// 2. limit 单次搜索数量
/// 3. offset 偏移量相当于搜索翻页
/// 4. search_type 搜索类型
/// - 1: 单曲,
/// - 100: 歌手,
/// - 1000: 歌单,
/// - 1006: 歌词,
/// - 1018:综合,
pub struct SearchConfig {
    pub key: String,
    pub limit: u16,
    pub offset: u16,
    pub search_type: u16,
}

impl SearchConfig {
    pub async fn search(self) -> Result<Vec<NCMSong>, reqwest::Error> {
        let (key, limit, offset, search_type) =
            (self.key, self.limit, self.offset, self.search_type);
        // 升级为 https api
        let url = format!("https://pl-fe.cn/cloud-music-api/cloudsearch?keywords={key}&&limit={limit}&&offset={offset}&&type={search_type}");
        let response = reqwest::get(url)
            .await
            .unwrap()
            .json::<SearchResult>()
            .await;
        Ok(response?.result.songs)
    }
}

#[derive(Deserialize, Clone)]
pub struct SearchResult {
    pub result: Songs,
}

impl SearchResult {
    /// 返回歌曲列表
    pub fn result(self) -> Vec<NCMSong> {
        self.result.songs
    }
}

#[derive(Deserialize, Clone)]
pub struct Songs {
    pub songs: Vec<NCMSong>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NCMSong {
    id: u32,
    name: String,
    ar: Vec<NCMArtist>,
    al: NCMAlbum,
    /// high 高品质音频320k
    h: Option<Quality>,
    /// middle 中品质音频192k
    m: Option<Quality>,
    /// low 低品质音频128k
    l: Option<Quality>,
    /// super quality 超高品质，即无损品质
    sq: Option<Quality>,
    /// high resolution高解析品质
    hr: Option<Quality>,
}

impl NCMSong {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn artist(&self) -> Vec<NCMArtist> {
        self.ar.clone()
    }
    pub fn album_id(&self) -> u32 {
        self.al.id
    }
    pub fn album_name(&self) -> String {
        self.name.clone()
    }
    pub fn img(&self) -> String {
        self.al.picUrl.clone()
    }
}

// impl<'a> From<NCMSong> for super::Song<'a> {
//     fn from(s: NCMSong) -> Self {
//         let artists:Vec<super::Artist> = s.ar.iter().map(|a|{
//             super::Artist{
//                 id:a.id,
//                 name:&a.name
//             }
//         }).collect();
//         let source = Vec::<super::Source>::new();
//         super::Song{
//             title:&s.name,
//             id:s.id,
//             artists,
//             album:super::Album{
//                 id:s.al.id,
//                 title:&s.al.name,
//                 img:&s.al.picUrl
//             },
//             source
//         }
//     }
// }

/// ##作者列表
#[derive(Deserialize, Clone, Debug)]
pub struct NCMArtist {
    pub id: u32,
    pub name: String,
}

/// ## 专辑
/// 封面用作歌曲封面
#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
struct NCMAlbum {
    id: u32,
    name: String,
    picUrl: String,
}

#[derive(Deserialize, Clone, Debug)]
struct Quality {
    /// 码率
    br: u32,
    /// 大小
    size: u32,
}