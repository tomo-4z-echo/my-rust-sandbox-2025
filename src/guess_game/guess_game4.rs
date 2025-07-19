// 推測ゲーム４
// テストができるように標準入力とロジックを分離する
use rand::Rng;
use std::io::{self, Write};
use std::cmp::Ordering;

// ロジック: 受け取った文字列をトリミング・数値へパースして・Option<u32>で返す
pub fn parse_and_validate_guess(input: &str) -> Option<u32> {
    input.trim().parse::<u32>().ok().and_then(|num| {
        if (1..=100).contains(&num) {
            Some(num)
        } else {
            None
        }
    })
}

// 標準入出力
pub fn get_guess() -> Option<u32> {
    // lebal
    print!("数字を推測してください(1 ~ 100): ");
    // 入力データの格納準備
    let mut guess = String::new();
    match io::stdout().flush() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("バッファの初期化に失敗しました: {e}");
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
    parse_and_validate_guess(&guess)
}

// 実行処理
pub fn guess() {
    // title
    println!("--- 数当てゲーム ---");
    // コンピュータ側の設定値
    let secret_number = rand::rng().random_range(1..=100);
    // 繰り返し処理
    loop {
        let guess = match get_guess() {
            Some(num) => num,
            None => {
                println!("1 ~ 100までの数値で入力してください。");
                continue;
            }
        };
        // 比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("推測値[{guess}]よりも大きい数値です"),
            Ordering::Greater => println!("推測値[{guess}]よりも小さい数値です"),
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
        assert_eq!(parse_and_validate_guess("abc"), None);
        assert_eq!(parse_and_validate_guess("-1"), None);
        assert_eq!(parse_and_validate_guess(""), None);
    }
}