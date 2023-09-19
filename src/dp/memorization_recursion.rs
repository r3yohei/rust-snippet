use cargo_snippet::snippet;

/// メモ化再帰関数
/// 例: 区間DP
#[snippet("r3yohei_memorization_recursion")]
pub fn memo_rec(l: usize, r: usize, a: &[i64], mut dp: &mut [Vec<Option<i64>>]) -> i64 {
    // ベースケース
    if l == r {return 0;}

    return match dp[l][r] {
        Some(val) => val,
        None => {
            let val = memo_rec(l+1, r, &a, &mut dp).max(memo_rec(l, r-1, &a, &mut dp));
            dp[l][r] = Some(val);
            val
        },
    };
}