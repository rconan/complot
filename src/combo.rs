use super::{canvas, Config, Utils};
use colorous;
use plotters::prelude::*;

/// Combines plots of different [`Kind`]s
pub struct Combo {}
impl Utils for Combo {}
#[derive(Clone)]
pub enum Kind {
    Plot(Option<String>),
    Scatter(Option<String>),
}
pub type Complot = (
    Vec<Box<(dyn Iterator<Item = (f64, Vec<f64>)>)>>,
    Vec<Kind>,
    Option<Config>,
);
impl From<Complot> for Combo {
    fn from((iters, draws, config): Complot) -> Self {
        let config = config.unwrap_or_default();
        let filename = config
            .filename
            .unwrap_or_else(|| "complot-plot.svg".to_string());

        let fig = canvas(&filename, (768, 512)); //SVGBackend::new(&filename, (768, 512)).into_drawing_area();
                                                 //let fig = SVGBackend::new(&filename, (768, 512)).into_drawing_area();
        fig.fill(&WHITE).unwrap();
        /*
                let (xrange, yrange) = match (config.xaxis.range, config.yaxis.range) {
                    (Some(xrange), Some(yrange)) => (xrange, yrange),
                    (Some(xrange), None) => {
                        let mut yrange = f64::INFINITY..f64::NEG_INFINITY;
                        for iter in &iters {
                            let xy: Vec<_> = (&**iter).collect();
                            let (_, it_yrange) = Self::xy_range(&xy);
                            yrange.start = yrange.start.min(it_yrange.start);
                            yrange.end = yrange.end.max(it_yrange.end);
                        }
                        (xrange, yrange)
                    }
                    (None, Some(yrange)) => {
                        let mut xrange = f64::INFINITY..f64::NEG_INFINITY;
                        for iter in &iters {
                            let xy: Vec<_> = (&**iter).collect();
                            let (it_xrange, _) = Self::xy_range(&xy);
                            xrange.start = xrange.start.min(it_xrange.start);
                            xrange.end = xrange.end.max(it_xrange.end);
                        }
                        (xrange, yrange)
                    }
                    (None, None) => {
                        let mut xrange = f64::INFINITY..f64::NEG_INFINITY;
                        let mut yrange = f64::INFINITY..f64::NEG_INFINITY;
                        for iter in &iters {
                            let xy: Vec<_> = (&**iter).collect();
                            let (it_xrange, it_yrange) = Self::xy_range(&xy);
                            xrange.start = xrange.start.min(it_xrange.start);
                            xrange.end = xrange.end.max(it_xrange.end);
                            yrange.start = yrange.start.min(it_yrange.start);
                            yrange.end = yrange.end.max(it_yrange.end);
                        }
                        (xrange, yrange)
                    }
                };
        */
        let (xrange, yrange) = (config.xaxis.range.unwrap(), config.yaxis.range.unwrap());

        let mut chart = ChartBuilder::on(&fig)
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .margin(10)
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

        let mut colors = colorous::TABLEAU10.iter().cycle();
        let mut legend = false;
        for (iter, draw) in iters.into_iter().zip(draws.into_iter()) {
            let xy: Vec<_> = iter.collect();
            let n_y = xy[0].1.len();
            let data: Vec<_> = xy
                .into_iter()
                .flat_map(|(x, y)| y.into_iter().map(|y| (x, y)).collect::<Vec<(f64, f64)>>())
                .collect();
            match draw {
                Kind::Scatter(None) => {
                    for k in 0..n_y {
                        let this_color = colors.next().unwrap().as_tuple();
                        chart
                            .draw_series(data.iter().skip(k).step_by(n_y).cloned().map(|point| {
                                Circle::new(
                                    point,
                                    3,
                                    RGBColor(this_color.0, this_color.1, this_color.2).filled(),
                                )
                            }))
                            .unwrap();
                    }
                }
                Kind::Plot(None) => {
                    for k in 0..n_y {
                        let this_color = colors.next().unwrap().as_tuple();
                        chart
                            .draw_series(LineSeries::new(
                                data.iter().skip(k).step_by(n_y).cloned(),
                                RGBColor(this_color.0, this_color.1, this_color.2),
                            ))
                            .unwrap();
                    }
                }
                Kind::Plot(Some(label)) => {
                    legend = true;
                    for k in 0..n_y {
                        let this_color = colors.next().unwrap().as_tuple();
                        let rgb = RGBColor(
                            this_color.clone().0,
                            this_color.clone().1,
                            this_color.clone().2,
                        );
                        let leg_rgb = RGBColor(
                            this_color.clone().0,
                            this_color.clone().1,
                            this_color.clone().2,
                        );
                        chart
                            .draw_series(LineSeries::new(
                                data.iter().skip(k).step_by(n_y).cloned(),
                                rgb,
                            ))
                            .unwrap()
                            .label(label.clone())
                            .legend(move |(x, y)| {
                                PathElement::new(vec![(x, y), (x + 20, y)], leg_rgb)
                            });
                    }
                }
                Kind::Scatter(Some(label)) => {
                    legend = true;
                    for k in 0..n_y {
                        let this_color = colors.next().unwrap().as_tuple();
                        let rgb = RGBColor(
                            this_color.clone().0,
                            this_color.clone().1,
                            this_color.clone().2,
                        );
                        let leg_rgb = RGBColor(
                            this_color.clone().0,
                            this_color.clone().1,
                            this_color.clone().2,
                        );
                        chart
                            .draw_series(
                                data.iter()
                                    .skip(k)
                                    .step_by(n_y)
                                    .cloned()
                                    .map(|point| Circle::new(point, 3, rgb)),
                            )
                            .unwrap()
                            .label(label.clone())
                            .legend(move |(x, y)| {
                                PathElement::new(vec![(x, y), (x + 20, y)], leg_rgb)
                            });
                    }
                }
            }
        }
        if legend {
            chart
                .configure_series_labels()
                .border_style(&BLACK)
                .background_style(&WHITE.mix(0.8))
                .draw()
                .unwrap();
        }
        Combo {}
    }
}
