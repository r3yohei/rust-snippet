#[derive(Clone, Debug)]
struct Particle {
    pos: Vec2d,
    vel: Vec2d,
}
impl Particle {
    fn new(pos: Vec2d, vel: Vec2d) -> Self {
        Self { pos, vel }
    }
}

struct ParticleFilter {
    n: usize, // 粒子数
    particles: Vec<Particle>, // 状態表す粒子
    weights: Vec<f64>, // 重み
    R: f64, // 観測ノイズ
    Q_distr: Normal<f64>, // プロセスノイズを発生させる正規分布
    rng: Pcg64Mcg,
    create: fn( // 粒子生成関数
        usize, // 粒子数
        &Vec<f64>, // 平均
        &Vec<f64>, // 分散
        &mut Pcg64Mcg
    ) -> Vec<Particle>,
    hx: fn( // 観測関数
        &[Particle], // 粒子
        &[Vec2d], // ランドマーク
    ) -> Vec<f64>,
}
impl ParticleFilter {
    const EPS: f64 = 1e-9;
    fn new(n: usize, input: &Input) -> Self {
        Self {
            n,
            particles: vec![],
            weights: vec![],
            R: input.delta,
            Q_distr: Normal::new(0.0, input.eps).unwrap(),
            rng: Pcg64Mcg::new(SEED),
            create: gaussian_particles,
            hx: hx_default,
        }
    }
    fn create_particles(&mut self, mu: &Vec<f64>, sigma: &Vec<f64>) {
        // n個の粒子を初期の平均と標準偏差を使って初期化する
        self.particles = (self.create)(self.n, mu, sigma, &mut self.rng);
        // 重みは均等に1/n
        self.weights = vec![1.0 / self.n as f64; self.n];
    }
    fn update(&mut self, landmarks: &[Vec2d], z: f64) {
        // 各粒子の重みを更新する
        let z_hat = (self.hx)(&self.particles, landmarks);
        let mut weights_sum = 0.0;
        // 粒子数がself.nでなくなってもよいように，self.particles.len()でループする
        for i in 0..self.particles.len() {
            // 各粒子について，観測値がzである尤度を計算
            let p = self.likelihood(z, z_hat[i], 1.0, self.R);
            // 事前分布 * 尤度をやる
            self.weights[i] *= p;
            // 0除算を防ぐために極小の値を足す
            self.weights[i].chmax(Self::EPS);

            weights_sum += self.weights[i];
        }
        // 重みを正規化
        self.weights
            .iter_mut()
            .for_each(|w| *w /= weights_sum);
    }
    fn likelihood(&self, z: f64, z_hat: f64, mu: f64, sigma: f64) -> f64 {
        // 外で周辺化するので，正規化定数は省略
        (-0.5 * (z / z_hat.max(Self::EPS) - mu).powf(2.0) / sigma.powf(2.0)).exp()
    }
    fn estimate(&self) -> (Particle, Particle) {
        // 位置と速度の平均と標準偏差を返す
        let mut pos_mu = Vec2d::new(0.0, 0.0);
        let mut vel_mu = Vec2d::new(0.0, 0.0);
        for (i, particle) in self.particles.iter().enumerate() {
            pos_mu = pos_mu.add(particle.pos.mul_scalar(self.weights[i]));
            vel_mu = vel_mu.add(particle.vel.mul_scalar(self.weights[i]));
        }

        let mut pos_sigma = Vec2d::new(0.0, 0.0);
        let mut vel_sigma = Vec2d::new(0.0, 0.0);
        for (i, particle) in self.particles.iter().enumerate() {
            let pos_diff = particle.pos.sub(pos_mu);
            let vel_diff = particle.vel.sub(vel_mu);
            pos_sigma = pos_sigma.add(Vec2d::new(pos_diff.x.powf(2.0), pos_diff.y.powf(2.0)).mul_scalar(self.weights[i]));
            vel_sigma = vel_sigma.add(Vec2d::new(vel_diff.x.powf(2.0), vel_diff.y.powf(2.0)).mul_scalar(self.weights[i]));
        }
        pos_sigma = Vec2d::new(pos_sigma.x.sqrt(), pos_sigma.y.sqrt());
        vel_sigma = Vec2d::new(vel_sigma.x.sqrt(), vel_sigma.y.sqrt());

        (Particle::new(pos_mu, vel_mu), Particle::new(pos_sigma, vel_sigma))
    }
}

fn gaussian_particles(n: usize, mu: &Vec<f64>, sigma: &Vec<f64>, mut rng: &mut Pcg64Mcg) -> Vec<Particle> {
    let mut normal = vec![];
    for (m, s) in mu.iter().zip(sigma.iter()) {
        normal.push(Normal::new(*m, *s).unwrap());
    }
    let mut particles = vec![];
    for i in 0..n {
        let pos = Vec2d::new(normal[0].sample(&mut rng), normal[1].sample(&mut rng));
        let vel = Vec2d::new(normal[2].sample(&mut rng), normal[3].sample(&mut rng));
        particles.push(Particle::new(pos, vel));
    }

    particles
}

fn hx_default(particles: &[Particle], landmarks: &[Vec2d]) -> Vec<f64> {
    let mut hx = vec![];
    for (particle, landmark) in particles.iter().zip(landmarks.iter()) {
        hx.push(particle.pos.dist(*landmark));
    }

    hx
}