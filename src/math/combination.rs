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

#[test]
fn test_nCr_small() {
    let n = 16;
    let r = 11;
    assert_eq!(nCr_small(n, r), 4368);
}