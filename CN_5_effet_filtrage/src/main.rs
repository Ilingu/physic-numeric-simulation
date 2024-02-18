mod fft;
mod filtrage;
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
use filtrage::filtrage;

use crate::{
    fft::{compute_fft, evaluation_point, generate_signal, generate_triangle_wave, FFTResult},
    filtrage::draw_filtrage,
    filtre::{
        FiltrePasseBande, FiltrePasseBas1er, FiltrePasseBas2nd, FiltrePasseHaut1er,
        FiltrePasseHaut2nd, FiltreRejecteur, FiltreTrait,
    },
    plot::{draw_fft, draw_plot},
};

fn main() {
    filtrage_square_wave()
}

fn filtrage_triangle_wave() {
    const N: usize = 2_usize.pow(14) - 1;
    const WIDTH: Range<f64> = 0.0..0.002;
    let in_signal = generate_triangle_wave(1000.0, 0.0..2.0);
    let filtre = FiltrePasseBande::new(1.0, 25.0 * 1000.0, 0.2);
    let report = filtrage(&in_signal, &filtre, N, &WIDTH, &(0.0..50_000.0), None);
    draw_filtrage(&report);
}

fn filtrage_square_wave() {
    const N: usize = 2_usize.pow(14) - 1;
    const WIDTH: Range<f64> = 0.0..0.002;
    let in_signal = generate_square_wave(1000.0, -2.0..4.0);
    let filtre = FiltreRejecteur::new(1.0, 0.04 * 1000.0, 20.0);
    let report = filtrage(&in_signal, &filtre, N, &WIDTH, &(0.0..50_000.0), Some(1.0));
    draw_filtrage(&report);
}

fn filtrage_sinus_signal() {
    const N: usize = 2_usize.pow(14) - 1;
    const WIDTH: Range<f64> = 0.0..0.010;
    let in_signal = generate_signal(vec![(200.0, 1.0, 0.0), (1800.0, 1.0, 0.0)], None);
    let coupe_bande = FiltrePasseBande::new(1.0, 200.0, 20.0);
    let report = filtrage(&in_signal, &coupe_bande, N, &WIDTH, &(0.0..2000.0), None);
    draw_filtrage(&report);
}

fn plot_gain_phase() {
    const N: usize = 5000;
    const WIDTH: Range<f64> = 0.0..5_000.0;

    let passe_haut = FiltrePasseHaut2nd::new(1.0, 1000.0, 10.0);
    let ph_gpoints = passe_haut.gain_graph(N, &WIDTH);
    let ph_ppoints = passe_haut.phase_graph(N, &WIDTH);

    let passe_bas = FiltrePasseBas2nd::new(1.0, 1000.0, 10.0);
    let pb_gpoints = passe_bas.gain_graph(N, &WIDTH);
    let pb_ppoints = passe_bas.phase_graph(N, &WIDTH);

    let passe_bande = FiltrePasseBande::new(1.0, 1000.0, 10.0);
    let pbd_gpoints = passe_bande.gain_graph(N, &WIDTH);
    let pbd_ppoints = passe_bande.phase_graph(N, &WIDTH);

    let coupe_bande = FiltreRejecteur::new(1.0, 1000.0, 10.0);
    let cb_gpoints = coupe_bande.gain_graph(N, &WIDTH);
    let cb_ppoints = coupe_bande.phase_graph(N, &WIDTH);

    // gain
    draw_plot(
        "gain/passe_haut.png",
        "gain passe haut",
        WIDTH,
        ("f", "gain"),
        None,
        vec![&ph_gpoints],
        None,
    )
    .expect("Failed to plot");
    draw_plot(
        "gain/passe_bas.png",
        "gain passe bas",
        WIDTH,
        ("f", "gain"),
        None,
        vec![&pb_gpoints],
        None,
    )
    .expect("Failed to plot");
    draw_plot(
        "gain/passe_bande.png",
        "gain passe bande",
        WIDTH,
        ("f", "gain"),
        None,
        vec![&pbd_gpoints],
        None,
    )
    .expect("Failed to plot");
    draw_plot(
        "gain/coupe_bande.png",
        "gain coupe bande",
        WIDTH,
        ("f", "gain"),
        None,
        vec![&cb_gpoints],
        None,
    )
    .expect("Failed to plot");

    // phase
    draw_plot(
        "phase/passe_haut.png",
        "phase passe haut",
        WIDTH,
        ("f", "phase"),
        None,
        vec![&ph_ppoints],
        None,
    )
    .expect("Failed to plot");
    draw_plot(
        "phase/passe_bas.png",
        "phase passe bas",
        WIDTH,
        ("f", "phase"),
        None,
        vec![&pb_ppoints],
        None,
    )
    .expect("Failed to plot");
    draw_plot(
        "phase/passe_bande.png",
        "phase passe bande",
        WIDTH,
        ("f", "phase"),
        None,
        vec![&pbd_ppoints],
        None,
    )
    .expect("Failed to plot");
    draw_plot(
        "phase/coupe_bande.png",
        "phase coupe bande",
        WIDTH,
        ("f", "phase"),
        None,
        vec![&cb_ppoints],
        None,
    )
    .expect("Failed to plot");
}

fn fft_triangle_test() {
    const N: usize = 2_usize.pow(14) - 1;
    const WIDTH: Range<f64> = 0.0..10.0;

    let square = generate_triangle_wave(1.0, 0.0..1.0);
    let points = evaluation_point(N, &WIDTH, &square);

    // compute fft
    let FFTResult { peaks, .. } = compute_fft(&points, &WIDTH);

    // draw the datas
    draw_plot(
        "triangle_time_input.png",
        "Time input function",
        WIDTH,
        ("x", "y"),
        None,
        vec![&points],
        None,
    )
    .expect("Failed to plot");
    draw_fft(
        "triangle_fft_result.png",
        "Frequency response",
        0.0..100.0,
        &peaks,
    )
    .expect("Failed to plot");
}

fn fft_square_test() {
    const N: usize = 2_usize.pow(14) - 1;
    const WIDTH: Range<f64> = 0.0..10.0;

    let square = generate_square_wave(5.0, 0.0..1.0);
    let points = evaluation_point(N, &WIDTH, &square);

    // compute fft
    let FFTResult { peaks, .. } = compute_fft(&points, &WIDTH);

    // draw the datas
    draw_plot(
        "square_time_input.png",
        "Time input function",
        WIDTH,
        ("x", "y"),
        None,
        vec![&points],
        None,
    )
    .expect("Failed to plot");
    draw_fft(
        "square_fft_result.png",
        "Frequency response",
        0.0..100.0,
        &peaks,
    )
    .expect("Failed to plot");
}

fn fft_basic_test() {
    const N: usize = 2_usize.pow(14) - 1;
    const WIDTH: Range<f64> = 0.0..0.010;

    let sin = generate_signal(vec![(200.0, 1.0, 0.0), (1800.0, 1.0, -3.22)], None);
    let points = evaluation_point(N, &WIDTH, &sin);

    // compute fft
    let FFTResult { peaks, .. } = compute_fft(&points, &WIDTH);

    // draw the datas
    draw_plot(
        "sin_time_input.png",
        "Time input function",
        WIDTH,
        ("x", "y"),
        None,
        vec![&points],
        None,
    )
    .expect("Failed to plot");
    draw_fft(
        "sin_fft_result.png",
        "Frequency response",
        0.0..2000.0,
        &peaks,
    )
    .expect("Failed to plot");
}
