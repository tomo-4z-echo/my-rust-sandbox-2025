// lib.rsで定義したモジュールをインポート
use my_rust_sandbox_2025::io_utils::my_file_control3::{write_to_file, read_from_file};
// use my_rust_sandbox_2025::calc::division::{division};
use my_rust_sandbox_2025::guess_game::guess_game2::{guess};
use my_rust_sandbox_2025::calc::fizzbuzz::{fizzbuzz, vowel_and_consonant};
// 標準クレート
use std::io;

// 数値を2倍にする関数
fn double_value(num: &mut i32) {
    // 参照先の値を変更するためにデリファレンス(*)が必要
    *num *= 2;
}

// ベクター（動的配列）の要素を可変参照を使って変更する例
fn add_prefix_to_strings(strings: &mut [String], prefix: &str) {
    for s in strings.iter_mut() {
        s.insert_str(0, prefix);
    }
}

// 構造体のフィールドを可変参照を使って変更する
struct User {
    name: String,
    age: u32,
    active: bool,
}
fn deactivate_user(user: &mut User) {
    user.active = false;
} 

fn main() {

    // // my_stringモジュール ---
    // // my_stringモジュール内のMyPoorMansStringを使う
    // let mut my_string = my_string::MyPoorMansString::new("Hello");
    // println!("Initial: {}", my_string.as_str());
    // my_string.push_str(", World!");
    // println!("After Push: {}", my_string.as_str());
    // // my_stringがスコープを抜けると、dropメソッドが呼ばれてメモリ解放される

    // my_file_controlモジュール ---
    let filename = "my_data.txt";
    // let content_to_write = "Hello!, Rust File I/O!";
    let mut content_to_write = String::new();
    // io::stdin()による書き込み受付
    io::stdin().read_line(&mut content_to_write).expect("Failed to read line");
    // ファイル書き込みテスト
    println!("--- Read File ---");
    match write_to_file(filename, &content_to_write) {
        Ok(()) => println!("ファイル '{filename}' に正常に書き込みました。"),
        Err(e) => eprintln!("ファイル書き込みエラー: {e}"),   // エラーはstderrに出力する。
    }
    // ファイル読み込みテスト
    println!("--- Write File ---");
    match read_from_file(filename) {
        Ok(content) => println!("ファイル '{filename}' から読み込んだ内容: '{content}'"),
        Err(e) => eprintln!("ファイル削除エラー: {e}"),
    }

    // 2倍にする
    let mut x = 10;
    println!("変更前: {x}");
    double_value(&mut x);
    println!("変更後: {x}");

    // 配列の中にある文字列の先頭に文字を追加する
    let mut my_strings = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];
    println!("変更前: {my_strings:?}");
    add_prefix_to_strings(&mut my_strings, "super_");
    println!("変更後: {my_strings:?}");

    // User構造体のアクティブ状態を変更する
    let mut user1 = User {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };
    println!("変更前: {} はアクティブ: {} / 年齢: {}", user1.name, user1.active, user1.age);
    deactivate_user(&mut user1);
    println!("変更後: {} はアクティブ: {} / 年齢: {}", user1.name, user1.active, user1.age);

    // 数字当てゲーム（guess_gameモジュールから導入）
    guess();

    // Fizzbuzz問題
    fizzbuzz();

    // 母音子音表示
    vowel_and_consonant();
}
