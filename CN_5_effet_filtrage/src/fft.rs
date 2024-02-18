use std::{f64::consts::PI, ops::Range};

use find_peaks::PeakFinder;
use rustfft::FftPlanner;

use crate::utils::xy_scatter_to_fft;

pub type XyScatter = Vec<(f64, f64)>;

pub fn evaluation_point(n: usize, width: &Range<f64>, f: &dyn Fn(f64) -> f64) -> XyScatter {
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

/// signals: Vec<(frequency, amplitude, initial phase)>
pub fn generate_signal(
    signal_info: Vec<(f64, f64, f64)>,
    offset: Option<f64>,
) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| {
        signal_info
            .iter()
            .map(|(f, amp, phase)| amp * (2.0 * PI * f * x + phase).sin())
            .sum::<f64>()
            + offset.unwrap_or(0.0)
    })
}

pub fn generate_square_wave(f: f64, minmax: Range<f64>) -> Box<dyn Fn(f64) -> f64> {
    let period = 1.0 / f;
    Box::new(move |x| {
        if x % period < period / 2.0 {
            minmax.end
        } else {
            minmax.start
        }
    })
}

pub fn generate_triangle_wave(f: f64, minmax: Range<f64>) -> Box<dyn Fn(f64) -> f64> {
    let period = 1.0 / f;
    Box::new(move |x| {
        let t = x % period;
        if t < period / 2.0 {
            ((2.0 * (minmax.end - minmax.start)) / period) * t + minmax.start
        } else {
            ((2.0 * (minmax.start - minmax.end)) / period) * t + 2.0 * minmax.end - minmax.start
        }
    })
}

/// returns: (index_in_original_array, (f, amp))
fn extract_peaks(fft_points: &XyScatter) -> XyScatter {
    let only_amp = fft_points.iter().map(|(_, y)| *y).collect::<Vec<f64>>();
    let fp = PeakFinder::new(&only_amp);

    let peaks = fp.find_peaks();
    peaks
        .into_iter()
        .map(|p| {
            let i = p.middle_position();
            let fftp = fft_points[i];
            fftp
        })
        .collect()
}

pub struct FFTResult {
    /// raw data, see peaks for postprocessed datas
    pub fft_amp: XyScatter,
    pub fft_phase: XyScatter,
    /// "real" fft result: Vec<(f, amp)>
    pub peaks: XyScatter,
}

pub fn compute_fft(points: &XyScatter, width: &Range<f64>) -> FFTResult {
    let complex_points = xy_scatter_to_fft(points);

    let mut planner = FftPlanner::<f64>::new();
    let sampling_nb = complex_points.len(); // N

    let fft = planner.plan_fft_forward(sampling_nb);

    let mut buffer = complex_points;
    fft.process(&mut buffer);

    let signal_window_size = width.end - width.start;
    let fft_points = buffer
        .into_iter()
        .take(sampling_nb / 2 - 1)
        .enumerate()
        .map(|(i, c)| {
            (
                (i as f64) / signal_window_size,
                2.0 * c.norm() / (sampling_nb as f64),
                (c.im / c.re).atan(),
            )
        })
        .collect::<Vec<_>>();
    let fft_amp = fft_points
        .iter()
        .map(|(f, amp, _)| (*f, *amp))
        .collect::<XyScatter>();
    let fft_phase = fft_points
        .iter()
        .map(|(f, _, phase)| (*f, *phase))
        .collect::<XyScatter>();

    let peaks_amp = extract_peaks(&fft_amp);
    FFTResult {
        fft_amp,
        fft_phase,
        peaks: peaks_amp,
    }
}
