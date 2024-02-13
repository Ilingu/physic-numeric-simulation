use std::{f64::consts::PI, ops::Range};

use find_peaks::PeakFinder;
use rustfft::{num_complex::Complex, FftPlanner};

pub type XyScatter = Vec<(f64, f64)>;

pub fn evaluation_point(n: usize, width: Range<f64>, f: &dyn Fn(f64) -> f64) -> XyScatter {
    let mut points: XyScatter = vec![];
    let step = (width.end - width.start) / (n as f64);
    for i in 0..=n {
        let x = width.start + (i as f64) * step;
        let y = f(x);
        points.push((x, y));
    }
    points
}

pub fn generate_sinus(amp: f64, f: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| amp * (2.0 * PI * f * x).sin())
}

/// signals: Vec<(amplitude, frequency)>
pub fn generate_signals(signals: Vec<(f64, f64)>) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| {
        signals
            .iter()
            .map(|(amp, f)| amp * (2.0 * PI * f * x).sin())
            .sum::<f64>()
    })
}

pub fn compute_fft(fft_points: Vec<Complex<f64>>, width: Range<f64>) -> XyScatter {
    let mut planner = FftPlanner::<f64>::new();
    let sampling_nb = fft_points.len(); // N
    let fft = planner.plan_fft_forward(sampling_nb);

    let mut buffer = fft_points;
    fft.process(&mut buffer);

    let signal_window_size = width.end - width.start;
    buffer
        .into_iter()
        .take(sampling_nb / 2 - 1)
        .enumerate()
        .map(|(i, c)| {
            (
                (i as f64) / signal_window_size,
                2.0 * c.norm() / (sampling_nb as f64),
            )
        })
        .collect::<XyScatter>()
}

pub fn extract_peaks(fft_points: &XyScatter) -> XyScatter {
    let only_amp = fft_points.iter().map(|(_, y)| *y).collect::<Vec<f64>>();
    let fp = PeakFinder::new(&only_amp);

    let peaks = fp.find_peaks();
    peaks
        .into_iter()
        .map(|p| fft_points[p.middle_position()])
        .collect()
}
