use cargo_snippet::snippet;
use std::collections::HashMap;

/// メモ化再帰関数
/// 例: フィボナッチ数列
#[snippet("r3yohei_memorization_recursion")]
pub fn memo_rec(n: i64, mut memo: &mut HashMap<i64, i64>) -> i64 {
    return match memo.get(&n) {
        None => {
            let new = memo_rec(n-2, memo) + memo_rec(n-1, memo);
            memo.insert(n, new);
            new
        },
        _ => memo[&n]
    }
}