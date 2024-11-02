use full_palette::{ORANGE, PURPLE};
use plotters::prelude::*;
use std::f32::consts::PI;
//use plotters_canvas::CanvasBackend;
use anyhow::Result;

fn main() -> Result<()> {
    //let canvas_backend = CanvasBackend::new("tmp.txt");
    let angles = vec![
        0.,
        15.0 * PI / 180.,
        35.0 * PI / 180.,
        55.0 * PI / 180.,
        95.0 * PI / 180.,
        115.0 * PI / 180.,
        135.0 * PI / 180.,
        155.0 * PI / 180.,
    ];

    let ranges = vec![44.0f32, 41.0, 38.0, 32.0, 30.0, 25.0, 23.0, 10.0];

    let x = ranges
        .iter()
        .zip(angles.iter())
        .map(|(r, theta)| r * f32::sin(*theta))
        .collect::<Vec<_>>();

    let y = ranges
        .iter()
        .zip(angles.iter())
        .map(|(r, theta)| r * f32::cos(*theta))
        .collect::<Vec<_>>();

    println!("x: {:?}", x);
    println!("y: {:?}", y);

    let mut xy = x.into_iter().zip(y.into_iter()).collect::<Vec<_>>();
    xy.push((0.0, 0.0));

    let mut neg_xy = xy
        .iter()
        .rev()
        .map(|(x, y)| (x * -1.0, *y))
        .collect::<Vec<_>>();

    xy.append(&mut neg_xy);

    println!("xy: {:?}", xy);

    //let svg = SVGBackend::new();

    let root = BitMapBackend::new("/home/andergrx/rust/plot/pics/rngs.png", (640, 480))
        .into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Ranges", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(-50f32..50f32, -20f32..50f32)?;

    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    //let vv = v;
    //And we can draw something in the drawing area
    //let xy2 = xy.iter().map(|(x,y)| (x*1.2, y*1.2)).collect::<Vec<_>>();
    let mut xyp = vec![];
    xyp.push(xy);
    // for i in 0..5 {
    //     xyp.push(
    //         xyp[0]
    //             .clone()
    //             .iter()
    //             .map(|(x, y)| (x * (1.2 + (i as f32) * 0.1), y * (1.2 + (i as f32) * 0.1)))
    //             .collect::<Vec<_>>(),
    //     );
    // }

    for (i, coords) in xyp.into_iter().enumerate() {
        //And we can draw something in the drawing area
        chart.draw_series(LineSeries::new(coords, &get_color(i)))?;
    }

    //let xy2 = xy.iter().map(|(x,y)| (x*1.2, y*1.2)).collect::<Vec<_>>();``
    //chart.draw_series(LineSeries::new(xy2, &BLUE))?;

    root.present()?;

    Ok(())
}

fn get_color(i: usize) -> RGBColor {
    match i {
        1 => return RED,
        2 => return BLUE,
        3 => return GREEN,
        4 => return PURPLE,
        _ => return ORANGE,
    }
}
