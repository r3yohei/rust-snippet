use cargo_snippet::snippet;

/// ベクタのargsort
#[snippet("r3yohei_argsort")]
fn argsort<T: Ord>(v: &[T]) -> Vec<usize> {
    let mut idx = (0..v.len()).collect::<Vec<_>>();
    idx.sort_by(|&i, &j| v[i].cmp(&v[j])); // 昇順
    // idx.sort_by(|&i, &j| v[j].cmp(&v[i])); // 降順
    idx
}

#[test]
fn test_argsort() {
    let v = vec![3, 2, 1, 5, 7];
    let sorted_idx = argsort(&v);
    assert_eq!(sorted_idx, vec![2, 1, 0, 3, 4]);
}