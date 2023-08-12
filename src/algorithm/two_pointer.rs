use cargo_snippet::snippet;

/// while1重尺取法
/// https://zenn.dev/luke256/articles/0d60a95fd86ffa
#[snippet("r3yohei_TwoPointer")]
struct TwoPointer {
    n: usize,
    t: Vec<i64>,
}
#[snippet("r3yohei_TwoPointer")]
impl TwoPointer {
    fn new(n: usize, t: Vec<i64>) -> Self {
        Self { n, t }
    }
    fn run(&mut self) {
        let mut l = 0;
        let mut r = 0;
        while l < self.n {
            if r == self.n || todo!("条件を満たさない場合を記載") {
                // 処理a
                l += 1;
            } else {
                // 処理a'
                r += 1;
            }
            // 処理b
        }
    }
}
