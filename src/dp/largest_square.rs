use cargo_snippet::snippet;

/// グリッド内の最大正方形の面積を求める O(HW)
/// https://qiita.com/H20/items/884551b4965739176afc
#[snippet("r3yohei_largest_square_in_grid")]
fn largest_square_in_grid(grid: &Vec<Vec<bool>>) -> Vec<Vec<i64>> {
    let H = grid.len();
    let W = grid[0].len();
    let mut dp = vec![vec![-1; W]; H];
    for i in 0..H {
        for j in 0..W {
            if grid[i][j] {
                if i == 0 || j == 0 {
                    // いずれか1方向からでも貰えない端のマスの面積は1になる
                    dp[i][j] = 1;
                } else {
                    // 3方向のから貰う
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                }
            } else {
                dp[i][j] = 0;
            }
        }
    }
    dp
}

#[test]
fn test_largest_square_in_grid() {
    let H = 6;
    let W = 10;
    let mut grid = vec![vec![true; W]; H];
    // 使えないグリッドを適当に与える
    // https://h20-dhmo.github.io/square.io/
    grid[0][1] = false;
    grid[2][4] = false;
    grid[2][6] = false;
    grid[2][9] = false;
    grid[3][9] = false;
    grid[5][7] = false;

    let dp = largest_square_in_grid(&grid);
    // dp配列の最大値は，最大正方形の1辺の長さとなっている
    let mut max_edge = 0;
    for i in 0..H {
        for j in 0..W {
            max_edge = max_edge.max(dp[i][j]);
        }
    }
    assert_eq!(max_edge * max_edge, 16);
}
