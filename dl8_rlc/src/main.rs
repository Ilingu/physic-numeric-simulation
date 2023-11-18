use std::{format, println};

use plotters::prelude::*;

use ode_solvers::*;

const X_START: f64 = 0.0;
const X_END: f64 = 0.001;

type RCState = Vector1<f64>;

struct RC {
    e: f64,
    rc: f64,
}

impl System<RCState> for RC {
    fn system(&self, _x: Time, y: &RCState, dy: &mut RCState) {
        dy[0] = self.e - y[0] / self.rc
    }
}

type RLCSinusState = Vector2<f64>;

struct RLCSinus {
    w0: f64,
    q: f64,
    e1: f64,
    e2: f64,
    pulse: f64,
}

impl RLCSinus {
    fn new(e1: f64, e2: f64, pulse: f64, r: f64, c: f64, l: f64) -> Self {
        Self {
            w0: 1.0 / (l * c).sqrt(),
            q: (r / 2.0) * (c / l).sqrt(),
            e1,
            e2,
            pulse,
        }
    }
}

impl System<RLCSinusState> for RLCSinus {
    fn system(&self, t: Time, y: &RLCSinusState, dy: &mut RLCSinusState) {
        let w0_square = (self.w0) * (self.w0);
        let e = self.e1 + self.e2 * (self.pulse * t).sin();
        dy[0] = y[1];
        dy[1] = w0_square * e - 2.0 * (self.q) * (self.w0) * y[1] - w0_square * y[0];
    }
}

type RLCState = Vector2<f64>;

struct RLC {
    w0: f64,
    q: f64,
    e: f64,
}

impl RLC {
    fn new(e: f64, r: f64, c: f64, l: f64) -> Self {
        Self {
            w0: 1.0 / (l * c).sqrt(),
            q: (r / 2.0) * (c / l).sqrt(),
            e,
        }
    }
}

impl System<RLCState> for RLC {
    fn system(&self, _x: Time, y: &RLCState, dy: &mut RLCState) {
        let w0_square = (self.w0) * (self.w0);
        dy[0] = y[1];
        dy[1] = w0_square * (self.e) - 2.0 * (self.q) * (self.w0) * y[1] - w0_square * y[0];
    }
}

type Time = f64;

fn vec_of_couple<T: PartialOrd>(couple: (Vec<T>, Vec<T>)) -> Vec<(T, T)> {
    couple.0.into_iter().zip(couple.1.into_iter()).collect()
}

fn draw(
    out: &str,
    caption: &str,
    (x_datas, y_datas): (Vec<f64>, Vec<f64>),
) -> Result<(), Box<dyn std::error::Error>> {
    let max_y = y_datas
        .clone()
        .into_iter()
        .reduce(|a, acc| a.max(acc))
        .ok_or("Failed to find the max height of the graph")?
        .ceil();
    let min_y = y_datas
        .clone()
        .into_iter()
        .reduce(|a, acc| a.min(acc))
        .ok_or("Failed to find the min height of the graph")?
        .ceil();

    let root = BitMapBackend::new(out, (600, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (X_START as f32)..(X_END as f32),
            (min_y.floor() as f32)..(max_y.ceil() as f32),
        )?;

    chart
        .configure_mesh()
        .y_desc("tension (V)")
        .x_desc("temps (s)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        vec_of_couple::<f64>((x_datas, y_datas))
            .into_iter()
            .map(|(x, y)| (x as f32, y as f32)),
        &MAGENTA,
    ))?;

    root.present()?;
    Ok(())
}

fn rc4_1(n: usize) {
    let system = RC { e: 1.0, rc: 1.0 };

    let mut stepper = Dopri5::new(
        system,
        X_START,
        X_END,
        1.0 / n as f64,
        RCState::new(0.0),
        1.0e-10,
        1.0e-10,
    );
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            println!("RC:\n{stats}\n\n");

            let (x_datas, y_datas) = (
                stepper.x_out(),
                stepper
                    .y_out()
                    .into_iter()
                    .map(|m| m[0])
                    .collect::<Vec<_>>(),
            );
            println!("{x_datas:?}");
            draw("./out/rc.png", "4.1 rc", (x_datas.clone(), y_datas)).expect("Failed to draw");
        }
        Err(_) => panic!("Failed to integrate"),
    }
}

fn rlc4_2(n: usize, r: f64) {
    let system = RLC::new(1.0, r, 1.0e-7, 0.001);

    let mut stepper = Dopri5::new(
        system,
        X_START,
        X_END,
        10_f64.powf(X_END.log10().round()) * 1.0 / n as f64,
        RLCState::new(0.0, 0.0),
        1.0e-10,
        1.0e-10,
    );
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            println!("RLC:\n{stats}\n\n");

            let (x_datas, y_datas) = (
                stepper.x_out(),
                stepper
                    .y_out()
                    .into_iter()
                    .map(|m| m[0])
                    .collect::<Vec<_>>(),
            );
            draw(
                "./out/rlc.png",
                format!("4.2 rlc, R={r}Ω").as_str(),
                (x_datas.clone(), y_datas),
            )
            .expect("Failed to draw");
        }
        Err(e) => panic!("Failed to integrate: {e:?}"),
    }
}

fn rlc_sinus4_3(n: usize, pulse: f64) {
    let system = RLCSinus::new(3.0, 2.0, pulse, 25.0, 1.0e-7, 0.001);

    let mut stepper = Dopri5::new(
        system,
        X_START,
        X_END,
        10_f64.powf(X_END.log10().round()) * 1.0 / n as f64,
        RLCSinusState::new(0.0, 0.0),
        1.0e-10,
        1.0e-10,
    );
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            println!("RLC Sinus:\n{stats}\n\n");

            let (x_datas, y_datas) = (
                stepper.x_out(),
                stepper
                    .y_out()
                    .into_iter()
                    .map(|m| m[0])
                    .collect::<Vec<_>>(),
            );
            draw(
                "./out/rlcsinus.png",
                format!("4.3 rlc, ω={pulse}rad/s").as_str(),
                (x_datas.clone(), y_datas),
            )
            .expect("Failed to draw");
        }
        Err(e) => panic!("Failed to integrate: {e:?}"),
    }
}

fn main() {
    // rc4_1(10_usize.pow(4));
    // rlc4_2(5000, 1000.0);
    rlc_sinus4_3(2000, 25e3)
}
