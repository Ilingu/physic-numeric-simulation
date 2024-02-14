use rustfft::num_complex::Complex;

use crate::fft::XyScatter;

pub fn xy_scatter_to_fft(points: &XyScatter) -> Vec<Complex<f64>> {
    points
        .iter()
        .map(|(_, y)| Complex { re: *y, im: 0.0 })
        .collect()
}

pub fn norm(x: f64, y: f64) -> f64 {
    (x * x + y * y).sqrt()
}
