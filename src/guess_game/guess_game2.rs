// 何度も書いて覚えるためのコード（数推定ゲーム）
// テストも考えたコードにする。基本的に入力を伴う部分とロジックは分離させます。
use rand::Rng;
use std::io::{self, Write};
use std::cmp::Ordering;

fn parse_and_validate_guess(input: &str) -> Option<u32> {
    input.trim().parse::<u32>().ok().and_then(|num| {
        if (1..=100).contains(&num) {
            Some(num)
        } else {
            None
        }
    })
}

fn get_guess() -> Option<u32> {
    print!("Input your guess number(1 ~ 100):: ");
    let mut guess = String::new();
    // バッファのフラッシュ
    match io::stdout().flush() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("出力のフラッシュに失敗しました: {e}");
            return None;
        }
    }
    // 入力待機
    match io::stdin().read_line(&mut guess) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("入力の読み込みに失敗しました: {e}");
            return None;
        },
    }
    // 値をu32にパースして戻す
    parse_and_validate_guess(&guess)
}

pub fn guess() {
    println!("--- 数字当てゲーム ---");
    // コンピュータ側の数値設定
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        let guess = match get_guess() {
            Some(num) => num,
            None => {
                println!("1 ~ 100までの有効な値を入力してください・・・");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("推測の値({guess})よりも大きな値です。"),
            Ordering::Greater => println!("推測の値({guess})よりも小さな値です。"),
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
        assert_eq!(parse_and_validate_guess("101"), None);
        assert_eq!(parse_and_validate_guess("-5"), None);
        assert_eq!(parse_and_validate_guess("abc"), None);
        assert_eq!(parse_and_validate_guess(""), None);
    }
}