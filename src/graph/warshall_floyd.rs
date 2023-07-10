use cargo_snippet::snippet;

/// ワーシャルフロイド法
/// 全点対最短経路問題を解くアルゴリズム
/// O(V^3)
/// 制約がN<=10^2くらいのときにエスパー可能
#[snippet("r3yohei_warshall_floyd")]
fn warshall_floyd(dist: &mut Vec<Vec<i64>>) {
    let n = dist.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // (i,j)間はkを経由したほうが短くなるか調べる
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }
}