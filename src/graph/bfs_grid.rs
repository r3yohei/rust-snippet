use cargo_snippet::snippet;
use std::collections::VecDeque;

/// 二次元グリッド上のBFS
#[snippet("r3yohei_bfs_grid")]
fn bfs(c: &Vec<Vec<char>>, s_x: usize, s_y: usize) {
    let h = c.len();
    let w = c[0].len();
    const DIJ: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];
    let mut deque = VecDeque::new();
    let mut visited = vec![vec![false; w]; h];
    deque.push_back((s_x, s_y));
    visited[s_x][s_y] = true;

    while let Some((crt_x, crt_y)) = deque.pop_front() {
        // 4方向それぞれに進めるかチェック
        for i in 0..4 {
            // 範囲外参照を防ぐ
            let to_x = crt_x.wrapping_add(DIJ[i].0);
            let to_y = crt_y.wrapping_add(DIJ[i].1);
            if to_x < h && to_y < w {
                // 進めるかつ未訪問なら進む
                if c[to_x][to_y] == '.' && !visited[to_x][to_y] {
                    // 訪問先を次の始点候補にする
                    deque.push_back((to_x, to_y));
                    // 訪問済みにする
                    visited[to_x][to_y] = true;
                }
            }
        }
    }
}