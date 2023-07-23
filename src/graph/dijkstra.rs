use cargo_snippet::snippet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// 隣接リスト表現に対するダイクストラ法
#[snippet("r3yohei_dijkstra")]
fn dijkstra(edges: &Vec<Vec<(usize, i64)>>, s: usize) -> Vec<i64> {
    let inf = 1_000_000_000;
    let mut dist = vec![inf; edges.len()];
    dist[s] = 0;
    let mut bh = BinaryHeap::new();
    bh.push((Reverse(0), s));
    while let Some((Reverse(dist_crt), crt)) = bh.pop() {
        // 見ようとしているものより既にいい経路が見つかっていれば飛ばす
        if dist[crt] < dist_crt {continue;}
        for &(to, w) in &edges[crt] {
            // 更新したほうがいいなら更新して，優先度付きキューに入れる
            if dist_crt + w < dist[to] {
                dist[to] = dist[crt] + w;
                // rustの優先度付きキューは最大値を取り出すので，Reverseでwrapする
                bh.push((Reverse(dist[to]), to));
            }
        }
    }
    dist
}