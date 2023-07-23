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

/// 二次元グリッド上で，壁にぶつかるまで動くBFS
#[snippet("r3yohei_bfs_grid_until_wall")]
fn bfs_until_wall(c: &Vec<Vec<char>>, s_x: usize, s_y: usize) {
    let h = c.len();
    let w = c[0].len();
    const DIJ: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];
    let mut deque = VecDeque::new();
    let mut visited = vec![vec![false; w]; h];
    deque.push_back((s_x, s_y));
    visited[s_x][s_y] = true;

    let mut stopped = vec![vec![false; w]; h];

    while let Some((frm_x, frm_y)) = deque.pop_front() {
        for &(dx, dy) in &DIJ {
            let mut to_x = frm_x;
            let mut to_y = frm_y;
            while c[to_x.wrapping_add(dx)][to_y.wrapping_add(dy)] == '.' {
                to_x = to_x.wrapping_add(dx);
                to_y = to_y.wrapping_add(dy);
                visited[to_x][to_y] = true;
            }
            // 初めて来た停止点なら，次の開始点に入れる
            if !stopped[to_x][to_y] {
                stopped[to_x][to_y] = true;
                deque.push_back((to_x, to_y));
            }
        }
    }

    // 例えば，触れられる点すべての個数を出すなど
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if visited[i][j] {
                ans += 1;
            }
        }
    }
}