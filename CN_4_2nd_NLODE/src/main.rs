mod utils;

use std::{f64::consts::PI, format, println};

use plotters::prelude::*;

use ode_solvers::*;

use crate::utils::{rad_to_deg, search_period};

const X_START: f64 = 0.0;
const X_END_SIMUL: f64 = 10.0;

type Time = f64;

type Precision = f64;
type PenduleState = Vector2<Precision>;

struct Pendule {
    l: f64,
}

impl Pendule {
    fn new(l: f64) -> Self {
        Self { l }
    }
}

impl System<Precision, PenduleState> for Pendule {
    fn system(&self, _t: Time, y: &PenduleState, dy: &mut PenduleState) {
        dy[0] = y[1];
        dy[1] = -(9.80665 / self.l) * (y[0]).sin();
    }
}

struct PenduleFriction {
    l: f64,
    m: f64,
    f: f64,
}

impl PenduleFriction {
    fn new(l: f64, m: f64, f: f64) -> Self {
        Self { l, m, f }
    }
}

impl System<Precision, PenduleState> for PenduleFriction {
    fn system(&self, _t: Time, y: &PenduleState, dy: &mut PenduleState) {
        dy[0] = y[1];
        dy[1] = -(self.f / self.m) * y[1] - (9.80665 / self.l) * (y[0]).sin();
    }
}

fn vec_of_couple<T: PartialOrd>(couple: (Vec<T>, Vec<T>)) -> Vec<(T, T)> {
    couple.0.into_iter().zip(couple.1).collect()
}

fn draw(
    out: &str,
    caption: &str,
    (legend_x, legend_y): (&str, &str),
    (start, end): (f64, f64),
    lines: Vec<(Vec<f64>, Vec<f64>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(lines.len() <= 7);

    let (mut max_y, mut min_y) = (None::<f64>, None::<f64>);
    for (x_datas, y_datas) in lines.clone() {
        let sub_max_y = y_datas
            .clone()
            .into_iter()
            .reduce(|a, acc| a.max(acc))
            .ok_or("Failed to find the max height of the graph")?
            .ceil();
        let sub_min_y = y_datas
            .clone()
            .into_iter()
            .reduce(|a, acc| a.min(acc))
            .ok_or("Failed to find the min height of the graph")?
            .floor();
        max_y = Some(match max_y {
            Some(m) => m.max(sub_max_y),
            None => sub_max_y,
        });
        min_y = Some(match min_y {
            Some(m) => m.min(sub_min_y),
            None => sub_min_y,
        });
    }

    let root = BitMapBackend::new(out, (600, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (start as f32)..(end as f32),
            (min_y.unwrap().floor() as f32)..(max_y.unwrap().ceil() as f32),
        )?;

    chart
        .configure_mesh()
        .x_desc(legend_x)
        .y_desc(legend_y)
        .draw()?;

    const COLORS: [RGBColor; 7] = [BLACK, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA];
    for (i, (x_datas, y_datas)) in lines.into_iter().enumerate() {
        chart.draw_series(LineSeries::new(
            vec_of_couple::<f64>((x_datas, y_datas))
                .into_iter()
                .map(|(x, y)| (x as f32, y as f32)),
            &COLORS[i],
        ))?;
    }

    root.present()?;
    Ok(())
}

fn pendule(n: usize, l: f64, theta_0: f64, angular_0: f64) {
    let system = Pendule::new(l);

    let mut stepper = Dopri5::new(
        system,
        X_START,
        X_END_SIMUL,
        10_f64.powf(X_END_SIMUL.log10().round()) * 1.0 / n as f64,
        PenduleState::new(theta_0, angular_0),
        1.0e-10,
        1.0e-10,
    );
    let stats = stepper.integrate().expect("Failed to integrate");

    // Handle result
    println!("Pendule:\n{stats}\n\n");
    let (x_datas, y_datas) = (
        stepper.x_out().to_owned(),
        stepper.y_out().iter().map(|m| m[0]).collect::<Vec<_>>(),
    );

    draw(
        "./out/pendule.png",
        format!(
            "Sans frottement: l={l}m; θ(0)={}°; dθ/dt(0)={}°/s",
            rad_to_deg(theta_0),
            rad_to_deg(angular_0)
        )
        .as_str(),
        ("temps (s)", "θ (rad)"),
        (X_START, X_END_SIMUL),
        vec![(x_datas, y_datas)],
    )
    .expect("Failed to draw");
}

fn borda() {
    const L: f64 = 1.0;
    const N_SIMUL: usize = 100000;
    const N_SAMPLES: usize = 250;

    let (mut x_datas, mut y_datas) = (vec![], vec![]);
    for sample in 1..N_SAMPLES {
        let theta_0 = (PI / (N_SAMPLES as f64)) * (sample as f64);

        let system = Pendule::new(L);
        let mut stepper = Dopri5::new(
            system,
            X_START,
            X_END_SIMUL,
            10_f64.powf(X_END_SIMUL.log10().round()) * 1.0 / N_SIMUL as f64,
            PenduleState::new(theta_0, 0.0),
            1.0e-10,
            1.0e-10,
        );
        stepper.integrate().expect("Failed to integrate");
        let (xres, yres) = (
            stepper.x_out().to_owned(),
            stepper.y_out().iter().map(|m| m[0]).collect::<Vec<_>>(),
        );
        let period = search_period((&xres, &yres));
        x_datas.push(theta_0);
        y_datas.push(period);
    }

    let (mut x_borda, mut y_borda) = (vec![], vec![]);
    for sample in 0..=N_SAMPLES {
        let theta_0 = (PI / (N_SAMPLES as f64)) * (sample as f64);
        let period = 2.0
            * PI
            * (L / 9.80665).sqrt()
            * (1.0 + theta_0 * theta_0 / 16.0 + 11.0 * theta_0.powi(4) / 3072.0);

        x_borda.push(theta_0);
        y_borda.push(period);
    }

    draw(
        "./out/borda.png",
        "Borda vérification, l=1m",
        ("θ(0) (rad)", "Période (s)"),
        (X_START, 3.3),
        vec![(x_datas, y_datas), (x_borda, y_borda)],
    )
    .expect("Failed to draw");
}

fn pendule_friction(n: usize, l: f64, theta_0: f64, angular_0: f64, m: f64, f: f64) {
    let system = PenduleFriction::new(l, m, f);

    let mut stepper = Dopri5::new(
        system,
        X_START,
        X_END_SIMUL,
        10_f64.powf(X_END_SIMUL.log10().round()) * 1.0 / n as f64,
        PenduleState::new(theta_0, angular_0),
        1.0e-10,
        1.0e-10,
    );
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            println!("Pendule friction:\n{stats}\n\n");

            let (x_datas, y_datas) = (
                stepper.x_out().to_owned(),
                stepper.y_out().iter().map(|m| m[0]).collect::<Vec<_>>(),
            );
            draw(
                "./out/pendule_friction.png",
                format!(
                    "Avec frottement: l={l}m; m={m}kg; f={f}kg/s; θ(0)={}°; dθ/dt(0)={}°/s",
                    rad_to_deg(theta_0),
                    rad_to_deg(angular_0)
                )
                .as_str(),
                ("temps (s)", "θ (rad)"),
                (X_START, X_END_SIMUL),
                vec![(x_datas, y_datas)],
            )
            .expect("Failed to draw");
        }
        Err(e) => panic!("Failed to integrate: {e:?}"),
    }
}

fn main() {
    // pendule(1000, 0.1, PI / 4.0, 0.0);
    pendule_friction(1000, 1.0, PI / 2.0, PI / 2.0, 10.0, 1.0);
    // borda();
}
