pub use crate::simulator::{Simulator};

pub struct EularMethodSimulator {
    x: f64,
    y: f64,
    dx: f64,

    // v = dy/dx = "v"elocity
    v: f64,
}

impl EularMethodSimulator {
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

impl Simulator for EularMethodSimulator {
    fn estimated(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn run(&mut self) {
        let (x, y, v, dx) = (self.x, self.y, self.v, self.dx);
        self.v += self.dv_dx(x, y, v) * dx;
        self.y += self.dy_dx(x, y, v) * dx;
        self.x += dx;
    }
}