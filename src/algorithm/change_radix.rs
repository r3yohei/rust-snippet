use cargo_snippet::snippet;

/// 文字列の数字sをa進数からb進数の文字列に変換する
/// 大きな桁の数字を扱うために文字列としている
#[snippet("r3yohei_change_radix")]
pub fn change_radix(s: &str, a: i64, b: i64) -> String {
    let mut base10: i64 = 0;
    for (i, c) in s.chars().rev().enumerate() {
        base10 += c.to_digit(10).unwrap() as i64 * a.pow(i as u32);
    }
    if base10 == 0 {return "0".to_string();}

    let mut result = String::new();
    while base10 != 0 {
        result.push(std::char::from_digit((base10 % b) as u32, 10).unwrap());
        base10 /= b;
    }
    return result.chars().rev().collect::<String>();
}

#[test]
fn test_change_radix() {
    let s = "7";
    let s_radix_2 = change_radix(s, 10, 2);
    assert_eq!(s_radix_2, "111");
}
