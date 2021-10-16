//! Complot is an idiomatic high-level wrapper to Rust [plotters](https://docs.rs/plotters/0.3.0/plotters/) visualization crate.
//! Complot allows to quickly visually inspect data without any other knowledge than the Rust standard library.
//! Complot relies on Rust traits from the standard libray to produce the plots and on a simple tree of structures to configure the plots.
//!
//! # Example
//!
//! Plotting sine and cosine functions
//! ```
//!(0..100).map(|k| {
//!                   let o = 5.*std::f64::consts::PI*k as f64/100.;
//!                   let (s,c) = o.sin_cos();
//!                   (o,vec![s,c])
//!                  }).collect::<complot::Plot>();
//!```
//! Plotting sine and cosine functions with custom properties
//!```
//!let _: complot::Plot = (
//! (0..100).map(|k| {
//!                    let o = 5.*std::f64::consts::PI*k as f64/100.;
//!                    let (s,c) = o.sin_cos();
//!                    (o,vec![s,c])
//!                   }),
//! complot::complot!("sin_cos.svg", xlabel="x label", ylabel= "y label")
//!                       ).into();
//!```

/// Macro to set some graph properties
///
/// Returns [`Some`] [`Config`]
/// ```
/// # #[macro_use] extern crate complot;
/// # fn main() {
/// complot!("filename");
/// complot!("filename", xlabel="xlabel");
/// complot!("filename", ylabel="xlabel");
/// complot!("filename", xlabel="xlabel", ylabel="ylabel");
/// complot!("filename", xlabel="xlabel", ylabel="ylabel", title="title");
/// # }
///```
#[macro_export]
macro_rules! complot {
    ($filename:expr) => {
        Some(complot::Config::new().filename($filename))
    };
    ($filename:expr, xlabel=$xlabel:expr) => {
        Some(
            complot::Config::new()
                .filename($filename)
                .xaxis(complot::Axis::new().label($xlabel)),
        )
    };
    ($filename:expr, ylabel=$ylabel:expr) => {
        Some(
            complot::Config::new()
                .filename($filename)
                .yaxis(complot::Axis::new().label($ylabel)),
        )
    };
    ($filename:expr, xlabel=$xlabel:expr, ylabel=$ylabel:expr) => {
        Some(
            complot::Config::new()
                .filename($filename)
                .xaxis(complot::Axis::new().label($xlabel))
                .yaxis(complot::Axis::new().label($ylabel)),
        )
    };
    ($filename:expr, xlabel=$xlabel:expr, ylabel=$ylabel:expr, title=$title:expr) => {
        Some(
            complot::Config::new()
                .filename($filename)
                .xaxis(complot::Axis::new().label($xlabel))
                .yaxis(complot::Axis::new().label($ylabel))
                .title($title),
        )
    };
}

mod line;
pub use line::{LinLog, LogLin, LogLog, Plot};
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

/// Axis properties
#[derive(Default, Clone)]
pub struct Axis {
    label: Option<String>,
    range: Option<Range<f64>>,
}
impl Axis {
    /// Creates a new axis
    pub fn new() -> Self {
        Default::default()
    }
    /// Sets the axis label
    pub fn label<S>(self, label: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            label: Some(label.into()),
            ..self
        }
    }
    /// Sets the axis range
    pub fn range(self, range: Range<f64>) -> Self {
        Self {
            range: Some(range),
            ..self
        }
    }
}
/// Colorbar properties
#[derive(Clone)]
pub struct Colorbar {
    cmap: colorous::Gradient,
    label: Option<String>,
    range: Option<Range<f64>>,
}
impl Default for Colorbar {
    fn default() -> Self {
        Self {
            cmap: colorous::VIRIDIS,
            label: None,
            range: None,
        }
    }
}
/// Graph properties
#[derive(Clone)]
pub struct Config {
    filename: Option<String>,
    title: Option<String>,
    xaxis: Axis,
    yaxis: Axis,
    cmap: colorous::Gradient,
    cmap_minmax: Option<(f64, f64)>,
    colorbar: Option<Colorbar>,
    osf: usize,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            filename: Default::default(),
            title: None,
            xaxis: Default::default(),
            yaxis: Default::default(),
            cmap: colorous::VIRIDIS,
            cmap_minmax: None,
            colorbar: None,
            osf: 2,
        }
    }
}
impl Config {
    /// Creates a new graph
    pub fn new() -> Self {
        Default::default()
    }
    /// Sets the filename to save the graph to
    pub fn filename<T>(self, filename: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            filename: Some(filename.into()),
            ..self
        }
    }
    /// Sets the graph title
    pub fn title<S>(self, title: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: Some(title.into()),
            ..self
        }
    }
    pub fn over_sampling_factor(self, osf: usize) -> Self {
        Self { osf, ..self }
    }
    /// Sets the colormap upper and lower bounds
    pub fn cmap_minmax(self, cmap_minmax: (f64, f64)) -> Self {
        Self {
            cmap_minmax: Some(cmap_minmax),
            ..self
        }
    }
    /// Sets the x-axis properties
    pub fn xaxis(self, xaxis: Axis) -> Self {
        Self { xaxis, ..self }
    }
    /// Sets the y-axis properties
    pub fn yaxis(self, yaxis: Axis) -> Self {
        Self { yaxis, ..self }
    }
    /// Sets the x and y axes to the same axis properties
    pub fn axes(self, axis: Axis) -> Self {
        Self {
            xaxis: axis.clone(),
            yaxis: axis,
            ..self
        }
    }
    /// Adds a colorbar to the graph
    pub fn with_colorbar(self) -> Self {
        if self.colorbar.is_none() {
            Self {
                colorbar: Default::default(),
                ..self
            }
        } else {
            self
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
trait Utils {
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
