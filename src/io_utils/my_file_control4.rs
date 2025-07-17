use std::fs::File;
use std::io::{self, Read, Write};

// ファイルへの書き込み
pub fn write_to_file(filepath: &str, text: &str) -> Result<(), String> {
    // ファイル生成
    let mut file = File::create(filepath).map_err(|e| format!("ファイル生成エラー: {e}"))?;
    // ファイル書き込み
    file.write_all(text.as_bytes()).map_err(|e| format!("ファイル書き込みエラー: {e}"))?;
    Ok(())
}

// ファイルの読み込み
pub fn read_from_file(filepath: &str) -> Result<String, String> {
    // ファイルオープン
    let mut file = File::open(filepath).map_err(|e| format!("ファイルオープンエラー: {e}"))?;
    // ファイル内容を書き込む
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|e| format!("ファイル書き込みエラー: {e}"))?;
    // コンテントの返却
    Ok(content)
}

pub fn writing_and_reading_example() {
    // ファイル名指定
    let filename = "my_data.txt";
    // ファイルに書き込みための文字列領域を用意
    let mut content_to_write = String::new();
    // 標準入力からの書き込み
    println!("ファイルに登録したい文章を入力してください！");
    match io::stdin().read_line(&mut content_to_write) {
        Ok(_) => (),
        Err(e) => panic!("標準入力に失敗しました: {e}"),
    }
    // ファイル書き込み
    println!("--- Write File ---");
    match write_to_file(filename, &content_to_write) {
        Ok(_) => println!("ファイル: {filename}に正常に書き込みました"),
        Err(e) => panic!("ファイルの書き込みに失敗します: {e}"),
    }
    // ファイル読み込み
    println!("--- Read File ---");
    match read_from_file(filename) {
        Ok(text) => {
            println!("--------");
            println!("ファイル: {filename}");
            println!("ファイル内容: {text}");
            println!("--------");
        },
        Err(e) => panic!("ファイルの読み込みに失敗します: {e}"),
    }
}

// 動作確認テスト
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_file_operations() {
        let test_file_path = "test_data_shorthand.txt";
        let test_content = "こんにちはRustの世界へ！\nこれは短縮系のテストデータです。";
        // ファイル書き込みテスト
        match write_to_file(test_file_path, test_content) {
            Ok(_) => println!("ファイルに正常に書き込みました。"),
            Err(e) => eprintln!("ファイル書き込みエラーが発生しました: {e}"),
        }
        // ファイル読み込みテスト
        match read_from_file(test_file_path) {
            Ok(read_content) => {
                println!("読み込み内容: {read_content}");
                assert_eq!(read_content, test_content);
            },
            Err(e) => panic!("ファイル読み込みエラーが発生しました: {e}"),
        }
        // 存在しないファイルの読み込みテスト（不具合想定テスト）
        match read_from_file("non_existent_file_shorthand.txt") {
            Ok(_) => panic!("存在しないファイルの読み込みが成功してしまいました！"),
            Err(e) => println!("存在しないファイルの読み込みエラー: {e}"),
        }
        // テスト後にファイルのクリーンアップ
        if Path::new(test_file_path).exists() {
            // if let構文 --- ファイルを削除できていればOkが返るのでそのまま終了。
            // なにかしらの失敗があってErrが返った場合に、Err型のeをバインドしてブロック内の処理を実行する。
            // The Book 第18章 --- letは変数の宣言キーワードではなく、let pattern = expression である。マッチしたものをbindする
            if let Err(e) = std::fs::remove_file(test_file_path) {
                eprintln!("テストファイルの削除に失敗しました: {e}");
            }
        }
    }
}