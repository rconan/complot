use colorous;
use num_traits::{cast::AsPrimitive, Float};
use plotters::{
    coord::{types::RangedCoordf64, Shift},
    prelude::*,
};
use std::iter::FromIterator;
use std::{error::Error, marker::PhantomData};

pub mod tri;

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

pub struct Font {
    pub r#type: String,
    pub size: usize,
}
impl Default for Font {
    fn default() -> Self {
        Self {
            r#type: String::from("sans-serif"),
            size: 12,
        }
    }
}
#[derive(Default)]
pub struct Text {
    pub text: String,
    pub font: Option<Font>,
}
#[derive(Default)]
pub struct Axis {
    pub title: Option<Text>,
}

pub fn chart<'a, D: DrawingBackend>(
    lims: [f64; 4],
    plot: &'a DrawingArea<D, Shift>,
) -> ChartContext<'a, D, Cartesian2d<RangedCoordf64, RangedCoordf64>> {
    let mut chart = ChartBuilder::on(plot)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .margin_top(40)
        .margin_right(40)
        .build_cartesian_2d(lims[0]..lims[1], lims[2]..lims[3])
        .unwrap();
    chart.configure_mesh().draw().unwrap();
    chart
}

pub fn imagesc<'a, D: DrawingBackend>(data: &[f64], root: &'a DrawingArea<D, Shift>) {
    let n = (data.len() as f64).sqrt() as usize;

    let mut chart = ChartBuilder::on(root)
        .build_cartesian_2d(0i32..(n - 1) as i32, 0i32..(n - 1) as i32)
        .expect("Failed building chart");
    let cells_max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let cells_min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let cmap = colorous::CUBEHELIX;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .unwrap();
    chart
        .draw_series(data.iter().enumerate().map(|(k, v)| {
            let i = (k / n) as i32;
            let j = (k % n) as i32;
            let u = (v - cells_min) / (cells_max - cells_min);
            let c = cmap.eval_continuous(u).as_tuple();
            Rectangle::new([(i, j), (i + 1, j + 1)], RGBColor(c.0, c.1, c.2).filled())
        }))
        .expect("Failed drawing image");
}

pub struct Plot {}
impl<'a> FromIterator<f64> for Plot {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let fig = canvas("m2_s7_delaunay.svg");
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
}
impl<'a> FromIterator<&'a f64> for Plot {
    fn from_iter<I: IntoIterator<Item = &'a f64>>(iter: I) -> Self {
        let fig = canvas("m2_s7_delaunay.svg");
        let xy: Vec<_> = iter
            .into_iter()
            .cloned()
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
}
impl<'a> From<&'a [f64]> for Plot {
    fn from(points: &[f64]) -> Self {
        let fig = canvas("m2_s7_delaunay.svg");
        let lim = 0.55_f64;
        let mut ax = chart([-lim, lim, -lim, lim], &fig);
        ax.draw_series(
            points
                .iter()
                .enumerate()
                .map(|(k, &point)| Circle::new((k as f64, point), 3, RED.filled())),
        )
        .unwrap();
        Plot {}
    }
}
impl<'a> From<[&'a [f64]; 2]> for Plot {
    fn from(points: [&'a [f64]; 2]) -> Self {
        let fig = canvas("m2_s7_delaunay.svg");
        let lim = 0.55_f64;
        let mut ax = chart([-lim, lim, -lim, lim], &fig);
        ax.draw_series(
            points[0]
                .iter()
                .zip(points[1].iter())
                .map(|(&x, &y)| Circle::new((x, y), 3, RED.filled())),
        )
        .unwrap();
        Plot {}
    }
}
impl<'a> From<Vec<&'a [f64]>> for Plot {
    fn from(points_set: Vec<&[f64]>) -> Self {
        let fig = canvas("m2_s7_delaunay.svg");
        let lim = 0.55_f64;
        let mut ax = chart([-lim, lim, -lim, lim], &fig);
        for points in points_set.iter() {
            ax.draw_series(
                points
                    .chunks(2)
                    .map(|point| Circle::new((point[0], point[1]), 3, RED.filled())),
            )
            .unwrap();
        }
        Plot {}
    }
}
impl<'a> From<(&'a [f64], Config<'a>)> for Plot {
    fn from((points, config): (&[f64], Config)) -> Self {
        let filename = config.filename.unwrap_or_else(|| "plot.svg".to_string());
        let fig = canvas(&filename);
        let lim = 0.55_f64;
        let mut ax = chart([-lim, lim, -lim, lim], &fig);
        ax.draw_series(
            points
                .chunks(2)
                .map(|point| Circle::new((point[0], point[1]), 3, RED.filled())),
        )
        .unwrap();
        Plot {}
    }
}

pub struct Heatmap<T: Float + AsPrimitive<f64>> {
    data: std::marker::PhantomData<T>,
}
impl<T: Float + AsPrimitive<f64>> From<(&[T], (usize, usize))> for Heatmap<T> {
    fn from(data: (&[T], (usize, usize))) -> Self {
        if heatmap(data, None).is_err() {
            println!("Complot::Heatmap has failed!")
        };
        Heatmap {
            data: PhantomData {},
        }
    }
}
impl<'a, T: Float + AsPrimitive<f64>> From<((&'a [T], (usize, usize)), Config<'a>)> for Heatmap<T> {
    fn from((data, config): ((&'a [T], (usize, usize)), Config)) -> Self {
        if heatmap(data, Some(config)).is_err() {
            println!("Complot::Heatmap has failed!")
        };
        Heatmap {
            data: PhantomData {},
        }
    }
}

/// Heatmap configuration parameters
#[derive(Default, Clone)]
pub struct XAxis<'a> {
    label: Option<&'a str>,
}
impl<'a> XAxis<'a> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn label(self, label: &'a str) -> Self {
        Self {
            label: Some(label),
            ..self
        }
    }
}
#[derive(Clone)]
pub struct Config<'a> {
    filename: Option<String>,
    title: Option<String>,
    xaxis: XAxis<'a>,
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
            cmap: colorous::VIRIDIS,
            cmap_minmax: None,
            osf: 1,
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
    pub fn xaxis(self, xaxis: XAxis<'a>) -> Self {
        Self { xaxis, ..self }
    }
}
/*pub struct Figure<'a> {
    backend: DrawingArea<BitMapBackend<'a>, Shift>,
    axe: ChartContext<'a, BitMapBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
}*/
/// Heatmap
pub fn heatmap<T: Float + AsPrimitive<f64>>(
    data: (&[T], (usize, usize)),
    config: Option<Config>,
) -> Result<(), Box<dyn Error>> {
    let (map, (rows, cols)) = data;
    assert_eq!(rows, cols, "rectangular heatmap unimplemented");

    let config = config.unwrap_or_default();
    let osf = config.osf;
    let res = rows;
    let size = res * osf;
    let filename = config.filename.unwrap_or_else(|| "heatmap.png".to_string());
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
