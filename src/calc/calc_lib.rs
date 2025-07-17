use std::io;

pub fn double_value(num: &mut i64) {
    *num *= 2
}

pub fn calc_example() {
    // title
    println!("--- 数値を2倍にします ---");
    // 入力値格納用変数
    let mut my_number = String::new();
    // 入力待機
    match io::stdin().read_line(&mut my_number) {
        Ok(_) => (),
        Err(e) => panic!("標準入力に失敗しました: {e}"),
    }
    // 文字列から数値(i64)変換
    let mut my_number = match my_number.trim().parse::<i64>() {
        Ok(num) => num,
        Err(_) => panic!("数値変換に失敗しました！"),
    };
    double_value(&mut my_number);
    println!("結果: {my_number}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_value() {
        let mut num1 = 5;
        double_value(&mut num1);
        assert_eq!(num1, 10);

        let mut num2 = -10;
        double_value(&mut num2);
        assert_eq!(num2, -20);

        let mut num3 = 0;
        double_value(&mut num3);
        assert_eq!(num3, 0);
    }
}