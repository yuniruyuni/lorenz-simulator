mod simulator;
mod eular;
mod lk4;

use crate::simulator::{Simulator};
// use crate::eular::{EularMethodSimulator};
use crate::lk4::{LK4Simulator};

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
    // let mut sim = EularMethodSimulator::new(0.0, 1.0, 0.0001);
    let mut sim = LK4Simulator::new(0.0, 1.0, 0.0001);
    run(&mut sim, 100000, 1);
}
