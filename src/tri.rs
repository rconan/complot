use crate::Config;
use colorous;
use plotters::{coord::types::RangedCoordf64, prelude::*};
use spade::{
    delaunay::{DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation},
    kernels::FloatKernel,
};
use std::{error::Error, iter::FromIterator, ops::Range};

pub struct Mesh {}
impl<'a> FromIterator<f64> for Mesh {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let fig = crate::canvas("complot-tri-mesh.svg");
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
        let mut ax = crate::chart([x_min, x_max, y_min, y_max], &fig);
        let (x, y): (Vec<_>, Vec<_>) = xy.into_iter().unzip();
        trimesh(&x, &y, [0; 3], &mut ax);
        Mesh {}
    }
}

pub fn trimesh<'a, D: DrawingBackend>(
    x: &[f64],
    y: &[f64],
    color: [u8; 3],
    chart: &mut ChartContext<'a, D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) {
    let mut tri = FloatDelaunayTriangulation::with_walk_locate();
    x.iter().zip(y.iter()).for_each(|(x, y)| {
        tri.insert([*x, *y]);
    });
    tri.mesh(&x, &y, color, chart);
}

pub fn trimap<'a, D: DrawingBackend>(
    x: &[f64],
    y: &[f64],
    z: &[f64],
    chart: &mut ChartContext<'a, D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) {
    let mut tri = FloatDelaunayTriangulation::with_walk_locate();
    x.iter().zip(y.iter()).for_each(|(x, y)| {
        tri.insert([*x, *y]);
    });
    tri.map(&x, &y, &z, chart);
}
pub fn heatmap(
    x: &[f64],
    y: &[f64],
    z: &[f64],
    range: Range<f64>,
    config: Option<Config>,
) -> Result<(), Box<dyn Error>> {
    let mut tri = FloatDelaunayTriangulation::with_walk_locate();
    x.iter().zip(y.iter()).for_each(|(x, y)| {
        tri.insert([*x, *y]);
    });
    tri.heatmap(&x, &y, &z, range, config)
}

pub trait TriPlot {
    fn mesh<'a, D: DrawingBackend>(
        &self,
        x: &[f64],
        y: &[f64],
        color: [u8; 3],
        chart: &mut ChartContext<'a, D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) -> &Self;
    fn map<'a, D: DrawingBackend>(
        &self,
        x: &[f64],
        y: &[f64],
        z: &[f64],
        chart: &mut ChartContext<'a, D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) -> &Self;
    fn heatmap(
        &self,
        x: &[f64],
        y: &[f64],
        z: &[f64],
        range: Range<f64>,
        config: Option<Config>,
    ) -> Result<(), Box<dyn Error>>;
}

impl TriPlot for DelaunayTriangulation<[f64; 2], FloatKernel, DelaunayWalkLocate> {
    fn mesh<'a, D: DrawingBackend>(
        &self,
        x: &[f64],
        y: &[f64],
        color: [u8; 3],
        chart: &mut ChartContext<'a, D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) -> &Self {
        let color = RGBColor(color[0], color[1], color[2]);
        self.triangles()
            .map(|t| {
                t.as_triangle()
                    .iter()
                    .map(|&i| (x[i.fix()], y[i.fix()]))
                    .collect::<Vec<(f64, f64)>>()
            })
            .for_each(|v| {
                chart
                    .draw_series(LineSeries::new(
                        v.iter().cycle().take(4).map(|(x, y)| (*x, *y)),
                        &color,
                    ))
                    .unwrap();
            });
        self
    }
    fn map<'a, D: DrawingBackend>(
        &self,
        x: &[f64],
        y: &[f64],
        z: &[f64],
        chart: &mut ChartContext<'a, D, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) -> &Self {
        let cells: Vec<f64> = self
            .triangles()
            .map(|t| t.as_triangle().iter().fold(0., |a, &i| a + z[i.fix()] / 3.))
            .collect();
        let cells_max = cells.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let cells_min = cells.iter().cloned().fold(f64::INFINITY, f64::min);
        let unit_cells: Vec<f64> = cells
            .iter()
            .map(|p| (p - cells_min) / (cells_max - cells_min))
            .collect();
        let cmap = colorous::CIVIDIS;
        self.triangles()
            .map(|t| {
                t.as_triangle()
                    .iter()
                    .map(|&i| (x[i.fix()], y[i.fix()]))
                    .collect::<Vec<(f64, f64)>>()
            })
            .zip(unit_cells.iter())
            .for_each(|(v, p)| {
                chart
                    .draw_series(std::iter::once(Polygon::new(
                        v.clone(),
                        if p.is_nan() {
                            BLACK.filled()
                        } else {
                            let c = cmap.eval_continuous(*p).as_tuple();
                            RGBColor(c.0, c.1, c.2).filled()
                        },
                    )))
                    .unwrap();
            });
        self
    }
    fn heatmap(
        &self,
        x: &[f64],
        y: &[f64],
        z: &[f64],
        range: Range<f64>,
        config: Option<Config>,
    ) -> Result<(), Box<dyn Error>> {
        let config = config.unwrap_or_default();
        let filename = config.filename.unwrap_or_else(|| "heatmap.png".to_string());

        let root_area = BitMapBackend::new(&filename, (800, 880)).into_drawing_area();

        root_area.fill(&WHITE)?;
        let (plot, colorbar) = root_area.split_vertically(800);
        let mut ctx = ChartBuilder::on(&plot)
            .margin(20)
            .set_label_area_size(LabelAreaPosition::Left, 20)
            .set_label_area_size(LabelAreaPosition::Bottom, 20)
            .build_cartesian_2d(range.clone(), range)?;
        ctx.configure_mesh().draw()?;
        trimap(&x, &y, &z, &mut ctx);
        /*    ctx.draw_series(
                coord
                    .iter()
                    .map(|&point| Circle::new(point, 2, BLACK.mix(0.25))),
            )
            .unwrap();
        }*/

        // COLORBAR
        let cells_min = z.iter().cloned().fold(f64::INFINITY, f64::min);
        let cells_max = z.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        colorbar.fill(&BLACK)?;
        let mut colorbar_chart = ChartBuilder::on(&colorbar)
            //    .margin_left(20)
            //    .margin_right(20)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(cells_min..cells_max, 0f64..1f64)?;
        let mut mesh = colorbar_chart.configure_mesh();
        mesh.axis_style(WHITE)
            .set_tick_mark_size(LabelAreaPosition::Bottom, 5)
            .x_label_style(("sans-serif", 14, &WHITE))
            .x_desc("Surface error [micron]")
            .draw()?;
        let dx = (cells_max - cells_min) / (800 - 1) as f64;
        let cmap = colorous::CIVIDIS;
        colorbar_chart.draw_series((0..800).map(|k| {
            let x = cells_min + k as f64 * dx;
            let c = cmap.eval_rational(k, 800).as_tuple();
            Rectangle::new([(x, 0.), (x + dx, 1.)], RGBColor(c.0, c.1, c.2).filled())
        }))?;
        root_area.present()?;
        Ok(())
    }
}
