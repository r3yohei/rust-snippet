use cargo_snippet::snippet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// 隣接リスト表現に対するダイクストラ法
#[snippet("r3yohei_dijkstra")]
fn dijkstra(edges: &Vec<Vec<(usize, i64)>>, s: usize) -> (Vec<i64>, Vec<usize>) {
    let inf = 1 << 60;
    let mut bh = BinaryHeap::new();
    bh.push((Reverse(0), s));
    let mut dist = vec![inf; edges.len()];
    dist[s] = 0;
    let mut prev = vec![!0; edges.len()];
    
    while let Some((Reverse(d), crt)) = bh.pop() {
        // 見ようとしているものより既にいい経路が見つかっていれば飛ばす
        if dist[crt] < d { continue; }
        for &(to, w) in &edges[crt] {
            // 更新したほうがいいなら更新して，優先度付きキューに入れる
            if d + w < dist[to] {
                bh.push((Reverse(dist[to]), to));
                dist[to] = d + w;
                prev[to] = crt;
            }
        }
    }

    (dist, prev)
}

/// ダイクストラ法経路復元
#[snippet("r3yohei_dijkstra")]
fn restore_dijkstra(prev: &Vec<usize>, t: usize) -> Vec<usize> {
    let mut path = vec![];
    let mut tt = t;
    while tt != !0 {
        path.push(tt);
        tt = prev[tt];
    }
    path.reverse();

    path
}