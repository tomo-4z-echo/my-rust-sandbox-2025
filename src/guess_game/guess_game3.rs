// コード理解のための推測ゲーム
// テストを考えてコードを組む（標準入出力とロジックを分離すること）
use rand::Rng;
use std::io::{self, Write};
use std::cmp::Ordering;

// 引数に渡された文字列をOption<u32>型にして返す
fn parse_and_validate_guess(input: &str) -> Option<u32> {
    input.trim().parse::<u32>().ok().and_then(|num| {
        if (1..=100).contains(&num) {
            Some(num)
        } else {
            None
        }
    })
}

// 入出力対応
fn get_guess() -> Option<u32> {
    print!("Input your guess number (1 ~ 100): ");
    let mut guess = String::new();
    // バッファのフラッシュ
    match io::stdout().flush() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("バッファのフラッシュに失敗しました: {e}");
            return None;
        }
    }
    // 入力待機
    match io::stdin().read_line(&mut guess) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("入力処理に失敗しました: {e}");
            return None;
        }
    }
    // 文字列 -> u32型変換ロジック
    parse_and_validate_guess(&guess)
}

// 実行
pub fn guess() {
    // title
    println!("--- 数字当てゲーム ---");
    // comp selected number
    let secret_number = rand::rng().random_range(1..=100) as u32;
    // debug
    println!("COMP NUMBER: {secret_number}");
    // loop - 正解の場合のみ終了
    loop {
        let guess = match get_guess() {
            Some(num) => num,
            None => {
                println!("1 ~ 100までの数値を選択してください・・・");
                continue;
            }
        };
        // 比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("正解は、推測値: {guess}よりも大きい数です。"),
            Ordering::Greater => println!("正解は、推測値: {guess}よりも小さい数です。"),
            Ordering::Equal => {
                println!("大正解！");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_validate_guess_valid() {
        // 1 ~ 100の数値範囲内の場合にSome(n)を返しているかどうか確認する
        assert_eq!(parse_and_validate_guess("50"), Some(50));
        assert_eq!(parse_and_validate_guess("100"), Some(100));
        assert_eq!(parse_and_validate_guess("1"), Some(1));
        assert_eq!(parse_and_validate_guess("75"), Some(75));
    }

    #[test]
    fn test_parse_and_validate_guess_invalid() {
        // 妥当な値が入力されなかった場合を確認する
        assert_eq!(parse_and_validate_guess("0"), None);
        assert_eq!(parse_and_validate_guess("1000"), None);
        assert_eq!(parse_and_validate_guess("abc"), None);
        assert_eq!(parse_and_validate_guess("-1"), None);
        assert_eq!(parse_and_validate_guess(""), None);
    }
}