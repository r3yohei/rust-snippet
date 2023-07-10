use cargo_snippet::snippet;

/// x^p mod MODを繰り返し二乗法により求める
/// O(log(p))
#[snippet("r3yohei_modpow")]
fn modpow(mut x: i64, mut p: i64, mod_num: i64) -> i64 {
    let mut ret = 1;
    while p > 0 {
        if p & 1 == 1 {
            ret *= x;
            ret %= mod_num;
        }
        x *= x;
        x %= mod_num;
        p >>= 1;
    }
    ret
}

/// MODを法とするxの逆元を見つける
#[snippet("r3yohei_modinv")]
#[snippet(include = "r3yohei_modpow")]
fn modinv(x: i64, mod_num: i64) -> i64 {
    modpow(x, mod_num - 2, mod_num)
}