use std::fs::File;
use std::io::{Read, Write};

pub fn write_to_file(filepath: &str, text: &str) -> Result<(), String> {
    // ファイル生成
    let mut file = File::create(filepath).map_err(|e| format!("ファイル生成エラー: {}", e))?;
    // ファイル書き込み
    file.write_all(text.as_bytes()).map_err(|e| format!("ファイル書き込みエラー: {}", e))?;
    Ok(())
}

pub fn read_from_file(filename: &str) -> Result<String, String> {
    // ファイルオープン
    let mut file = File::open(filename).map_err(|e| format!("ファイルオープンエラー: {}", e))?;
    // ファイルの内容をStringに書き込む
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|e| format!("ファイル書き込みエラー: {}", e))?;
    Ok(content)
}

// 動作確認テスト
#[cfg(test)]
mod tests {
    use super::*;           // 親モジュールのインポート
    use std::path::Path;    // パス操作のためにインポート

    #[test]
    fn test_file_operations() {
        let test_file_path = "test_data_shorthand.txt"; // ファイル名を変更して衝突を避ける
        let test_content = "こんにちは、Rustの世界へ\nこれは短縮系のテストデータです。";
        // ファイルへの書き込みテスト
        match write_to_file(test_file_path, test_content) {
            Ok(_) => println!("ファイルに正常に書き込みました。"),
            Err(e) => panic!("ファイル書き込みエラーが発生しました: {}", e),
        }
        // ファイルからの読み込みテスト
        match read_from_file(test_file_path) {
            Ok(read_content) => {
                println!("ファイルから読み込んだ内容:\n{}", read_content);
                assert_eq!(read_content, test_content);
            },
            Err(e) => panic!("ファイル読み込みエラーが発生しました: {}", e),
        }
        // 存在しないファイルの読み込みテスト（不具合想定）
        match read_from_file("non_existent_file_shorthand.txt") {
            Ok(_) => panic!("存在しないファイルの読み込みが成功してしまいました。（テスト失敗）"),
            Err(e) => println!("存在しないファイルの読み込みエラー（期待通り）: {}", e),
        }
        // テスト後にファイルのクリーンアップ
        if Path::new(test_file_path).exists() {
            if let Err(e) = std::fs::remove_file(test_file_path) {
                eprintln!("テストファイルの削除に失敗しました: {}", e);
            }
        }
    }
}
