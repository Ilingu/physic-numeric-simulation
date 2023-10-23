use std::{env::args, f64::consts::PI, format, vec};

use plotters::{
    coord::{types::RangedCoordf32, Shift},
    prelude::*,
};

#[allow(non_snake_case)]
fn F(y: f64, _t: f64, e: f64, tau: f64) -> f64 {
    (e - y) / tau
}

const AMP: f64 = 1.0;
const AMP_M: f64 = 0.5;

fn euler_method(n: usize, a: f64, b: f64, y0: f64, frequency: f64, tau: f64) -> Vec<(f64, f64)> {
    let h = (b - a) / n as f64;

    let mut result = vec![(0.0, 0.0); n + 1];
    result[0] = (a, y0);

    for k in 1..=n {
        let (x, y) = (a + k as f64 * h, result[k - 1].1);
        let e = if (2.0 * PI * frequency * x).sin() >= 0.0 {
            AMP_M.ceil()
        } else {
            AMP_M.floor()
        };

        result[k] = (x, y + F(y, 0.0, e, tau) * h);
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // paramètre d'entrée
    let args = args().collect::<Vec<_>>();
    if args.len() != 6 {
        return Err("Missing arguemtns: n, frequency, number of periode, tau, y0".into());
    }

    let n = args[1].parse::<usize>()?;
    let frequency = args[2].parse::<f64>()?;
    let periode = 1.0 / frequency;
    let periode_num = args[3].parse::<usize>()?;
    let tau = args[4].parse::<f64>()?;
    let y0 = args[5].parse::<f64>()?;

    // graph
    let root = BitMapBackend::new("out/result.png", (600, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("4.3.4: f={frequency}Hz tau={tau}s n={n}"),
            ("sans-serif", 20).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0f32..((periode_num as f64) * periode) as f32,
            (AMP_M.floor() as f32)..(AMP_M.ceil() as f32),
        )?;

    chart
        .configure_mesh()
        .y_desc("tension (V)")
        .x_desc("temps (s)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        (0..n)
            .map(|x| x as f32 * ((periode_num as f64) * periode) as f32 / n as f32)
            .map(|x| {
                (
                    x,
                    if (2.0 * PI * frequency * (x as f64)).sin() >= 0.0 {
                        AMP_M.ceil() as f32
                    } else {
                        AMP_M.floor() as f32
                    },
                )
            }),
        &GREEN,
    ))?;

    let euler_resp = euler_method(n, 0.0, (periode_num as f64) * periode, y0, frequency, tau);
    let graph_y = euler_resp.iter().map(|(_, y)| *y);
    let max_y = graph_y
        .clone()
        .reduce(f64::max)
        .ok_or("Failed to find the max height of the graph")?
        .ceil();
    let min_y = graph_y
        .clone()
        .reduce(f64::min)
        .ok_or("Failed to find the min height of the graph")?
        .ceil();
    chart.draw_series(LineSeries::new(
        euler_resp.into_iter().map(|(x, y)| (x as f32, y as f32)),
        &MAGENTA,
    ))?;

    root.present()?;
    Ok(())
}

/*
 let chart_euler = new_chart(
       &root,
       (a as f32 - frame, b as f32 + frame + 0.5),
       (min_y as f32 - frame, max_y as f32 + frame + 0.5),
       Some(format!("4.2: Circuit RC, n={n}")),
       None,
   )?;
*/
