use std::ops::Range;

use crate::{
    fft::{compute_fft, evaluation_point, generate_signal, FFTResult, XyScatter},
    filtre::{FiltreCaracteristique, FiltreTrait},
    plot::draw_plot,
};

pub struct FiltrageReport {
    pub signal_input: XyScatter,
    pub signal_ouput: XyScatter,
    pub fft_input: XyScatter,
    pub fft_output: XyScatter,
    pub gain_graph: XyScatter,
    pub phase_graph: XyScatter,
    pub filtre_caracteristique: FiltreCaracteristique,
    pub width: Range<f64>,
}

pub fn filtrage<T: FiltreTrait>(
    signal: &dyn Fn(f64) -> f64,
    filtre: &T,
    n: usize,
    width: &Range<f64>,
) -> FiltrageReport {
    let signal_in = evaluation_point(n, width, signal);
    let FFTResult {
        fft_amp: fft_in,
        peaks,
        ..
    } = compute_fft(&signal_in, width);

    let out_signal_info = peaks
        .into_iter()
        .map(|(_, (f, amp))| {
            let amp_out = amp * filtre.gain_at(f);
            let phase_out = filtre.phase_at(f);
            (f, amp_out, phase_out)
        })
        .collect::<Vec<_>>();

    let signal_out_func = generate_signal(out_signal_info);
    let signal_out = evaluation_point(n, width, &signal_out_func);
    let fft_res_out = compute_fft(&signal_out, width);

    let gain_graph = filtre.gain_graph(n, &(0.0..2_000.0)); // range should be the same as the frequence max
    let phase_graph = filtre.phase_graph(n, &(0.0..2_000.0));

    FiltrageReport {
        signal_input: signal_in,
        signal_ouput: signal_out,
        fft_input: fft_in,
        fft_output: fft_res_out.fft_amp,
        gain_graph,
        phase_graph,
        filtre_caracteristique: filtre.get_caracteristique(),
        width: width.clone(),
    }
}

pub fn draw_filtrage(report: &FiltrageReport) {
    draw_plot(
        "filtrage/signals.png",
        &format!(
            "Effet filtre {:?} avec ω0={}rad/s Q={}",
            report.filtre_caracteristique.filtre_type,
            report.filtre_caracteristique.omega0.round(),
            report.filtre_caracteristique.q,
        ),
        report.width.clone(),
        ("t (s)", ""),
        None,
        vec![&report.signal_input, &report.signal_ouput],
    )
    .expect("Failed to plot");
    draw_plot(
        "filtrage/fft_in.png",
        "Spectre signal d'entré",
        0.0..2000.0,
        ("frequency", "amplitude"),
        Some((600, 300)),
        vec![&report.fft_input],
    )
    .expect("Failed to plot");
    draw_plot(
        "filtrage/fft_out.png",
        "Spectre signal de sortie",
        0.0..2000.0,
        ("frequency", "amplitude"),
        Some((600, 300)),
        vec![&report.fft_output],
    )
    .expect("Failed to plot");
    draw_plot(
        "filtrage/gain.png",
        &format!(
            "Gain du filtre {:?} avec ω0={}rad/s Q={}",
            report.filtre_caracteristique.filtre_type,
            report.filtre_caracteristique.omega0.round(),
            report.filtre_caracteristique.q,
        ),
        0.0..20_000.0,
        ("frequency", "gain"),
        None,
        vec![&report.gain_graph],
    )
    .expect("Failed to plot");
}
