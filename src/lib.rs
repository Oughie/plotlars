#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

mod common;
mod components;
mod plots;

pub use crate::common::plot::Plot;
pub use crate::components::axis::{Axis, AxisSide, AxisType};
pub use crate::components::color::Rgb;
pub use crate::components::colorbar::ColorBar;
pub use crate::components::contours::{Coloring, Contours};
pub use crate::components::exponent::ValueExponent;
pub use crate::components::legend::Legend;
pub use crate::components::line::Line;
pub use crate::components::orientation::Orientation;
pub use crate::components::palette::Palette;
pub use crate::components::shape::Shape;
pub use crate::components::arrangement::Arrangement;
pub use crate::components::text::Text;
pub use crate::components::tick::TickDirection;
pub use crate::plots::array2dplot::Array2dPlot;
pub use crate::plots::barplot::BarPlot;
pub use crate::plots::boxplot::BoxPlot;
pub use crate::plots::contourplot::ContourPlot;
pub use crate::plots::heatmap::HeatMap;
pub use crate::plots::histogram::Histogram;
pub use crate::plots::image::Image;
pub use crate::plots::lineplot::LinePlot;
pub use crate::plots::piechart::PieChart;
pub use crate::plots::sankeydiagram::SankeyDiagram;
pub use crate::plots::scatter3dplot::Scatter3dPlot;
pub use crate::plots::scattermap::ScatterMap;
pub use crate::plots::scatterplot::ScatterPlot;
pub use crate::plots::timeseriesplot::TimeSeriesPlot;
