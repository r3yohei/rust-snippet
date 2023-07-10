use cargo_snippet::snippet;

/// ランレングス符号化
/// s = "RRRLLRLRRLLLLRLRR"などを
/// [(R, 3), (L, 2), (R, 1), (L, 1), (R, 2), (L, 4), (R, 1), (L, 1), (R, 2)]
/// のように変換する
/// O(n)
#[snippet("r3yohei_run_length_encoding")]
fn run_length_encoding(s: Vec<i64>) -> Vec<(i64, usize)> {
    let mut tmp = s[0];
    let mut cnt = 0;
    let mut encode = vec![];
    for &si in &s {
        if tmp == si {
            cnt += 1;
        } else {
            encode.push((tmp, cnt));
            tmp = si;
            cnt = 1;
        }
    }
    encode.push((tmp, cnt));
    encode
}