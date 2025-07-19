// 推測ゲーム
use rand::Rng;
use std::io::{self, Write};
use std::cmp::Ordering;

// 文字列をparseしてOption<u32>型で返却する
pub fn parse_and_validate_guess(input: &str) -> Option<u32> {
    // 文字列変換
    input.trim().parse::<u32>().ok().and_then(|num| {
        if (1..=100).contains(&num) {
            Some(num)
        } else {
            None
        }
    })
}

// 入出力処理
pub fn get_guess() -> Option<u32> {
    // Label
    print!("1 ~ 100までの数値を入力してください: ");
    // 変数格納準備
    let mut guess = String::new();
    // 入力待機（バッファを解放する）
    match io::stdout().flush() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("バッファの解放に失敗しました: {e}");
            return None;
        }
    }
    // 文字格納
    match io::stdin().read_line(&mut guess) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("文字入力に失敗しました: {e}");
            return None;
        }
    }
    parse_and_validate_guess(&guess)
}

// 処理実行
pub fn guess() {
    // Title
    println!("--- 数当てゲーム ---");
    // COMP値
    let secret_number = rand::rng().random_range(1..=100) as u32;

    loop {
        let guess = match get_guess() {
            Some(num) => num,
            None => {
                println!("1 ~ 100までの数値を入力してください・・・");
                continue;
            }
        };
        // 比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("入力値:{guess}よりも大きい値です"),
            Ordering::Greater => println!("入力値:{guess}よりも小さい値です"),
            Ordering::Equal => {
                println!("大正解！！");
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
        assert_eq!(parse_and_validate_guess("50"), Some(50));
        assert_eq!(parse_and_validate_guess("100"), Some(100));
        assert_eq!(parse_and_validate_guess("1"), Some(1));
        assert_eq!(parse_and_validate_guess("75"), Some(75));
    }

    #[test]
    fn test_parse_and_validate_guess_invalid() {
        assert_eq!(parse_and_validate_guess("0"), None);
        assert_eq!(parse_and_validate_guess("1000"), None);
        assert_eq!(parse_and_validate_guess("-1"), None);
        assert_eq!(parse_and_validate_guess("abc"), None);
        assert_eq!(parse_and_validate_guess(""), None);
    }
}