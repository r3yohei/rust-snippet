use cargo_snippet::snippet;
use std::collections::VecDeque;

/// 隣接リスト表現に対するBFS
#[snippet("r3yohei_bfs")]
fn bfs(edges: &Vec<Vec<usize>>, s: usize) -> (Vec<i64>, Vec<usize>) {
    let mut deque = VecDeque::new();
    deque.push_back(s);
    // 頂点sから各頂点への最短距離を格納するベクタ
    let mut dist = vec![-1; edges.len()]; // -1は未訪問を示す
    dist[s] = 0; // 始点自身への距離は0
    // 経路復元用
    let mut prev = vec![!0; edges.len()]; // !0は未設定を示す
    while let Some(crt) = deque.pop_front() {
        for &to in &edges[crt] {
            if dist[to] == -1 {
                dist[to] = dist[crt] + 1;
                prev[to] = crt;
                deque.push_back(to);
            }
        }
    }

    (dist, prev)
}

/// BFS経路復元
#[snippet("r3yohei_bfs")]
fn restore_bfs(prev: &Vec<usize>, t: usize) -> Vec<usize> {
    let mut path = vec![];
    let mut tt = t;
    while tt != !0 {
        path.push(tt);
        tt = prev[tt];
    }
    path.reverse();

    path
}

/// 隣接リスト表現に対する01BFS
/// 辺のコストが0or1のグラフに使用する
#[snippet("r3yohei_01bfs")]
fn zero_one_bfs(edges: &Vec<Vec<(usize, i64)>>, s: usize) -> Vec<i64> {
    //! 辺のコストが1ならdequeの右端，0なら左端を繰り返す
    //! dequeの中身の頂点の暫定最短距離が常に(1,1,2,3,4,4,..)
    //! のように，左と同じかそれ+1であるように並ぶ
    //! 暫定距離が短いものからpop_frontしたいのでこうなる
    //! ダイクストラでheapqを使うのと同じ気持ちだが，dequeなので計算量がO(V+ElogV)からO(V+E)に落ちる
    let n = edges.len();
    const INF: i64 = 1_000_000_000;
    let mut dist = vec![INF; n];
    dist[s] = 0;
    let mut deque = VecDeque::new();
    deque.push_back(s);

    while let Some(frm) = deque.pop_front() {
        for &(to, cost) in &edges[frm] {
            let d = dist[frm] + cost;
            // frmから行くほうが短いなら更新する
            if d < dist[to] {
                dist[to] = d;
                if cost == 1 {
                    // 辺のコストが1なら，右端に詰める
                    deque.push_back(to);
                } else {
                    // 0なら，左端
                    deque.push_front(to);
                }
            }
        }
    }

    dist
}
