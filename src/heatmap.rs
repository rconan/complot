use super::Config;
use num_traits::{cast::AsPrimitive, Float};
use plotters::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Data<'a, T> = (&'a [T], (usize, usize));

/// Heatmap chart
///
/// ```
/// let n = 401;
/// let data = (0..n)
///     .flat_map(|k| {
///         let x = (2.*std::f64::consts::PI * (k as f64 / (n - 1) as f64 - 0.5) * 5.0);
///         let xc = x.cos();
///         (0..n)
///             .map(|k| {
///                 let y = (2.*std::f64::consts::PI * (k as f64 / (n - 1) as f64 - 0.5) * 5.0);
///                 let g = (-32f64.recip()*(x*x + y*y)).exp();
///                 0.5*g*(1. + xc * y.cos())
///             })
///             .collect::<Vec<f64>>()
///     })
///     .collect::<Vec<f64>>();
/// let _: complot::Heatmap = ((data.as_slice(), (n, n)), None).into();
/// ```
pub struct Heatmap {}
impl<'a, T: Float + AsPrimitive<f64>> From<(Data<'a, T>, Option<Config>)> for Heatmap {
    fn from((data, config): (Data<T>, Option<Config>)) -> Self {
        fn inner<T>((data, config): (Data<T>, Option<Config>)) -> Result<()>
        where
            T: Float + AsPrimitive<f64>,
        {
            let (map, (rows, cols)) = data;
            assert_eq!(rows, cols, "rectangular heatmap unimplemented");

            let config = config.unwrap_or_default();
            let osf = config.osf;
            let res = rows;
            let size = res * osf;
            let filename = config
                .filename
                .unwrap_or_else(|| "complot-heatmap.png".to_string());
            let cmap = config.cmap;

            let width = size as u32 + 40;
            let height = size as u32 + 90;
            let root = BitMapBackend::new(&filename, (width, height)).into_drawing_area();
            let (plot, colorbar) = root.split_vertically(size as u32 + 30);
            // HEATMAP
            plot.fill(&BLACK)?;
            let mut chart = ChartBuilder::on(&plot);
            chart
                .margin_left(20)
                .margin_right(20)
                .margin_top(0)
                .margin_bottom(0);
            if let Some(value) = config.title {
                chart.caption(value, ("sans-serif", 16, &WHITE));
            }
            let mut chart_ctx = chart
                .build_cartesian_2d(0i32..(size - 1) as i32, 0i32..(size - 1) as i32)
                .expect("Failed building chart");
            let (cells_min, cells_max) = match config.cmap_minmax {
                Some(value) => value,
                None => (
                    map.iter()
                        .cloned()
                        .fold(Float::infinity(), Float::min)
                        .as_(),
                    map.iter()
                        .cloned()
                        .fold(Float::neg_infinity(), Float::max)
                        .as_(),
                ),
            };
            chart_ctx
                .configure_mesh()
                .disable_x_mesh()
                .disable_y_mesh()
                .draw()?;
            chart_ctx.draw_series(map.iter().enumerate().map(|(k, &v)| {
                let j = (k / res) as i32;
                let i = (k % res) as i32;
                let u = (v.as_() - cells_min) / (cells_max - cells_min);
                let c = cmap.eval_continuous(u as f64).as_tuple();
                Rectangle::new(
                    [
                        (osf as i32 * i, osf as i32 * j),
                        (osf as i32 * (i + 1), osf as i32 * (j + 1)),
                    ],
                    RGBColor(c.0, c.1, c.2).filled(),
                )
            }))?;
            // COLORBAR
            colorbar.fill(&BLACK)?;
            let mut colorbar_chart = ChartBuilder::on(&colorbar)
                .margin_left(20)
                .margin_right(20)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(cells_min..cells_max, 0f64..1f64)?;
            let mut mesh = colorbar_chart.configure_mesh();
            mesh.axis_style(WHITE)
                .set_tick_mark_size(LabelAreaPosition::Bottom, 5)
                .x_label_style(("sans-serif", 14, &WHITE));
            if let Some(value) = config.xaxis.label {
                mesh.x_desc(value);
            }
            mesh.draw()?;
            let dx = (cells_max - cells_min) / (size - 1) as f64;
            colorbar_chart.draw_series((0..size).map(|k| {
                let x = cells_min + k as f64 * dx;
                let c = cmap.eval_rational(k, size).as_tuple();
                Rectangle::new([(x, 0.), (x + dx, 1.)], RGBColor(c.0, c.1, c.2).filled())
            }))?;
            Ok(())
        }
        if let Err(e) = inner((data, config)) {
            eprintln!("Complot failed in Heatmap: {}", e);
        }
        Heatmap {}
    }
}
