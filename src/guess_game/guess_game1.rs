// The Bookにある[Guess Game]（数当てゲームのプログラム）について
// これはRustという言語ってこんな感じだよ？ということを提示しているテキストだと思います。
// 数当てゲームを作るにあたって必要な要素を以下に記述していきます。
// このゲームは、コンピュータ側が1~100までの数値の中からひとつを選び、ユーザー側が入力した値を比較して一致したら勝ちというゲームです。
// 1. コンピュータ側が1~100の内の数値をランダムに取得（u32型）
// 2. ユーザーが入力する値を格納する変数を用意します（String型）
// 3. 入出力装置の機能を使ってユーザーの入力を受け付けます。ただし受け付けるのは数値で1~100までに範囲です。(u32型)
// 4. 入出力に失敗した場合は再度入力を要求します。（expectを使わずにNoneを返すして再入力を促す）
// 5. コンピュータ側とユーザー側の数値を比較して正解したら終了。異なる場合は再度入力をさせます。

// 必要な機能のimport:
// 乱数を使用するために、randクレートを使用
// 入出力対応のために、std::ioを使用、かつバッファリング対応してIO操作の効率を上げるためにflushさせます。flushにはWriteトレイトが必要なので宣言します。
// 数値比較のために、std::cmp::Orderingを使用します。

use rand::Rng;
use std::io::{self, Write};
use std::cmp::Ordering;

fn get_guess() -> Option<u32> {
    println!("1~100までの数値を入力してください・・・");
    // 入力プロンプトの直後にフラッシュしてすぐに表示させるようにする
    match io::stdout().flush() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("出力のフラッシュに失敗しました: {e}");
            return None;
        }
    }
    // 格納用変数の宣言
    let mut guess = String::new();
    // 入力読み込み処理
    match io::stdin().read_line(&mut guess) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("入力の読み込みに失敗しました: {e}");
            return None;
        }
    }
    // trim()で空白を除去しparseで数値に変換する
    guess.trim().parse::<u32>().ok().and_then(|num| {
        if (1..=100).contains(&num) {
            Some(num)
        } else {
            eprintln!("入力された数値は1~100の範囲の外です");
            None
        }
    })
}

pub fn guess() {
    println!("--- 数字当てゲーム ---");
    // コンピューター側の数値
    let secret_number = rand::rng().random_range(1..=100);
    // 正解が出るまで繰り返す
    loop {
        let guess = match get_guess() {
            Some(num) => num,
            None => {
                println!("有効な値(1~100)を入力してください・・・");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("もっと大きな値です"),
            Ordering::Greater => println!("もっと小さな値です"),
            Ordering::Equal => {
                println!("大正解です!");
                break;
            }
        }
    }
}