use super::{Config, Utils};
use colorous;
use plotters::prelude::*;
use std::iter::FromIterator;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Scatter {}
impl Utils for Scatter {}

/// Plots scattered data (x,y1), (x,y2), ... with the data formated into an iterator
/// where each item is the tuple `(x[i], vec![y1[i], y2[i], ...])`,
/// the graph is written in the file `scatter.svg`
/// ```
///(0..100).map(|k| {
///                   let o = 5.*std::f64::consts::PI*k as f64/100.;
///                   let (s,c) = o.sin_cos();
///                   (c,vec![s])
///                  }).collect::<complot::Scatter>();
///```
impl<'a> FromIterator<(f64, Vec<f64>)> for Scatter {
    fn from_iter<I: IntoIterator<Item = (f64, Vec<f64>)>>(iter: I) -> Self {
        fn inner<I: IntoIterator<Item = (f64, Vec<f64>)>>(iter: I) -> Result<()> {
            let fig = SVGBackend::new("scatter.svg", (768, 512)).into_drawing_area();
            fig.fill(&WHITE).unwrap();
            let xy: Vec<_> = iter.into_iter().collect();
            let (x_max, y_max) = Scatter::xy_max(&xy);
            let (x_min, y_min) = Scatter::xy_min(&xy);
            let mut chart = ChartBuilder::on(&fig)
                .set_label_area_size(LabelAreaPosition::Left, 50)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .margin(10)
                .build_cartesian_2d(x_min..x_max, y_min..y_max)
                .unwrap();
            chart.configure_mesh().draw().unwrap();
            let n_y = xy[0].1.len();
            let data: Vec<_> = xy
                .into_iter()
                .flat_map(|(x, y)| y.into_iter().map(|y| (x, y)).collect::<Vec<(f64, f64)>>())
                .collect();
            let mut colors = colorous::TABLEAU10.iter().cycle();
            for k in 0..n_y {
                let this_color = colors.next().unwrap().as_tuple();
                chart
                    .draw_series(data.iter().skip(k).step_by(n_y).cloned().map(|point| {
                        Circle::new(point, 3, RGBColor(this_color.0, this_color.1, this_color.2))
                    }))
                    .unwrap();
            }
            Ok(())
        }
        if let Err(e) = inner(iter) {
            println!("Complot failed in `Scatter::FromIterator` : {}", e);
        }
        Scatter {}
    }
}

impl<'a, I: Iterator<Item = (f64, Vec<f64>)>> From<(I, Option<Config<'a>>)> for Scatter {
    fn from((iter, config): (I, Option<Config>)) -> Self {
        let config = config.unwrap_or_default();
        let filename = config
            .filename
            .unwrap_or_else(|| "complot-plot.svg".to_string());

        let fig = SVGBackend::new(&filename, (768, 768)).into_drawing_area();
        fig.fill(&WHITE).unwrap();
        let xy: Vec<_> = iter.collect();

        let (x_max, y_max) = xy.iter().cloned().fold(
            (f64::NEG_INFINITY, f64::NEG_INFINITY),
            |(fx, fy), (x, y)| {
                (
                    fx.max(x),
                    fy.max(y.iter().cloned().fold(f64::NEG_INFINITY, |fy, y| fy.max(y))),
                )
            },
        );
        let (x_min, y_min) =
            xy.iter()
                .cloned()
                .fold((f64::INFINITY, f64::INFINITY), |(fx, fy), (x, y)| {
                    (
                        fx.min(x),
                        fy.min(y.iter().cloned().fold(f64::INFINITY, |fy, y| fy.min(y))),
                    )
                });

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
            //            .set_label_area_size(LabelAreaPosition::Left, 50)
            //            .set_label_area_size(LabelAreaPosition::Bottom, 40)
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

        let n_y = xy.iter().nth(0).unwrap().1.len();
        let data: Vec<_> = xy
            .into_iter()
            .flat_map(|(x, y)| y.into_iter().map(|y| (x, y)).collect::<Vec<(f64, f64)>>())
            .collect();
        let mut colors = colorous::TABLEAU10.iter().cycle();
        for k in 0..n_y {
            let this_color = colors.next().unwrap().as_tuple();
            chart
                .draw_series(data.iter().skip(k).step_by(n_y).cloned().map(|point| {
                    Circle::new(point, 3, RGBColor(this_color.0, this_color.1, this_color.2))
                }))
                .unwrap();
        }
        Scatter {}
    }
}
