// 必要なモジュールをインポートします
use std::fs::File;  // ファイル操作
use std::io::Write; // ファイル書き込みのため
use std::io::Read;  // ファイル読み込みのため

// ファイル書き込み
pub fn write_to_file(filepath: &str, text: &str) -> Result<(), String> {
    // ファイルを開く（あるいは作成する）
    // File::create(filepath)はResult<File, io::Error>を返す
    // エラーハンドリングはmatchで行う
    let mut file = match File::create(filepath) {
        Ok(f) => f, // ファイルが正常に開けたらファイルインスタンスを受け取る
        Err(e) => return Err(format!("ファイル作成エラー: {}", e)), // エラーの場合は即座にErrを返す
    };

    // ファイルにテキストを書き込む
    match file.write_all(text.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("ファイル書き込みエラー: {}", e)),    // エラーの場合は即座にErrを返す
    }
}

pub fn read_from_file(filepath: &str) -> Result<String, String> {
    // ファイルを開く
    let mut file = match File::open(filepath) {
        Ok(f) => f,
        Err(e) => return Err(format!("ファイル読み込みエラー: {}", e)),
    };
    // ファイルの内容をStringに読み込む
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(format!("ファイル読み込みエラー: {}", e)),
    }
}