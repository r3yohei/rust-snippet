use cargo_snippet::snippet;
use superslice::*;

/// 最長部分増加列(LIS)を求める O(NlogN)
/// https://qiita.com/python_walker/items/d1e2be789f6e7a0851e5
#[snippet("r3yohei_lis")]
fn lis(a: &[i64]) -> usize {
    let inf: i64 = 1_000_000_000;
    // LISそのものを格納するDP配列
    let mut dp = vec![inf; a.len()];
    // aの各数値がLISでは何番目の要素になるかを格納する配列
    let mut p = vec![0; a.len()];
    for (i, &ai) in a.iter().enumerate() {
        let j = dp.lower_bound(&ai);
        dp[j] = ai;
        p[i] = j + 1;
    }
    let mut ans = 0;
    for i in 0..a.len() {
        if dp[i] != inf {ans += 1;}
    }
    ans
}

#[test]
fn test_lis() {
    let v = vec![4, 2, 3, 1, 5];
    assert_eq!(lis(&v), 3);
}