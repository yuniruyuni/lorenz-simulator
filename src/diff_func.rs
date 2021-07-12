pub trait DiffFunc {
    fn initial(&self) -> (f64, f64, f64);
    fn dv_dx(&self, x: f64, y: f64, v: f64) -> f64;
    fn dy_dx(&self, x: f64, y: f64, v: f64) -> f64;
}