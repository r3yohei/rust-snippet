use cargo_snippet::snippet;

/// 二次元配列の転置を求める
#[snippet("r3yohei_transpose")]
fn transpose(a: &Vec<Vec<i64>>) -> Vec<Vec<i64>>{
    let h = a.len();
    let w = a[0].len();
    let mut a_t = vec![vec![0; h]; w];
    for hi in 0..h {
        for wi in 0..w {
            a_t[wi][hi] = a[hi][wi];
        }
    }
    a_t
}