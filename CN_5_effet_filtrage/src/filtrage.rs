use std::{f64::consts::PI, ops::Range};

use crate::{
    fft::{compute_fft, evaluation_point, generate_signal, FFTResult, XyScatter},
    filtre::{FiltreCaracteristique, FiltreTrait},
    plot::{draw_fft, draw_plot},
};

pub struct FiltrageReport {
    pub signal_input: XyScatter,
    pub signal_ouput: XyScatter,
    pub fft_input: XyScatter,
    pub fft_output: XyScatter,
    pub gain_graph: XyScatter,
    pub phase_graph: XyScatter,
    pub filtre_caracteristique: FiltreCaracteristique,
    pub width_graph: Range<f64>,
    pub width_frequence: Range<f64>,
}

pub fn filtrage<T: FiltreTrait>(
    signal: &dyn Fn(f64) -> f64,
    filtre: &T,
    n: usize,
    width_graph: &Range<f64>,
    width_frequence: &Range<f64>,
    offset: Option<f64>,
) -> FiltrageReport {
    let signal_in = evaluation_point(n, width_graph, signal);
    let FFTResult { peaks: fft_in, .. } = compute_fft(&signal_in, width_graph);

    let out_signal_info = fft_in
        .iter()
        .map(|&(f, amp)| {
            let amp_out = amp * filtre.gain_at(f);
            let phase_out = filtre.phase_at(f);
            (f, amp_out, phase_out)
        })
        .collect::<Vec<_>>();

    let signal_out_func = generate_signal(out_signal_info, offset);
    let signal_out = evaluation_point(n, width_graph, &signal_out_func);
    let fft_res_out = compute_fft(&signal_out, width_graph);

    let gain_graph = filtre.gain_graph(n, width_frequence);
    let phase_graph = filtre.phase_graph(n, width_frequence);

    FiltrageReport {
        signal_input: signal_in,
        signal_ouput: signal_out,
        fft_input: fft_in,
        fft_output: fft_res_out.peaks,
        gain_graph,
        phase_graph,
        filtre_caracteristique: filtre.get_caracteristique(),
        width_graph: width_graph.clone(),
        width_frequence: width_frequence.clone(),
    }
}

pub fn draw_filtrage(report: &FiltrageReport) {
    draw_plot(
        "filtrage/signals.png",
        &format!(
            "Effet filtre {:?} avec f0={}Hz Q={}",
            report.filtre_caracteristique.filtre_type,
            (report.filtre_caracteristique.omega0 / (2.0 * PI)).round(),
            report.filtre_caracteristique.q,
        ),
        report.width_graph.clone(),
        ("t (s)", ""),
        None,
        vec![&report.signal_input, &report.signal_ouput],
    )
    .expect("Failed to plot");
    draw_fft(
        "filtrage/fft_in.png",
        "Spectre signal d'entré",
        report.width_frequence.clone(),
        &report.fft_input,
    )
    .expect("Failed to plot");
    draw_fft(
        "filtrage/fft_out.png",
        "Spectre signal de sortie",
        report.width_frequence.clone(),
        &report.fft_output,
    )
    .expect("Failed to plot");
    draw_plot(
        "filtrage/gain.png",
        &format!(
            "Gain du filtre {:?} avec f0={}Hz Q={}",
            report.filtre_caracteristique.filtre_type,
            (report.filtre_caracteristique.omega0 / (2.0 * PI)).round(),
            report.filtre_caracteristique.q,
        ),
        report.width_frequence.clone(),
        ("frequency", "gain"),
        Some((600, 300)),
        vec![&report.gain_graph],
    )
    .expect("Failed to plot");
    draw_plot(
        "filtrage/phase.png",
        &format!(
            "Phase du filtre {:?} avec f0={}Hz Q={}",
            report.filtre_caracteristique.filtre_type,
            (report.filtre_caracteristique.omega0 / (2.0 * PI)).round(),
            report.filtre_caracteristique.q,
        ),
        report.width_frequence.clone(),
        ("frequency", "phase"),
        Some((600, 300)),
        vec![&report.phase_graph],
    )
    .expect("Failed to plot");
}
