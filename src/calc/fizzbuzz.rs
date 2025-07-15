// Fizzbuzz対応について
// プログラムによくある問題で
// - [3の倍数]であるとき[fizz]を表示し・・・
// - [5の倍数]であるとき[buzz]を表示する。
// - そして[3と5の公倍数]であるときに[fizz buzz]を表示するプログラムである。

use std::array;

fn get_fizzbuzz_string(n: i32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "fizz buzz".to_string(),
        (0, _) => "fizz".to_string(),
        (_, 0) => "buzz".to_string(),
        _ => n.to_string()
    }
}

pub fn fizzbuzz() {
    for i in 1..=100 {
        println!("{}", get_fizzbuzz_string(i));
    }
}

fn get_vowel_and_consonant(v: char) -> String {
    match v {
        'a' | 'i' | 'u' | 'e' | 'o' => "vowel".to_string(),
        _ => "consonant".to_string(),
    }
}

pub fn vowel_and_consonant() {
    let alphabet_chars: [char; 26] = array::from_fn(|i| {
        (b'a' + i as u8) as char
    });
    for v in alphabet_chars.iter() {
        println!("{}: {}", v, get_vowel_and_consonant(*v));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fizzbuzz_string() {
        // 各ケースを個別にテスト
        assert_eq!(get_fizzbuzz_string(1), "1");
        assert_eq!(get_fizzbuzz_string(2), "2");
        assert_eq!(get_fizzbuzz_string(3), "fizz");
        assert_eq!(get_fizzbuzz_string(4), "4");
        assert_eq!(get_fizzbuzz_string(5), "buzz");
        assert_eq!(get_fizzbuzz_string(10), "buzz");
        assert_eq!(get_fizzbuzz_string(15), "fizz buzz");
    }

    #[test]
    fn test_vowal_and_consonant() {
        assert_eq!(get_vowel_and_consonant('a'), "vowel");
        assert_eq!(get_vowel_and_consonant('z'), "consonant");
    }
}
