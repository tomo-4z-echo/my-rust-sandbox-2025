// lib.rsで定義したモジュールをインポート
use my_rust_sandbox_2025::io_utils::my_file_control3::{write_to_file, read_from_file};
// 標準クレート
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// 数値を2倍にする関数
fn double_value(num: &mut i32) {
    // 参照先の値を変更するためにデリファレンス(*)が必要
    *num *= 2;
}

// ベクター（動的配列）の要素を可変参照を使って変更する例
fn add_prefix_to_strings(strings: &mut Vec<String>, prefix: &str) {
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
        Ok(()) => println!("ファイル '{}' に正常に書き込みました。", filename),
        Err(e) => eprintln!("ファイル書き込みエラー: {}", e),   // エラーはstderrに出力する。
    }
    // ファイル読み込みテスト
    println!("--- Write File ---");
    match read_from_file(filename) {
        Ok(content) => println!("ファイル '{}' から読み込んだ内容: '{}'", filename, content),
        Err(e) => eprintln!("ファイル削除エラー: {}", e),
    }

    // 2倍にする
    let mut x = 10;
    println!("変更前: {}", x);
    double_value(&mut x);
    println!("変更後: {}", x);

    // 配列の中にある文字列の先頭に文字を追加する
    let mut my_strings = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];
    println!("変更前: {:?}", my_strings);
    add_prefix_to_strings(&mut my_strings, "super_");
    println!("変更後: {:?}", my_strings);

    // User構造体のアクティブ状態を変更する
    let mut user1 = User {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };
    println!("変更前: {} はアクティブ: {} / 年齢: {}", user1.name, user1.active, user1.age);
    deactivate_user(&mut user1);
    println!("変更後: {} はアクティブ: {} / 年齢: {}", user1.name, user1.active, user1.age);

    // message
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Your guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
