use anyhow::Result;
pub mod search_ncm;

pub enum SearchEngine {
    NCM,
}

pub struct SearchConfig<'a> {
    pub key: &'a str,
    pub limit: u16,
    pub offset: u16,
    pub search_type: u16,
}

struct Song<'a> {
    id: u32,
    title: &'a str,
    artists: Vec<Artist<'a>>,
    album: Album<'a>,
    source: Vec<Source>,
}

struct Artist<'a> {
    pub id: u32,
    pub name: &'a str,
}

/// ## 专辑
/// 封面用作歌曲封面
struct Album<'a> {
    id: u32,
    title: &'a str,
    img: &'a str,
}

enum Source {
    /// 高品质音频320k
    High { br: u32, size: u32 },
    /// 中品质音频192k
    Middle { br: u32, size: u32 },
    /// 低品质音频128k
    Low { br: u32, size: u32 },
    /// 超高品质，即无损品质
    Super { br: u32, size: u32 },
    /// 高解析品质
    HighResolution { br: u32, size: u32 },
}

use search_ncm::NCMSong;
use search_ncm::SearchResult;
impl SearchEngine {
    pub async fn search<'a>(self, sc: SearchConfig<'a>) -> Result<Vec<NCMSong>> {
        // 升级为 https api
        let url = match self {
            SearchEngine::NCM => format!("https://pl-fe.cn/cloud-music-api/cloudsearch?keywords={}&&limit={}&&offset={}&&type={}", sc.key, sc.limit, sc.offset, sc.search_type),
        };
        Ok(reqwest::get(url)
            .await?
            .json::<SearchResult>()
            .await?
            .result())
    }
}


