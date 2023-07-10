
use cargo_snippet::snippet;

/// 二次元グリッド上のDFS
#[snippet("r3yohei_dfs_grid")]
fn dfs(c: &Vec<Vec<char>>, crt_x: usize, crt_y: usize, prev_x: usize, prev_y: usize, mut visited: &mut Vec<Vec<bool>>) {
    const DIJ: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];
    visited[crt_x][crt_y] = true;
    todo!("行きがけの処理");
    for i in 0..4 {
        // 範囲外参照を防ぐ
        let to_x = crt_x.wrapping_add(DIJ[i].0);
        let to_y = crt_y.wrapping_add(DIJ[i].1);
        if to_x < c.len() && to_y < c[0].len() {
            // 行き先が元の頂点 or 行き先に道がない or 行ったことがあるなら飛ばす
            if (to_x == prev_x && to_y == prev_y) || c[to_x][to_y] == '#' || visited[to_x][to_y] {
                continue;
            }
            dfs(c, to_x, to_y, crt_x, crt_y, &mut visited);
        }
    }
    todo!("帰りがけの処理");
}