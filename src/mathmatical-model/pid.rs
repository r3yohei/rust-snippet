struct PID {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: Vec2d,
    prev_error: Vec2d,
}
impl PID {
    fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self { kp, ki, kd, integral: Vec2d::new(0.0, 0.0), prev_error: Vec2d::new(0.0, 0.0) }
    }
    fn calc_manipulated_variable(&mut self, crt: Vec2d, destination: Vec2d) -> Vec2d {
        let error = destination.sub(crt);
        self.integral = self.integral.add(error);
        let derivative = error.sub(self.prev_error);
        self.prev_error = error;

        let m = error.mul_scalar(self.kp)
            .add(self.integral.mul_scalar(self.ki))
            .add(derivative.mul_scalar(self.kd));
        
        m
    }
}
