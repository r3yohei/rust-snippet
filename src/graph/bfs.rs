use cargo_snippet::snippet;
use std::collections::VecDeque;

/// 隣接リスト表現に対するBFS
#[snippet("r3yohei_bfs")]
fn bfs(edges: &Vec<Vec<usize>>, s: usize) {
    let mut deque = VecDeque::new();
    deque.push_back(s);
    let mut dist = vec![-1; edges.len()]; // -1は未訪問を示す
    // 頂点sから各頂点への最短距離を格納するベクタ
    dist[s] = 0; // 始点自身への距離は0
    while let Some(crt) = deque.pop_front() {
        for &to in &edges[crt] {
            if dist[to] == -1 {
                dist[to] = dist[crt] + 1;
                deque.push_back(to);
            }
        }
    }
}