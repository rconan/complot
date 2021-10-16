use super::{Axis, Config, Utils};
use colorous;
use plotters::prelude::*;
use std::iter::FromIterator;

/// Line plots
pub struct Plot;
impl Utils for Plot {}
/*impl FromIterator<f64> for Plot {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let fig = SVGBackend::new("plot.svg", (768, 512)).into_drawing_area();
        fig.fill(&WHITE).unwrap();
        let xy: Vec<_> = iter
            .into_iter()
            .collect::<Vec<f64>>()
            .chunks(2)
            .map(|xy| (xy[0], xy[1]))
            .collect();
        let (x_max, y_max) = xy.iter().cloned().fold(
            (f64::NEG_INFINITY, f64::NEG_INFINITY),
            |(fx, fy), (x, y)| (fx.max(x), fy.max(y)),
        );
        let (x_min, y_min) = xy
            .iter()
            .cloned()
            .fold((f64::INFINITY, f64::INFINITY), |(fx, fy), (x, y)| {
                (fx.min(x), fy.min(y))
            });
        let mut ax = chart([x_min, x_max, y_min, y_max], &fig);
        ax.draw_series(xy.into_iter().map(|xy| Circle::new(xy, 3, RED.filled())))
            .unwrap();
        Plot {}
    }
}*/

/// Plots different lines (x,y1), (x,y2), ... with the data formated into an iterator
/// where each item is the tuple `(x[i], vec![y1[i], y2[i], ...])`,
/// the graph is written in the file `plot.svg`
/// ```
///(0..100).map(|k| {
///                   let o = 5.*std::f64::consts::PI*k as f64/100.;
///                   let (s,c) = o.sin_cos();
///                   (o,vec![s,c])
///                  }).collect::<complot::Plot>();
///```
impl FromIterator<(f64, Vec<f64>)> for Plot {
    fn from_iter<I: IntoIterator<Item = (f64, Vec<f64>)>>(iter: I) -> Self {
        let fig = SVGBackend::new("complot-plot.svg", (768, 512)).into_drawing_area();
        fig.fill(&WHITE).unwrap();
        let xy: Vec<_> = iter.into_iter().collect();
        let (x_max, y_max) = Self::xy_max(&xy);
        let (x_min, y_min) = Self::xy_min(&xy);
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
                .draw_series(LineSeries::new(
                    data.iter().skip(k).step_by(n_y).cloned(),
                    RGBColor(this_color.0, this_color.1, this_color.2),
                ))
                .unwrap();
        }
        Plot {}
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl<I: Iterator<Item = (f64, Vec<f64>)>> From<(I, Option<Config>)> for Plot {
    fn from((iter, config): (I, Option<Config>)) -> Self {
        fn inner<I: Iterator<Item = (f64, Vec<f64>)>>(
            (iter, config): (I, Option<Config>),
        ) -> Result<()> {
            let config = config.unwrap_or_default();
            let filename = config
                .filename
                .unwrap_or_else(|| "complot-plot.svg".to_string());

            let fig = SVGBackend::new(&filename, (768, 512)).into_drawing_area();
            fig.fill(&WHITE)?;
            let xy: Vec<_> = iter.collect();
            let (x_max, y_max) = Plot::xy_max(&xy);
            let (x_min, y_min) = Plot::xy_min(&xy);
            assert!(
                x_max > x_min,
                "Incorrect x axis range: {:?}",
                [x_min, x_max]
            );
            assert!(
                y_max > y_min,
                "Incorrect y axis range: {:?}",
                [y_min, y_max]
            );

            let mut chart = ChartBuilder::on(&fig)
                .set_label_area_size(LabelAreaPosition::Left, 50)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .margin(10)
                .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
            let mut mesh = chart.configure_mesh();
            if let Some(value) = config.xaxis.label {
                mesh.x_desc(value);
            }
            if let Some(value) = config.yaxis.label {
                mesh.y_desc(value);
            }
            mesh.draw()?;

            let n_y = xy[0].1.len();
            let data: Vec<_> = xy
                .into_iter()
                .flat_map(|(x, y)| y.into_iter().map(|y| (x, y)).collect::<Vec<(f64, f64)>>())
                .collect();
            let mut colors = colorous::TABLEAU10.iter().cycle();
            for k in 0..n_y {
                let this_color = colors
                    .next()
                    .ok_or("Couldn't get another color.")?
                    .as_tuple();
                chart.draw_series(LineSeries::new(
                    data.iter().skip(k).step_by(n_y).cloned(),
                    RGBColor(this_color.0, this_color.1, this_color.2),
                ))?;
            }
            Ok(())
        }
        if let Err(e) = inner((iter, config)) {
            println!("Complot failed in Plot: {}", e);
        }
        Plot {}
    }
}

/// Log-log plots
///
/// Like [`Plot`] but for `(log10(x),vec![log10(y),...])` items
pub struct LogLog;
impl Utils for LogLog {}
impl<I: Iterator<Item = (f64, Vec<f64>)>> From<(I, Option<Config>)> for LogLog {
    fn from((iter, config): (I, Option<Config>)) -> Self {
        let _: Plot = (
            iter.map(|(x, y)| {
                (
                    x.log10(),
                    y.into_iter().map(|y| y.log10()).collect::<Vec<f64>>(),
                )
            }),
            Some(match config {
                Some(config) => {
                    match (&config.filename, &config.xaxis.label, &config.yaxis.label) {
                        (None, None, None) => config.filename("complot-linlog.svg"),
                        (None, None, Some(label)) => {
                            let log10_label = format!("log10( {} )", label);
                            config
                                .filename("complot-linlog.svg")
                                .yaxis(Axis::new().label(log10_label))
                        }
                        (None, Some(label), None) => {
                            let log10_label = format!("log10( {} )", label);
                            config
                                .filename("complot-linlog.svg")
                                .xaxis(Axis::new().label(log10_label))
                        }
                        (None, Some(xlabel), Some(ylabel)) => {
                            let log10_xlabel = format!("log10( {} )", xlabel);
                            let log10_ylabel = format!("log10( {} )", ylabel);
                            config
                                .filename("complot-linlog.svg")
                                .xaxis(Axis::new().label(log10_xlabel))
                                .yaxis(Axis::new().label(log10_ylabel))
                        }
                        (Some(_), None, None) => config,
                        (Some(_), None, Some(label)) => {
                            let log10_label = format!("log10( {} )", label);
                            config.yaxis(Axis::new().label(log10_label))
                        }
                        (Some(_), Some(label), None) => {
                            let log10_label = format!("log10( {} )", label);
                            config.xaxis(Axis::new().label(log10_label))
                        }
                        (Some(_), Some(xlabel), Some(ylabel)) => {
                            let log10_xlabel = format!("log10( {} )", xlabel);
                            let log10_ylabel = format!("log10( {} )", ylabel);
                            config
                                .xaxis(Axis::new().label(log10_xlabel))
                                .yaxis(Axis::new().label(log10_ylabel))
                        }
                    }
                }
                None => Config::new().filename("complot-linlog.svg"),
            }),
        )
            .into();
        LogLog
    }
}
/// Log-Lin plots
///
/// Like [`Plot`] but for `(log10(x),vec![y,...])` items
pub struct LogLin;
impl Utils for LogLin {}
impl<I: Iterator<Item = (f64, Vec<f64>)>> From<(I, Option<Config>)> for LogLin {
    fn from((iter, config): (I, Option<Config>)) -> Self {
        let _: Plot = (
            iter.map(|(x, y)| (x.log10(), y)),
            Some(match config {
                Some(config) => match (&config.filename, &config.xaxis.label) {
                    (None, None) => config.filename("complot-linlog.svg"),
                    (None, Some(label)) => {
                        let log10_label = format!("log10( {} )", label);
                        config
                            .filename("complot-linlog.svg")
                            .yaxis(Axis::new().label(log10_label))
                    }
                    (Some(_), Some(label)) => {
                        let log10_label = format!("log10( {} )", label);
                        config.yaxis(Axis::new().label(log10_label))
                    }
                    (Some(_), None) => config,
                },
                None => Config::new().filename("complot-linlog.svg"),
            }),
        )
            .into();
        LogLin
    }
}
/// Lin-log plots
///
/// Like [`Plot`] but for `(x,vec![log10(y),...])` items
pub struct LinLog;
impl Utils for LinLog {}
impl<I: Iterator<Item = (f64, Vec<f64>)>> From<(I, Option<Config>)> for LinLog {
    fn from((iter, config): (I, Option<Config>)) -> Self {
        let _: Plot = (
            iter.map(|(x, y)| (x, y.into_iter().map(|y| y.log10()).collect::<Vec<f64>>())),
            Some(match config {
                Some(config) => match (&config.filename, &config.yaxis.label) {
                    (None, None) => config.filename("complot-linlog.svg"),
                    (None, Some(label)) => {
                        let log10_label = format!("log10( {} )", label);
                        config
                            .filename("complot-linlog.svg")
                            .yaxis(Axis::new().label(log10_label))
                    }
                    (Some(_), Some(label)) => {
                        let log10_label = format!("log10( {} )", label);
                        config.yaxis(Axis::new().label(log10_label))
                    }
                    (Some(_), None) => config,
                },
                None => Config::new().filename("complot-linlog.svg"),
            }),
        )
            .into();
        LinLog
    }
}
