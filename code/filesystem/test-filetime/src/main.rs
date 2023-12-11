//!
//! ファイルのタイムスタンプを書き換える
//!

/// ファイルタイムを書き換えます。
fn set_filetime(
    path: &str,
    time: &std::time::SystemTime,
) -> Result<(), Box<dyn std::error::Error>> {
    let mtime: filetime::FileTime = filetime::FileTime::from_system_time(*time);
    filetime::set_file_mtime(path, mtime)?;
    return Ok(());
}

/// システムタイムをフォーマットします。
fn format_systemtime(time: &std::time::SystemTime) -> String {
    let datetime: chrono::DateTime<chrono::Local> = time.clone().into();
    let datetime = datetime.format("%Y-%m-%d %H:%M:%S%.3f");
    let datetime = datetime.to_string();
    return datetime;
}

/// ファイルタイムを表示します。
fn show_filetime(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = std::fs::metadata(path)?;
    let mtime = metadata.modified()?;
    let mtime = format_systemtime(&mtime);
    println!("{}: {}", path, mtime);
    return Ok(());
}

/// ファイルタイムを取得します。
fn get_file_mtime(path: &str) -> Result<std::time::SystemTime, Box<dyn std::error::Error>> {
    let metadata = std::fs::metadata(path)?;
    let mtime = metadata.modified()?;
    return Ok(mtime);
}

/// エントリーポイント
fn main() {
    // 空のファイルを作成します。
    std::fs::File::create("hello.txt").unwrap();

    // ファイルタイムを診断(1)
    show_filetime("hello.txt").unwrap();

    // 更新日時
    let mtime = get_file_mtime("hello.txt").unwrap();

    // 24時間戻す
    let mtime = mtime - std::time::Duration::from_secs(60 * 60 * 24);

    // ファイルタイムを設定
    set_filetime("hello.txt", &mtime).unwrap();

    // ファイルタイムの診断(2)
    show_filetime("hello.txt").unwrap();
}
