use std::ops::Range;

use plotters::prelude::*;

use crate::fft::XyScatter;

pub fn draw_plot(
    filename: &str,
    caption: &str,
    width: Range<f64>,
    (legend_x, legend_y): (&str, &str),
    lines: Vec<&XyScatter>,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(lines.len() <= 7);

    let (mut max_y, mut min_y) = (None::<f64>, None::<f64>);
    for datas in &lines {
        let sub_max_y = datas
            .iter()
            .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
            .ok_or("Failed to find the max height of the graph")?
            .1
            .ceil();
        let sub_min_y = datas
            .iter()
            .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
            .ok_or("Failed to find the max height of the graph")?
            .1
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

    let save_path = format!("./out/{filename}");
    let root = BitMapBackend::new(&save_path, (600, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (width.start as f32)..(width.end as f32),
            (min_y.unwrap().floor() as f32)..(max_y.unwrap().ceil() as f32),
        )?;

    chart
        .configure_mesh()
        .x_desc(legend_x)
        .y_desc(legend_y)
        .draw()?;

    const COLORS: [RGBColor; 7] = [BLACK, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA];
    for (i, datas) in lines.iter().enumerate() {
        chart.draw_series(LineSeries::new(
            datas.iter().map(|&(x, y)| (x as f32, y as f32)),
            &COLORS[i],
        ))?;
    }

    root.present()?;
    Ok(())
}
