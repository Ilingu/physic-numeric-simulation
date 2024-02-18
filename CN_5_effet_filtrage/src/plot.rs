use std::ops::Range;

use plotters::prelude::*;

use crate::fft::XyScatter;

fn fft_points_to_vertical_lines(fft_points: &XyScatter) -> XyScatter {
    let mut out: XyScatter = vec![];
    for &(f, amp) in fft_points {
        out.push((f, 0.0));
        out.push((f, amp));
        out.push((f, 0.0));
    }
    out
}

pub fn draw_fft(
    filename: &str,
    caption: &str,
    width: Range<f64>,
    peaks: &XyScatter,
) -> Result<(), Box<dyn std::error::Error>> {
    draw_plot(
        filename,
        caption,
        width,
        ("frequency", "amplitude"),
        Some((600, 300)),
        vec![&fft_points_to_vertical_lines(peaks)],
        None,
    )
}

pub fn draw_plot(
    filename: &str,
    caption: &str,
    width: Range<f64>,
    (legend_x, legend_y): (&str, &str),
    size: Option<(u32, u32)>,
    lines: Vec<&XyScatter>,
    lines_legend: Option<Vec<&str>>,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(lines.len() <= 7);
    let (is_line_legend, lines_legend) = match lines_legend {
        Some(ll) => {
            assert_eq!(lines.len(), ll.len(), "each lines must have a legend");
            (true, ll)
        }
        None => (false, vec![]),
    };

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
    let root = BitMapBackend::new(&save_path, size.unwrap_or((600, 600))).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (width.start)..(width.end),
            (min_y.unwrap().floor())..(max_y.unwrap().ceil()),
        )?;

    chart
        .configure_mesh()
        .x_desc(legend_x)
        .y_desc(legend_y)
        .draw()?;

    const COLORS: [RGBColor; 7] = [RED, BLUE, GREEN, BLACK, YELLOW, CYAN, MAGENTA];
    for (i, datas) in lines.into_iter().enumerate() {
        let serie = chart.draw_series(LineSeries::new(
            datas.iter().map(|&(x, y)| (x, y)),
            &COLORS[i],
        ))?;
        if is_line_legend {
            serie
                .label(lines_legend[i])
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], COLORS[i]));
        }
    }

    if is_line_legend {
        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;
    }

    root.present()?;
    Ok(())
}
