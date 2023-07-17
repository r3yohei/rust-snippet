use cargo_snippet::snippet;
use itertools::Itertools;
use nalgebra::{DMatrix, DVector};

/// ガウス過程回帰のカーネルのハイパーパラメータ
#[snippet("r3yohei_gaussian_process_regression")]
#[derive(Debug, Clone, Copy)]
struct GaussianProcessRegressionParameter {
    theta1: f64,
    theta2: f64,
    theta3: f64,
}
#[snippet("r3yohei_gaussian_process_regression")]
impl GaussianProcessRegressionParameter {
    fn new(theta1: f64, theta2: f64, theta3: f64) -> Self {
        Self {
            theta1,
            theta2,
            theta3,
        }
    }
}

/// 1次元ガウス過程回帰モデル
#[snippet("r3yohei_gaussian_process_regression")]
#[derive(Debug, Clone)]
struct GaussianProcessRegression {
    x: Vec<DVector<f64>>,
    y: Vec<f64>,
    param: GaussianProcessRegressionParameter, // カーネルのtheta
}
#[snippet("r3yohei_gaussian_process_regression")]
impl GaussianProcessRegression {
    pub fn new() -> Self {
        Self {
            x: vec![],
            y: vec![],
            param: GaussianProcessRegressionParameter::new(1.0, 1.0, 0.1), // 後で最尤推定する
        }
    }
    pub fn collect_data(&mut self, xi: DVector<f64>, yi: f64) {
        self.x.push(xi);
        self.y.push(yi);
    }
    // ガウス過程回帰により，新たな観測x_testが与えられたとき，対応するy_testの平均と分散を返す
    pub fn predict(&self, x_test: &DMatrix<f64>) -> (Vec<f64>, Vec<f64>) {
        // メンバ変数xをDMatrix型に変換
        let mut x_train = DMatrix::from_row_slice(
            self.x.len(),
            self.x[0].len(),
            &self.x.iter().flatten().copied().collect_vec(),
        );
        // メンバ変数yをDVector型に変換
        let mut y_train = DVector::from_vec(self.y.clone());

        // 平均はガウス過程回帰において本質的意味があまりない
        // カーネル関数で決められる共分散行列によってのみ形が決まる
        // よって，モデル内では平均は0として扱い，推定の前後で差し引きして帳尻を合わせる
        let y_average = y_train.mean();
        y_train.add_scalar_mut(-y_average);

        // trainとtestの長さを保持しておく
        let train_len = self.x.len();
        let test_len = x_test.shape().0;

        // 共分散行列の計算
        let kernel_mat = self.compute_kernel_matrix(&x_train);

        // y=Kxを解く
        let kernel_lu = kernel_mat.lu(); // 後で使いまわすのでLU分解の計算は1回だけにしたい
        let yy = kernel_lu.solve(&y_train).unwrap();

        // y_testの平均と分散を格納するベクタを作る
        let mut mean = vec![];
        let mut covariance = vec![];
        for j in 0..test_len {
            let mut k = vec![];
            for i in 0..train_len {
                let xi = x_train.row(i).transpose();
                let xj = x_test.row(j).transpose();
                let kernel = self.kernel(&xi, &xj, i, j);
                k.push(kernel);
            }
            let k = DVector::from_vec(k);
            let xj = x_test.row(j).transpose();
            let s = self.kernel(&xj, &xj, j + train_len, j + train_len);
            // 元の平均を戻しつつベクタに格納
            mean.push(k.dot(&yy) + y_average);
            covariance.push(s - (k.transpose() * kernel_lu.solve(&k).unwrap())[(0, 0)]);
        }
        (mean, covariance)
    }
    // カーネル関数
    // ここに，(i, j)間でどういう関係を持っていてほしいかの思想を組み込む
    fn kernel(&self, xi: &DVector<f64>, xj: &DVector<f64>, i: usize, j: usize) -> f64 {
        // ベクトルxi,xjの近さを，各成分ごとの差の二乗和とする
        // そうすれば，xでたとえば2次元座標を表したいとき，(1,2)と(1,1)はnormが小さくなり，(1,2)と(100,200)は大きくなる
        let diff = xi - xj;
        let norm = diff.component_mul(&diff).sum();
        let mut kernel = self.param.theta1 * (-norm / self.param.theta2.powf(2.0)).exp();
        // i == j なら自身なので観測ノイズも足す
        if i == j {
            kernel += self.param.theta3;
        }
        kernel
    }
    // カーネル行列の計算
    fn compute_kernel_matrix(&self, x_train: &DMatrix<f64>) -> DMatrix<f64> {
        let train_len = x_train.shape().0;
        let mut kernel_mat = DMatrix::zeros(train_len, train_len);
        for i in 0..train_len {
            for j in 0..train_len {
                // xi, xjに対し，RBFカーネルを計算する
                let xi = x_train.row(i).transpose();
                let xj = x_train.row(j).transpose();
                kernel_mat[(i, j)] = self.kernel(&xi, &xj, i, j);
            }
        }
        kernel_mat
    }
    // 対数尤度を計算する
    // 式: ln(p(y_test|theta)) = -ln|cov_n| - y_test^t * cov_n * y_test - ln(2pi)
    fn log_likelihood(&self, y_train: &DVector<f64>, kernel_mat: DMatrix<f64>) -> f64 {
        let det = kernel_mat.determinant().max(1e-100);
        -det.ln() - (y_train.transpose() * kernel_mat.lu().solve(y_train).unwrap())[(0, 0)]
    }
    // グリッドサーチの探索範囲を決める
    fn search_ranges(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let theta1_range = (3..10).map(|v| 2.0_f64.powi(v)).collect_vec();
        let theta2_range = (2..12).map(|v| 5.0 * v as f64).collect_vec();
        let theta3_range = (0..6).map(|v| 2.0_f64.powi(v)).collect_vec();
        (theta1_range, theta2_range, theta3_range)
    }
    // グリッドサーチで最適なtheta1~3を決定する
    pub fn grid_search(&mut self) {
        let x_train = DMatrix::from_row_slice(
            self.x.len(),
            self.x[0].len(),
            &self.x.iter().flatten().copied().collect_vec(),
        );
        let mut y_train = DVector::from_vec(self.y.clone());

        // 平均を引く
        let y_average = y_train.mean();
        y_train.add_scalar_mut(-y_average);

        let mut best_param = self.param;

        let kernel_mat = self.compute_kernel_matrix(&x_train);

        // 暫定の対数尤度を計算する
        // ln(p(y_test|theta)) = -ln|cov_n| - y_test^t * cov_n * y_test - ln(2pi)
        let mut best_likelihood = self.log_likelihood(&y_train, kernel_mat);

        // グリッドサーチで使う検索範囲を適当に決める
        let (theta1_range, theta2_range, theta3_range) = self.search_ranges();
        for &theta1 in &theta1_range {
            for &theta2 in &theta2_range {
                for &theta3 in &theta3_range {
                    self.param = GaussianProcessRegressionParameter::new(theta1, theta2, theta3);
                    let kernel_mat = self.compute_kernel_matrix(&x_train);
                    let likelihood = self.log_likelihood(&y_train, kernel_mat);
                    if best_likelihood < likelihood {
                        best_likelihood = likelihood;
                        best_param =
                            GaussianProcessRegressionParameter::new(theta1, theta2, theta3);
                    }
                }
            }
        }
        self.param = best_param;
    }
}
