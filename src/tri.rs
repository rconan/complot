use super::Config;
use plotters::prelude::*;

pub struct Mesh {}

impl<'a, I: Iterator<Item = Vec<(f64, f64)>>> From<(I, Option<Config<'a>>)> for Mesh {
    fn from((iter, config): (I, Option<Config>)) -> Self {
        let config = config.unwrap_or_default();
        let filename = config
            .filename
            .unwrap_or_else(|| "complot-plot.svg".to_string());

        let fig = SVGBackend::new(&filename, (768, 768)).into_drawing_area();
        fig.fill(&WHITE).unwrap();
        let xy: Vec<_> = iter.collect();
        let (x_max, y_max) = xy
            .iter()
            .flatten()
            .cloned()
            .reduce(|(a, b), (x, y)| (a.max(x), b.max(y)))
            .unwrap();
        let (x_min, y_min) = xy
            .iter()
            .flatten()
            .cloned()
            .reduce(|(a, b), (x, y)| (a.min(x), b.min(y)))
            .unwrap();

        let xrange = if let Some(xrange) = config.xaxis.range {
            xrange
        } else {
            x_min..x_max
        };
        let yrange = if let Some(yrange) = config.yaxis.range {
            yrange
        } else {
            y_min..y_max
        };

        let mut chart = ChartBuilder::on(&fig)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .margin(20)
            .build_cartesian_2d(xrange, yrange)
            .unwrap();
        let mut mesh = chart.configure_mesh();
        if let Some(value) = config.xaxis.label {
            mesh.x_desc(value);
        }
        if let Some(value) = config.yaxis.label {
            mesh.y_desc(value);
        }
        mesh.draw().unwrap();

        xy.into_iter().for_each(|v| {
            chart
                .draw_series(LineSeries::new(
                    v.iter().cycle().take(4).map(|(x, y)| (*x, *y)),
                    &BLACK,
                ))
                .unwrap();
        });
        Mesh {}
    }
}

pub struct Heatmap {}

impl<'a, I: Iterator<Item = (Vec<(f64, f64)>, f64)>> From<(I, Option<Config<'a>>)> for Heatmap {
    fn from((iter, config): (I, Option<Config>)) -> Self {
        let config = config.unwrap_or_default();
        let filename = config
            .filename
            .unwrap_or_else(|| "complot-plot.svg".to_string());

        let size = 768usize;
        let cb_size = 80;
        let root =
            SVGBackend::new(&filename, (size as u32, size as u32 + cb_size)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let (fig, colorbar) = root.split_vertically(size as u32);
        let mut xy: Vec<_> = iter.collect();
        let (x_max, y_max) = xy
            .iter()
            .flat_map(|(v, _)| v.clone())
            .reduce(|(a, b), (x, y)| (a.max(x), b.max(y)))
            .unwrap();
        let (x_min, y_min) = xy
            .iter()
            .flat_map(|(v, _)| v.clone())
            .reduce(|(a, b), (x, y)| (a.min(x), b.min(y)))
            .unwrap();

        let xrange = if let Some(xrange) = config.xaxis.range {
            xrange
        } else {
            x_min..x_max
        };
        let yrange = if let Some(yrange) = config.yaxis.range {
            yrange
        } else {
            y_min..y_max
        };

        let cells_max = xy
            .iter()
            .map(|(_, p)| p)
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let cells_min = xy
            .iter()
            .map(|(_, p)| p)
            .cloned()
            .fold(f64::INFINITY, f64::min);
        xy.iter_mut().for_each(|(_, p)| {
            *p = (*p - cells_min) / (cells_max - cells_min);
        });

        let mut chart = ChartBuilder::on(&fig)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .margin(20)
            .build_cartesian_2d(xrange, yrange)
            .unwrap();
        let mut mesh = chart.configure_mesh();
        if let Some(value) = config.xaxis.label {
            mesh.x_desc(value);
        }
        if let Some(value) = config.yaxis.label {
            mesh.y_desc(value);
        }
        mesh.draw().unwrap();
        let cmap = colorous::CIVIDIS;
        xy.into_iter().for_each(|(v, p)| {
            chart
                .draw_series(std::iter::once(Polygon::new(
                    v.clone(),
                    if p.is_nan() {
                        BLACK.filled()
                    } else {
                        let c = cmap.eval_continuous(p).as_tuple();
                        RGBColor(c.0, c.1, c.2).filled()
                    },
                )))
                .unwrap();
        });

        // COLORBAR
        colorbar.fill(&BLACK).unwrap();
        let mut colorbar_chart = ChartBuilder::on(&colorbar)
            //    .margin_left(20)
            //    .margin_right(20)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(cells_min..cells_max, 0f64..1f64)
            .unwrap();
        let mut mesh = colorbar_chart.configure_mesh();
        mesh.axis_style(WHITE)
            .set_tick_mark_size(LabelAreaPosition::Bottom, 5)
            .x_label_style(("sans-serif", 14, &WHITE))
            .x_desc("Surface error [micron]")
            .draw()
            .unwrap();
        let dx = (cells_max - cells_min) / (size - 1) as f64;
        let cmap = colorous::CIVIDIS;
        colorbar_chart
            .draw_series((0..size).map(|k| {
                let x = cells_min + k as f64 * dx;
                let c = cmap.eval_rational(k, size).as_tuple();
                Rectangle::new([(x, 0.), (x + dx, 1.)], RGBColor(c.0, c.1, c.2).filled())
            }))
            .unwrap();
        root.present().unwrap();

        Heatmap {}
    }
}
