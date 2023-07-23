use cargo_snippet::snippet;

/// n <= 200くらいまでの組合せ
#[snippet("r3yohei_ncr_small")]
fn nCr_small(n: u128, r: u128) -> u128 {
    //! オーバーフローしないように，以下の工夫をする
    //! nCr = n*(n-1)*...*(n-r+1) / 1*2*...*r より，
    //! n/1 * (n-1)/2 *...* (n-r+1) / r
    let mut ncr = 1;
    for i in 1..=r {
        ncr *= n - i + 1;
        ncr /= i;
    }
    ncr
}

/// p-1C0 ~ p-1Cp-1 をDPで求める
/// comb[i][j] := iCj mod p (i >= j)
/// iCj = i-1Cj-1 + i-1Cjであることを利用
#[snippet("r3yohei_ncr_mod")]
fn ncr_mod(p: usize) -> Vec<Vec<usize>> {
    let mut comb = vec![vec![0; p]; p];
    comb[0][0] = 1; // 0C0 = 1
    for i in 1..p {
        // 各行先頭はp-1C0 = 1
        comb[i][0] = 1;
        for j in (1..=i).rev() {
            // i >= j, j >= 1なるjのみについて，二項係数を以下のように逐次的に計算
            comb[i][j] = (comb[i-1][j-1] + comb[i-1][j]) % p;
        }
    }
    comb
}

/// リュカの定理によりnCr mod pを求める
/// O(p^2 + log_p(n))
#[snippet("r3yohei_lucas_theorem")]
#[snippet(include="r3yohei_ncr_mod")]
fn ncr_lucas(mut n: usize, mut r: usize, p: usize) -> usize {
    let mut ret = 1;
    let comb = ncr_mod(p);
    while n > 0 {
        let n_next = n % p;
        let r_next = r % p;
        ret *= comb[n_next][r_next];
        ret %= p;
        n /= p;
        r /= p;
    }
    ret
}

#[test]
fn test_nCr_small() {
    let n = 16;
    let r = 11;
    assert_eq!(nCr_small(n, r), 4368);
}