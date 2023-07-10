use cargo_snippet::snippet;
use itertools::Itertools;
use nalgebra::{DMatrix, DVector};

/// N件のソースからM件のターゲットにいくらかモノを運ぶ
/// 各ソースsiにはai個の供給があり，各ターゲットtiにはbi個の需要がある
/// Σ(1~n)ai = Σ(1~M)biとする
/// si->tj にモノを運ぶのに，cijの輸送コストがかかる
/// si->tjへの輸送量xijを適切に決めて，各ターゲットの要求を満たしつつ輸送コストの和を最小化する
/// [つるさんのブログ](https://theory-and-me.hatenablog.com/entry/2021/05/09/181435)
#[snippet("r3yohei_sinkhorn_knopp")]
fn sinkhorn_knopp(a: Vec<f64>, b: Vec<f64>, C: Vec<Vec<f64>>, lambda: f64, tolerance: f64) -> Vec<Vec<f64>> {
    // Cの各成分cijをexp(-λ * cij)にした行列Kを作る
    let K = DMatrix::from_row_slice(
        C.len(),
        C[0].len(),
        &C.iter().flatten().map(|&cij| f64::exp(-lambda * cij)).collect_vec(),
    );
    // Kの転置を用意する
    let K_t = K.transpose();
    // ソースの供給量ベクタ，ターゲットの需要量ベクタをDVector型になおす
    let a = DVector::from_vec(a);
    let b = DVector::from_vec(b);
    // sinkhorn-knoppアルゴリズムのU, Vの元
    let mut u = DVector::from_element(C.len(), 1.0);
    let mut v = b.component_div(&(&K_t * &u));
    // u, vが収束するまでループ
    let mut iter = 0;
    loop {
        iter += 1;
        let next_u = a.component_div(&(&K * &v));
        let next_v = b.component_div(&(&K_t * &u));

        let error = (&(&next_u - &u)).norm() + (&(&next_v - &v)).norm();
        if error < tolerance {
            eprintln!("converged");
            break;
        }

        u = next_u;
        v = next_v;
    }
    let U = DMatrix::from_diagonal(&u);
    let V = DMatrix::from_diagonal(&v);

    let R = U * K * V;
    let R: Vec<Vec<f64>> = R.row_iter().map(|r| r.iter().map(|&c| c).collect()).collect();
    R
}

#[test]
fn test_sinkhorn_knopp() {
    // [参考](http://www.kogures.com/hitoshi/webtext/or-lp-yusou/index.html)
    // ここで色々試して手で解を作り，検算できる
    let a = vec![18.0, 22.0, 26.0];
    let row = a.len();
    let b = vec![12.0, 20.0, 16.0, 18.0];
    let col = b.len();
    let C = vec![
        vec![7.0, 3.0, 2.0, 10.0],
        vec![9.0, 3.0, 6.0, 8.0],
        vec![8.0, 7.0, 8.0, 6.0],
    ];

    let result = sinkhorn_knopp(a, b, C, 10.0, 0.001);
    let opt = vec![
        vec![2.0, 0.0, 16.0, 0.0],
        vec![2.0, 20.0, 0.0, 0.0],
        vec![8.0, 0.0, 0.0, 18.0],
    ];
    for i in 0..row {
        for j in 0..col {
            assert!((result[i][j] - opt[i][j]).abs() < 0.001)
        }
    }
}