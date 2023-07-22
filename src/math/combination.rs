fn combination(n: usize, r: usize) -> usize {
    let mut ncr = vec![vec![0; r+1]; n+1];
    ncr[0][0] = 1;
    for i in 1..n {
        ncr[i][0] = 1;
        for j in 1..=n.min(r) {
            ncr[i][j] = ncr[i-1][j-1] + ncr[i-1][j];
        }
    }
    ncr[n][r]
}