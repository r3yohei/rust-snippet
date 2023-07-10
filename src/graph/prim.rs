use cargo_snippet::snippet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


/// プリム法により最小全域木のコストを返す
/// O(N + MlogN)
#[snippet("r3yohei_prim")]
pub fn prim(edges: &Vec<Vec<(usize, i64)>>) -> i64 {
    let mut cost = 0;
    let mut visited = vec![false; edges.len()];
    // 重み最小の頂点を取り出すための優先度付きキュー
    let mut bh = BinaryHeap::new();
    // 頂点0をスタート地点とする
    bh.push((Reverse(0), 0, !0));

    // 最小の重みの辺をqueから取り出す
    while let Some((Reverse(w1), crt, frm)) = bh.pop() {
        // 訪問済みならスキップ
        if visited[crt] {continue;}
        visited[crt] = true;
        // 最初以外グラフへ記録する
        if frm != !0 {
            cost += w1;
        }
        // frmに隣接する頂点について，コストの小さい順に詰める
        for &(to, w2) in &edges[crt] {
            if visited[to] {continue;}
            bh.push((Reverse(w2), to, crt));
        }
    }
    cost
}