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
fn bfs_until_wall(c: &Vec<Vec<char>>, s_x: usize, s_y: usize) -> Vec<Vec<bool>> {
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
    visited
}

/// 二次元グリッド上での，01BFS
/// 例えば方向を状態に持てば壁にぶつかるまでの間好きなだけ方向転換できる
#[snippet("r3yohei_01bfs_grid")]
fn zero_one_bfs(c: &Vec<Vec<char>>, s_x: usize, s_y: usize) -> Vec<Vec<i64>> {
    let h = c.len();
    let w = c[0].len();
    // 十字
    // const DIJ: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];
    // 斜め
    const DIJ: [(usize, usize); 4] = [(!0, !0), (!0, 1), (1, !0), (1, 1)];
    let mut deque = VecDeque::new();
    deque.push_back((s_x, s_y));
    const INF: i64 = 1_000_000_000;
    let mut dist = vec![vec![INF; w]; h];
    dist[s_x][s_y] = 0;

    while let Some((frm_x, frm_y)) = deque.pop_front() {
        let to_cost = dist[frm_x][frm_y] + 1;
        for &(dx, dy) in &DIJ {
            let mut to_x = frm_x.wrapping_add(dx);
            let mut to_y = frm_y.wrapping_add(dy);
            // 行ける限り行く
            while to_x < h && to_y < w && c[to_x][to_y] == '.' {
                if to_cost < dist[to_x][to_y] {
                    // コストを更新できるなら更新しつつそこを開始点に入れる
                    deque.push_back((to_x, to_y));
                    dist[to_x][to_y] = to_cost;
                } else if to_cost > dist[to_x][to_y] {
                    break;
                }
                to_x = to_x.wrapping_add(dx);
                to_y = to_y.wrapping_add(dy);
            }
        }
    }
    dist
}

#[test]
fn test_bfs_until_wall() {
    let H = 6;
    let W = 6;
    // #:壁
    // .:行けるところ
    let c = vec![
        vec!['#', '#', '#', '#', '#', '#'],
        vec!['#', '.', '.', '.', '.', '#'],
        vec!['#', '.', '#', '.', '.', '#'],
        vec!['#', '.', '.', '#', '.', '#'],
        vec!['#', '.', '.', '.', '.', '#'],
        vec!['#', '#', '#', '#', '#', '#'],
    ];
    // (1, 1)開始
    let (s_x, s_y) = (1, 1);
    // 壁にぶつかるまで止まれない時に訪問できる頂点
    let visited = bfs_until_wall(&c, s_x, s_y);

    // 例えば，触れられる点すべての個数を出すなど
    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if visited[i][j] {
                ans += 1;
            }
        }
    }
    assert_eq!(ans, 12);
}
