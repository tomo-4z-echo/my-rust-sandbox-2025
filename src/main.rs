// lib.rsで定義したモジュールをインポート
use my_rust_sandbox_2025::io_utils::my_file_control4::writing_and_reading_example;
// use my_rust_sandbox_2025::calc::division::{division};
use my_rust_sandbox_2025::guess_game::guess_game3::{guess};
use my_rust_sandbox_2025::calc::fizzbuzz::{fizzbuzz, vowel_and_consonant};
use my_rust_sandbox_2025::calc::calc_lib::{calc_example};
use my_rust_sandbox_2025::structure::rectangle::{Rectangle};

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
    writing_and_reading_example();

    // 入力数値を2倍にする
    calc_example();

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

    // 面積計算
    let rect1 = Rectangle {
        width: 10,
        height: 5,
    };
    println!(
        "The area of the rectangle is {} square pixcels.", rect1.area()
    );

}
