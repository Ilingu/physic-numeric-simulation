mod fft;
mod plot;
mod utils;

use std::ops::Range;

use crate::{
    fft::{compute_fft, evaluation_point, extract_peaks, generate_signals},
    plot::draw_plot,
    utils::xy_scatter_to_fft,
};

fn main() {
    #[allow(clippy::eq_op)]
    const N: usize = 2_usize.pow(12);
    const WIDTH: Range<f64> = 0.0..10.0;

    let sin = generate_signals(vec![(2.0, 2.0), (0.5, 0.5), (1.0, 9.0)]);
    let points = evaluation_point(N, WIDTH, &sin);
    let complex_points = xy_scatter_to_fft(&points);
    let fft_points = compute_fft(complex_points, WIDTH);

    let peaks = extract_peaks(&fft_points);
    println!("{peaks:?}");

    draw_plot(
        "time_input.png",
        "Time input function",
        WIDTH,
        ("x", "y"),
        vec![&points],
    )
    .expect("Failed to plot");
    draw_plot(
        "fft_result.png",
        "Frequency response",
        0.0..10.0,
        ("frequency", "amplitude"),
        vec![&fft_points],
    )
    .expect("Failed to plot");
    // let max = amplitude
    //     .iter()
    //     .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
    //     .unwrap();
    // println!("{max:?}");
}
