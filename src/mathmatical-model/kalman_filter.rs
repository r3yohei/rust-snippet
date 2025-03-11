use nalgebra::*;

#[derive(Clone, Debug)]
struct KalmanFilter {
    x: Vector4<f64>,   // 状態
    P: Matrix4<f64>,   // 事前分布の共分散行列
    F: Matrix4<f64>,   // 状態遷移行列
    B: Matrix4x2<f64>, // 制御関数
    Q: Matrix4<f64>,   // プロセスノイズ行列
    H: Matrix2x4<f64>, // 観測関数
}
impl KalmanFilter {
    fn new() -> Self {
        // 事前分布の平均: 与えられた初期位置と初期速度0
        let x = Vector4::new(0.0, 0.0, 0.0, 0.0);
        // 事前分布の分散: 初期位置と初期速度は確定しているので対応する分散は0
        let P = Matrix4::zeros();
        // 次時刻の事前分布の平均の予測
        // x_hat = F * x + B * u
        let F = Matrix4::new(
            1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        );
        let B = Matrix4x2::new(1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0);
        // 系が受けるノイズ
        let Q = Matrix4::new(
            0.0, 1e-3, 0.0, 0.0, 1e-3, 1e-5, 0.0, 0.0, 0.0, 0.0, 0.0, 1e-3, 0.0, 0.0, 1e-3, 1e-5,
        );
        // 観測値z，状態x_hatに対する残差y
        // y = z - H * x_hat
        // 例:
        // |x_diff| = |x_ob| - |1 0 0 0||x_hat |
        // |y_diff|   |y_ob|   |0 0 1 0||vx_hat|
        //                              |y_hat |
        //                              |vy_hat|
        // H: 観測関数
        let H = Matrix2x4::new(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

        Self { x, P, F, B, Q, H }
    }
    fn predict(&mut self, u: (f64, f64)) {
        // 次時刻の事前分布の予測
        // 事前分布の平均
        self.x = self.F * self.x + self.B * Vector2::new(u.0, u.1);

        // 事前分布の共分散行列
        self.P = self.F * self.P * self.F.transpose() + self.Q;
    }
    fn update(&mut self, mu: f64, sigma2: f64) {
        // 更新
        // 事前分布*尤度に相当する操作を行う
        // 観測ノイズ行列を，計測の結果の分散から定義
        let R = Matrix2::new(sigma2, 0.0, 0.0, sigma2);
        let S = self.H * self.P * self.H.transpose() + R;
        let K = self.P * self.H.transpose() * S.try_inverse().unwrap();
        let z = Matrix2::new(mu, 0.0, 0.0, mu);
        let y = z - self.H * self.x;
        self.x += K * y;

        // 数値的にロバストなPの更新 (必ず対称行列になる)
        self.P = (Matrix4::identity() - K * self.H)
            * self.P
            * (Matrix4::identity() - K * self.H).transpose()
            + K * R * K.transpose();
    }
}
