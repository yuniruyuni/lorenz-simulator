pub use crate::diff_func::{DiffFunc};
pub use crate::simulator::{Simulator};

pub struct EularMethodSimulator<'a> {
    f: &'a dyn DiffFunc,

    x: f64,
    y: f64,
    // v = dy/dx = "v"elocity
    v: f64,

    dx: f64,
}

impl<'a> EularMethodSimulator<'a> {
    pub fn new(f: &'a dyn DiffFunc, dx: f64) -> Self {
        let (x, y, v) = f.initial();
        Self { f, x, y, v, dx }
    }
}

impl<'a> Simulator for EularMethodSimulator <'a> {
    fn estimated(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn run(&mut self) {
        let (x, y, v, dx) = (self.x, self.y, self.v, self.dx);
        self.v += self.f.dv_dx(x, y, v) * dx;
        self.y += self.f.dy_dx(x, y, v) * dx;
        self.x += dx;
    }
}