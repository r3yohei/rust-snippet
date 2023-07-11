use cargo_snippet::snippet;

/// イテレータの並びのうち，辞書順で次の並びのものがあるかどうかを判定する
/// あれば，イテレータも並び替えられる
#[snippet("r3yohei_next_permutation")]
pub fn next_permutation<T>(a: &mut [T]) -> bool
where
    T: PartialOrd,
{
    let n = a.len();
    for i in (1..n).rev() {
        if a[i - 1] < a[i] {
            let mut j = n - 1;
            while a[i - 1] >= a[j] {
                j -= 1;
            }
            a.swap(i - 1, j);
            a[i..n].reverse();
            return true;
        }
    }
    a.reverse();
    false
}

#[test]
fn test_next_permutation() {
    let mut v = vec![5, 1, 2, 10];
    assert!(next_permutation(&mut v));
    assert_eq!(v, vec![5, 1, 10, 2]);
}
