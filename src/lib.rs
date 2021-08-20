mod line;
pub use line::Plot;
mod scatter;
pub use scatter::Scatter;
use std::ops::Range;
mod combo;
pub mod tri;
pub use combo::{Combo, Complot, Kind};
use plotters::{coord::Shift, prelude::*};
mod heatmap;
pub use heatmap::Heatmap;

pub fn canvas(filename: &str) -> DrawingArea<SVGBackend, Shift> {
    let plot = SVGBackend::new(filename, (768, 768)).into_drawing_area();
    plot.fill(&WHITE).unwrap();
    plot
}
pub fn png_canvas(filename: &str) -> DrawingArea<BitMapBackend, Shift> {
    let plot = BitMapBackend::new(filename, (768, 768)).into_drawing_area();
    plot.fill(&WHITE).unwrap();
    plot
}

#[derive(Default, Clone)]
pub struct Axis<'a> {
    label: Option<&'a str>,
    range: Option<Range<f64>>,
}
impl<'a> Axis<'a> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn label(self, label: &'a str) -> Self {
        Self {
            label: Some(label),
            ..self
        }
    }
    pub fn range(self, range: Range<f64>) -> Self {
        Self {
            range: Some(range),
            ..self
        }
    }
}
#[derive(Clone)]
pub struct Config<'a> {
    filename: Option<String>,
    title: Option<String>,
    xaxis: Axis<'a>,
    yaxis: Axis<'a>,
    cmap: colorous::Gradient,
    cmap_minmax: Option<(f64, f64)>,
    osf: usize,
}
impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            filename: Default::default(),
            title: None,
            xaxis: Default::default(),
            yaxis: Default::default(),
            cmap: colorous::VIRIDIS,
            cmap_minmax: None,
            osf: 2,
        }
    }
}
impl<'a> Config<'a> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn filename<T: AsRef<str>>(self, filename: T) -> Self {
        Self {
            filename: Some(filename.as_ref().to_string()),
            ..self
        }
    }
    pub fn title(self, title: String) -> Self {
        Self {
            title: Some(title),
            ..self
        }
    }
    pub fn over_sampling_factor(self, osf: usize) -> Self {
        Self { osf, ..self }
    }
    pub fn cmap_minmax(self, cmap_minmax: (f64, f64)) -> Self {
        Self {
            cmap_minmax: Some(cmap_minmax),
            ..self
        }
    }
    pub fn xaxis(self, xaxis: Axis<'a>) -> Self {
        Self { xaxis, ..self }
    }
    pub fn yaxis(self, yaxis: Axis<'a>) -> Self {
        Self { yaxis, ..self }
    }
    pub fn axes(self, axis: Axis<'a>) -> Self {
        Self {
            xaxis: axis.clone(),
            yaxis: axis,
            ..self
        }
    }
    pub fn auto_range(&mut self, iters: Vec<&[(f64, Vec<f64>)]>) -> &mut Self {
        let mut xrange = f64::INFINITY..f64::NEG_INFINITY;
        let mut yrange = f64::INFINITY..f64::NEG_INFINITY;
        for iter in &iters {
            let xy = *iter;
            let (it_xrange, it_yrange) = Plot::xy_range(&xy);
            xrange.start = xrange.start.min(it_xrange.start);
            xrange.end = xrange.end.max(it_xrange.end);
            yrange.start = yrange.start.min(it_yrange.start);
            yrange.end = yrange.end.max(it_yrange.end);
        }
        self.xaxis = self.xaxis.clone().range(xrange);
        self.yaxis = self.yaxis.clone().range(yrange);
        self
    }
}
pub trait Utils {
    fn xy_max(data: &[(f64, Vec<f64>)]) -> (f64, f64) {
        data.iter().cloned().fold(
            (f64::NEG_INFINITY, f64::NEG_INFINITY),
            |(fx, fy), (x, y)| {
                (
                    fx.max(x),
                    fy.max(y.iter().cloned().fold(f64::NEG_INFINITY, |fy, y| fy.max(y))),
                )
            },
        )
    }
    fn xy_min(data: &[(f64, Vec<f64>)]) -> (f64, f64) {
        data.iter()
            .cloned()
            .fold((f64::INFINITY, f64::INFINITY), |(fx, fy), (x, y)| {
                (
                    fx.min(x),
                    fy.min(y.iter().cloned().fold(f64::INFINITY, |fy, y| fy.min(y))),
                )
            })
    }
    fn xy_range(data: &[(f64, Vec<f64>)]) -> (Range<f64>, Range<f64>) {
        let (x_max, y_max) = Self::xy_max(data);
        let (x_min, y_min) = Self::xy_min(data);
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
        (x_min..x_max, y_min..y_max)
    }
}
