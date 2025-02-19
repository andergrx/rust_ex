use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_area =
        BitMapBackend::new("./pics/surface_plot.png", (1024, 768))
            .into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Surface Plot: f(x, y) = x² + y²", ("sans-serif", 30))
        .margin(10)
        .set_all_label_area_size(50)
        .build_cartesian_3d(-10.0..10.0, -10.0..10.0, 0.0..20.0)?;

    chart.configure_axes().draw()?;

    chart.draw_series(SurfaceSeries::xoz(
        (-50..=50).map(|x| x as f64 / 5.0), // x-axis range
        (-50..=50).map(|y| y as f64 / 5.0), // y-axis range
        |x, y| x.powi(2) + y.powi(2),       // z = f(x, y)
    ))?;

    Ok(())
}
