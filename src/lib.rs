use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::prelude::*;
use spade::delaunay::{DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation};
use spade::kernels::FloatKernel;

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
                            HSLColor(*p*0.65, 0.5, 0.4).filled()
                        },
                    )))
                    .unwrap();
            });
        self
    }
}
