#[test]
fn test_search() {
    use crate::data::search::search_ncm::SearchConfig;
    let sc = SearchConfig {
        key: String::from("独角"),
        offset: 0,
        limit: 1,
        search_type: 1,
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(sc.search());
    let song = result.unwrap();
    println!("{:?}", song);
}

/*
[Song {
id: 1934213146, name: "独角",
ar: [Artist { id: 36288729, name: "UnicornPhantom" }],
al: Album { id: 142883110, name: "独角",
picUrl: "http://p3.music.126.net/1HOpmf61G-QHQmm5xIv9rg==/109951167230524234.jpg" },
h: Some(Quality { br: 320000, size: 7266264 }),
m: Some(Quality { br: 192000, size: 4359776 }),
l: Some(Quality { br: 128000, size: 2906532 }),
sq: Some(Quality { br: 1489769, size: 33816085 }),
hr: None }]
 */


#[test]
fn test_new_search() {
    use crate::data::search::{SearchConfig, SearchEngine};
    let sc = SearchConfig {
        key: "独角",
        offset: 0,
        limit: 1,
        search_type: 1,
    };

    let ss = SearchEngine::NCM.search(sc);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(ss);
    match result {
        Ok(song) => println!("{:?}", song),
        Err(err) => println!("{}", err),
    };
}