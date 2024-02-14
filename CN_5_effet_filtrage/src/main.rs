mod fft;
mod filtre;
mod plot;
mod utils;

/*
https://docs.rs/rustfft/latest/rustfft/
https://howthefouriertransformworks.com/understanding-the-output-of-an-fft/
https://phip1611.de/blog/frequency-spectrum-analysis-with-fft-in-rust/
*/

use std::ops::Range;

use fft::generate_square_wave;

use crate::{
    fft::{compute_fft, evaluation_point, generate_signals, generate_triangle_wave},
    filtre::{FiltrePasseBande, FiltrePasseBas, FiltrePasseHaut, FiltreRejecteur, FiltreTrait},
    plot::draw_plot,
};

fn main() {
    plot_gain_phase()
}

fn plot_gain_phase() {
    const N: usize = 5000;
    const WIDTH: Range<f64> = 0.0..20_000.0;

    let passe_haut = FiltrePasseHaut::new(1.0, 1000.0, 10.0);
    let ph_gpoints = passe_haut.gain_graph(N, WIDTH);
    let ph_ppoints = passe_haut.phase_graph(N, WIDTH);

    let passe_bas = FiltrePasseBas::new(1.0, 1000.0, 10.0);
    let pb_gpoints = passe_bas.gain_graph(N, WIDTH);
    let pb_ppoints = passe_bas.phase_graph(N, WIDTH);

    let passe_bande = FiltrePasseBande::new(1.0, 1000.0, 10.0);
    let pbd_gpoints = passe_bande.gain_graph(N, WIDTH);
    let pbd_ppoints = passe_bande.phase_graph(N, WIDTH);

    let coupe_bande = FiltreRejecteur::new(1.0, 1000.0, 10.0);
    let cb_gpoints = coupe_bande.gain_graph(N, WIDTH);
    let cb_ppoints = coupe_bande.phase_graph(N, WIDTH);

    // gain
    draw_plot(
        "gain/passe_haut.png",
        "gain passe haut",
        WIDTH,
        ("f", "gain"),
        vec![&ph_gpoints],
    )
    .expect("Failed to plot");
    draw_plot(
        "gain/passe_bas.png",
        "gain passe bas",
        WIDTH,
        ("f", "gain"),
        vec![&pb_gpoints],
    )
    .expect("Failed to plot");
    draw_plot(
        "gain/passe_bande.png",
        "gain passe bande",
        WIDTH,
        ("f", "gain"),
        vec![&pbd_gpoints],
    )
    .expect("Failed to plot");
    draw_plot(
        "gain/coupe_bande.png",
        "gain coupe bande",
        WIDTH,
        ("f", "gain"),
        vec![&cb_gpoints],
    )
    .expect("Failed to plot");

    // phase
    draw_plot(
        "phase/passe_haut.png",
        "phase passe haut",
        WIDTH,
        ("f", "phase"),
        vec![&ph_ppoints],
    )
    .expect("Failed to plot");
    draw_plot(
        "phase/passe_bas.png",
        "phase passe bas",
        WIDTH,
        ("f", "phase"),
        vec![&pb_ppoints],
    )
    .expect("Failed to plot");
    draw_plot(
        "phase/passe_bande.png",
        "phase passe bande",
        WIDTH,
        ("f", "phase"),
        vec![&pbd_ppoints],
    )
    .expect("Failed to plot");
    draw_plot(
        "phase/coupe_bande.png",
        "phase coupe bande",
        WIDTH,
        ("f", "phase"),
        vec![&cb_ppoints],
    )
    .expect("Failed to plot");
}

fn fft_triangle_test() {
    const N: usize = 2_usize.pow(14) - 1;
    const WIDTH: Range<f64> = 0.0..10.0;

    let square = generate_triangle_wave(1.0, 0.0..1.0);
    let points = evaluation_point(N, WIDTH, &square);

    // compute fft
    let fft_points = compute_fft(&points, WIDTH, true);

    // draw the datas
    draw_plot(
        "triangle_time_input.png",
        "Time input function",
        WIDTH,
        ("x", "y"),
        vec![&points],
    )
    .expect("Failed to plot");
    draw_plot(
        "triangle_fft_result.png",
        "Frequency response",
        0.0..100.0,
        ("frequency", "amplitude"),
        vec![&fft_points],
    )
    .expect("Failed to plot");
}

fn fft_square_test() {
    const N: usize = 2_usize.pow(14) - 1;
    const WIDTH: Range<f64> = 0.0..10.0;

    let square = generate_square_wave(5.0, 0.0..1.0);
    let points = evaluation_point(N, WIDTH, &square);

    // compute fft
    let fft_points = compute_fft(&points, WIDTH, true);

    // draw the datas
    draw_plot(
        "square_time_input.png",
        "Time input function",
        WIDTH,
        ("x", "y"),
        vec![&points],
    )
    .expect("Failed to plot");
    draw_plot(
        "square_fft_result.png",
        "Frequency response",
        0.0..100.0,
        ("frequency", "amplitude"),
        vec![&fft_points],
    )
    .expect("Failed to plot");
}

fn fft_basic_test() {
    const N: usize = 2_usize.pow(14) - 1;
    const WIDTH: Range<f64> = 0.0..10.0;

    let sin = generate_signals(vec![(2.0, 2.0), (0.5, 0.5), (1.0, 9.0)]);
    let points = evaluation_point(N, WIDTH, &sin);

    // compute fft
    let fft_points = compute_fft(&points, WIDTH, true);

    // draw the datas
    draw_plot(
        "sin_time_input.png",
        "Time input function",
        WIDTH,
        ("x", "y"),
        vec![&points],
    )
    .expect("Failed to plot");
    draw_plot(
        "sin_fft_result.png",
        "Frequency response",
        0.0..10.0,
        ("frequency", "amplitude"),
        vec![&fft_points],
    )
    .expect("Failed to plot");
}
