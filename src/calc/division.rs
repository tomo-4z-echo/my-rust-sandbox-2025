pub fn division(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 正常な割り算のテストケース
    fn test_normal_division() {
        let numerator = 30.0;
        let denominator = 5.0;
        let expected_result = Some(6.0);
        let actual_result = division(numerator, denominator);
        // assert_eq!マクロを使用して、実際の出力と期待される出力を比較します
        assert_eq!(actual_result, expected_result, "30.0 / 5.0 = Some(6.0)であるべき"); 
    }

    #[test]
    // 0で割る場合のテストケース
    fn test_division_by_zero() {
        let numerator = 30.0;
        let denominator = 0.0;
        let expected_result = None;
        let actual_result = division(numerator, denominator);
        // assert_eq!マクロを使用して、実際の出力と期待される出力を比較します
        assert_eq!(actual_result, expected_result, "30.0 / 0.0 = Noneであるべき");
    }

    #[test]
    // 負の数を含む場合のテストケース
    fn test_negative_division() {
        let mut numerator = 30.0;
        let mut denominator = -5.0;
        let mut expected_result = Some(-6.0);
        let mut actual_result = division(numerator, denominator);
        // assert_eq!マクロを使用して、実際の出力と期待される出力を比較します
        assert_eq!(actual_result, expected_result, "30.0 / -5.0 = Some(-6.0)であるべき");

        numerator = -30.0;
        denominator = 5.0;
        expected_result = Some(-6.0);
        actual_result = division(numerator, denominator);
        // assert_eq!マクロを使用して、実際の出力と期待される出力を比較します
        assert_eq!(actual_result, expected_result, "-30.0 / 5.0 = Some(-6.0)であるべき");

        // マイナスとマイナスをかけた場合にプラスになること
        numerator = -30.0;
        denominator = -5.0;
        expected_result = Some(6.0);
        actual_result = division(numerator, denominator);
        // assert_eq!マクロを使用して、実際の出力と期待される出力を比較します
        assert_eq!(actual_result, expected_result, "-30.0 / -5.0 = Some(6.0)であるべき");
    }

    #[test]
    // 小数点以下の割り算のケース
    fn test_decimal_division() {
        let numerator = 7.0;
        let denominator = 2.0;
        let expected_result = Some(3.5);
        let actual_result = division(numerator, denominator);
        assert_eq!(actual_result, expected_result, "7.0 / 2.0 = Some(3.5)であるべき");
    }

    #[test]
    // 結果がSomeの場合に値を取り出して検証する例（if letを使用）
    fn test_division_and_unwrap_some() {
        let numerator = 15.0;
        let denominator = 3.0;
        let actual_result = division(numerator, denominator);
        // if letを使ってSomeの中身を取り出しその値が正しいか検証する
        if let Some(result_value) = actual_result {
            assert_eq!(result_value, 5.0, "15.0 / 3.0 = 5.0であるべき");
        } else {
            // ここに到達したらテストは失敗とみなす
            panic!("15.0 / 3.0 = Someを返すべきなのにNoneを返した");
        }
    }
}
