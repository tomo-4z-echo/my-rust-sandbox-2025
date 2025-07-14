use std::fs::File;
use std::io::{Read, Write};

pub fn write_to_file(filepath: &str, text: &str) -> Result<(), String> {
    // ファイルを開く
    // let mut file = match File::create(filepath) {
    //     Ok(f) => f,
    //     Err(e) => return Err(format!("ファイル作成エラー: {}", e)),
    // };
    let mut file = File::create(filename).map_err(|e| format!("ファイル作成エラー: {}", e))?;
    // ファイルに書き込む
    // match file.write_all(text.as_bytes()) {
    //     Ok(_) => Ok(()),
    //     Err(e) => Err(format!("ファイル書き込みエラー: {}", e)),
    // }
    file.write_all(text.as_bytes()).map_err(|e|, format!("ファイル書き込みエラー: {}", e))?;

    Ok(())
}

pub fn read_from_file(filename: &str) -> Result<String, String> {
    // ファイルを開く
    let mut file = match File.open(filename) {
        Ok(f) => f,
        Err(e) => return Err(format!("ファイルオープンエラー: {}", e)),
    };
    // ファイルの内容をString型にして戻す --- String型contentにファイルの内容を書き込む
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(format!("ファイル読み込みエラー: {}", e)),
    }
}