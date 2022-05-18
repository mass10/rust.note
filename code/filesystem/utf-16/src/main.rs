/// ユーティリティトレイト
trait AppStringUtil {
    /// この文字列が BOM で開始しているかどうかを返す
    fn has_bom(&self) -> bool;
}

impl AppStringUtil for &str {
    fn has_bom(&self) -> bool {
        if self.len() < 3 {
            return false;
        }

        if self.as_bytes()[0] == 0xEF && self.as_bytes()[1] == 0xBB && self.as_bytes()[2] == 0xBF {
            return true;
        }

        return false;
    }
}

/// ファイル全体の内容を String で返します。
#[allow(unused)]
fn read_text_file_string(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path).unwrap();
	let mut content = String::new();
	file.read_to_string(&mut content)?;

    return Ok(content);
}

/// ファイル全体の内容を Vec<u8> で返します。
fn read_text_file_vec_u8(path: &str) -> std::result::Result<Vec<u8>, Box<dyn std::error::Error>> {
    use std::io::Read;

    let mut file = std::fs::File::open(path)?;
    let mut content: Vec<u8> = vec![];
    file.read_to_end(&mut content)?;

    return Ok(content);
}

/// UTF-8 の BOM を取り除きます。
fn remove_bom(content: &str) -> String {
    if !content.has_bom() {
        return content.to_string();
    }
    let (_, content) = content.split_at(3);

    return content.to_string();
}

/// UTF-16 テキストファイルの全体を読み込みます。
fn read_text_file_utf16(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let buffer: Vec<u8> = read_text_file_vec_u8(path)?;
    let (_prefix, shorts, _suffix) = unsafe { buffer.align_to::<u16>() };
    let result = String::from_utf16(shorts)?;
    let result = remove_bom(&result);

    return Ok(result);
}

fn main() {
    let result = read_text_file_utf16("C:\\Users\\mass10\\Desktop\\test.txt");
    if result.is_err() {
        println!("[ERROR] {}", result.unwrap_err());
        return;
    }

    println!("{}", result.unwrap());
}
