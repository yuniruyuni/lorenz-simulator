pub trait Simulator {
    /// return simulation result
    fn estimated(&self) -> (f64, f64);

    /// run a step for simulation
    fn run(&mut self);
}