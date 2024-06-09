use std::ops::RangeInclusive;

use plotters::{backend::BitMapBackend, chart::{ChartBuilder, LabelAreaPosition}, drawing::IntoDrawingArea, element::Circle, series::LineSeries, style::{RGBColor, ShapeStyle, WHITE}};

use crate::input::Dots;


const RESERVE: f64 = 0.05; // 5%
const ACCURACY: f64 = 100.;

pub struct Graph {
    series: Vec<(Box<dyn Fn(f64) -> f64>, RangeInclusive<f64>, RGBColor)>,
    dots: Vec<(Dots, RGBColor)>
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            series: Vec::new(),
            dots: Vec::new()
        }
    }

    pub fn draw_function(&mut self, f: Box<dyn Fn(f64) -> f64>, range: RangeInclusive<f64>, color: RGBColor) {
        self.series.push((f, range, color));
    }

    pub fn draw_dots(&mut self, dots: Dots, color: RGBColor) {
        self.dots.push((dots, color))
    }
}


impl Drop for Graph {
    fn drop(&mut self) {
        let left = self.series.iter().fold(f64::INFINITY, |a, (_, range, _)| a.min(*range.start()));
        let left = left - left.abs() * RESERVE;

        let right = self.series.iter().fold(f64::NEG_INFINITY, |a, (_, range, _)| a.max(*range.end()));
        let right = right + right.abs() * RESERVE;

    
        let drawing_area = BitMapBackend::new("graph.png", (1200, 800))
            .into_drawing_area();
    
        drawing_area.fill(&WHITE).unwrap();

        let series: Vec<_> = self.series.iter()
            .map(|(f, _, color)| {
                (
                ((left * ACCURACY) as i64..(right * ACCURACY) as i64)
                    .map(|x| x as f64 / ACCURACY)
                    .map(|x: f64| (x, f(x)))
                    .collect::<Vec<(f64, f64)>>(),
                color)
            }).collect();

        let bottom = series.clone().iter()
            .map(|series| {
                series.0.iter().fold(f64::INFINITY, |a, (_, b)| a.min(*b))
            })
            .fold(f64::INFINITY, |a, b| a.min(b));

        let bottom = bottom - bottom.abs() * RESERVE;

        let top = series.clone().iter()
            .map(|series| {
                series.0.iter().fold(f64::NEG_INFINITY, |a, (_, b)| a.max(*b))
            })
            .fold(f64::NEG_INFINITY, |a, b| a.max(b));

        let top = top + top.abs() * RESERVE;
        
        let mut ctx = ChartBuilder::on(&drawing_area)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(left..right, bottom..top)
            .unwrap();
    
        ctx.configure_mesh().draw().unwrap();

        series.iter()
            .for_each(|series| {
                ctx.draw_series(
                    LineSeries::new(series.0.iter().map(|el| *el), series.1)
                ).unwrap();
            });

        self.dots.iter()
            .for_each(|(dots, color)| {
                ctx.draw_series(
                    dots.iter().map(|dot| Circle::new(*dot, 5, ShapeStyle::from(color).filled()))
                ).unwrap();
            })
    }
}