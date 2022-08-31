struct Configuration {}

impl Configuration {
    fn new() -> Configuration {
        return Configuration {};
    }
}

/// ファイル走査
///
/// # Arguments
/// * `e` パス
/// * `handler` ファイルハンドラー
pub fn search(
    conf: &Configuration,
    path: &std::path::Path,
    handler: &mut dyn FnMut(
        &std::path::Path,
    ) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>,
) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    if !path.exists() {
        println!("[TRACE] invalid path {}", path.to_str().unwrap());
        return Ok(());
    }

    if path.is_dir() {
        let name = retrieve_name(path).unwrap();

        // // 名前のフィルタリング
        // for pat in &conf.exclude_dirs {
        //     // TODO: ちゃんとする
        //     if &name == pat {
        //         return Ok(());
        //     }
        // }

        let it = std::fs::read_dir(path)?;
        for e in it {
            let entry = e.unwrap();
            let entry_path = entry.path();
            search(conf, &entry_path, handler)?;
        }
        return Ok(());
    } else if path.is_file() {
        let name = retrieve_name(path).unwrap();

        // 名前のフィルタリング
        // for pat in &conf.exclude_files {
        //     // TODO: ちゃんとする
        //     if name.contains(pat) {
        //         return Ok(());
        //     }
        // }

        return handler(path);
    } else {
        println!("[WARN] 不明なファイルシステム {:?}", path);
    }
    return Ok(());
}

fn main() {
    let conf = Configuration::new();

    let result = search(&conf, "C:\\");
    if result.is_err() {
        let err = result.err().unwrap();
        println!("ERROR: {}", err);
        return;
    }
}
