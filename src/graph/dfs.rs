use cargo_snippet::snippet;

/// 無向グラフの隣接リスト表現に対するDFS
#[snippet("r3yohei_dfs")]
fn dfs(crt: usize, pre: usize, edges: &Vec<Vec<usize>>, mut visited: &mut Vec<bool>) {
    visited[crt] = true;
    todo!("行きがけの処理");
    for &to in &edges[crt] {
        if visited[to] {
            continue;
        }
        dfs(to, crt, edges, visited);
    }
    todo!("帰りがけの処理");
}