pub use crate::simulator::{Simulator};

pub struct LK4Simulator {
    x: f64,
    y: f64,
    dx: f64,

    // v = dy/dx = "v"elocity
    v: f64,
}

impl LK4Simulator {
    pub fn new(x: f64, y: f64, dx: f64) -> Self {
        Self { x, y, dx, v: 0.0 }
    }

    fn dv_dx(&self, _x: f64, cy: f64, _cv: f64) -> f64 {
        let k = 1.0f64;
        -k * cy
    }

    fn dy_dx(&self, _x: f64, _cy: f64, cv: f64) -> f64{
        cv
    }
}

impl Simulator for LK4Simulator {
    fn estimated(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn run(&mut self) {
        let (x, y, v, dx) = (self.x, self.y, self.v, self.dx);
        let half_dx = dx * 0.5;

        let v_k1 = self.dv_dx(x, y, v);
        let y_k1 = self.dy_dx(x, y, v);

        let v_k2 = self.dv_dx(x + half_dx, y + half_dx*y_k1, v + half_dx*v_k1);
        let y_k2 = self.dy_dx(x + half_dx, y + half_dx*y_k1, v + half_dx*v_k1);

        let v_k3 = self.dv_dx(x + half_dx, y + half_dx*y_k2, v + half_dx*v_k2);
        let y_k3 = self.dy_dx(x + half_dx, y + half_dx*y_k2, v + half_dx*v_k2);

        let v_k4 = self.dv_dx(x + dx, y + dx*y_k3, v + dx*v_k3);
        let y_k4 = self.dy_dx(x + dx, y + dx*y_k3, v + dx*v_k3);

        self.v += (dx/6.0) * (v_k1 + 2.0*v_k2 + 2.0*v_k3 + v_k4);
        self.y += (dx/6.0) * (y_k1 + 2.0*y_k2 + 2.0*y_k3 + y_k4);
        self.x += dx;
    }
}