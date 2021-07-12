mod diff_func;
mod simulator;
mod eular;
mod lk4;

use crate::diff_func::{DiffFunc};
use crate::simulator::{Simulator};
// use crate::eular::{EularMethodSimulator};
use crate::lk4::{LK4Simulator};

struct Cosine { }

impl DiffFunc for Cosine {
    fn initial(&self) -> (f64, f64, f64) {
        (0.0, 1.0, 0.0)
    }

    fn dv_dx(&self, _x: f64, y: f64, _v: f64) -> f64 {
        let k = 1.0f64;
        -k * y
    }

    fn dy_dx(&self, _x: f64, _y: f64, v: f64) -> f64{
        v
    }
}

pub fn run<T: Simulator>(sim: &mut T, steps: usize, out_steps: usize) {
    for i in 0..steps {
        if i % out_steps == 0 {
            let (x, y) = sim.estimated();
            println!("{} {}", x, y);
        }
        sim.run();
    }
}

fn main() {
    let cos = Cosine {};
    // let mut sim = EularMethodSimulator::new(cos, 0.0001);
    let mut sim = LK4Simulator::new(&cos, 0.0001);
    run(&mut sim, 100000, 1);
}
