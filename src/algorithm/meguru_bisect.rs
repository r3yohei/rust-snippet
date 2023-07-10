use cargo_snippet::snippet;

#[snippet("r3yohei_is_ok")]
fn is_ok(mid: i64) -> bool {
    todo!();
}

/// めぐる式二分探索(https://aotamasaki.hatenablog.com/entry/meguru_bisect)
/// ng: '取り得る最小の値' - 1
/// ok: '取り得る最大の値' + 1
#[snippet("r3yohei_meguru_bisect")]
#[snippet(include = "r3yohei_is_ok")]
fn meguru_bisect(mut ng: i64, mut ok: i64) -> i64 {
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
