use std::ops::RangeInclusive;

use plotters::{backend::BitMapBackend, chart::{ChartBuilder, LabelAreaPosition}, drawing::IntoDrawingArea, element::Circle, series::LineSeries, style::{RGBColor, ShapeStyle, WHITE}};

use crate::Dots;



const RESERVE: f64 = 0.01; // 5%
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
        let left_series = self.series.iter().map(|el| *el.1.start()).min_by(|a, b| a.partial_cmp(b).unwrap());
        let left_dots =  self.dots.iter()
                                .map(|el| {
                                    el.0.iter()
                                        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap().0
                                })
                                .min_by(|a, b| a.partial_cmp(b).unwrap());

        let left = if left_dots.is_some() {left_dots.unwrap()} else {left_series.unwrap()};             
        let left = left - left.abs() * RESERVE;
        // println!("left: {left}");


        let right_series = self.series.iter().map(|el| *el.1.end()).max_by(|a, b| a.partial_cmp(b).unwrap());
        let right_dots = self.dots.iter()
                                .map(|el| {
                                    el.0.iter()
                                        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap().0
                                })
                                .max_by(|a, b| a.partial_cmp(b).unwrap());

        let right = if right_dots.is_some() {right_dots.unwrap()} else {right_series.unwrap()}; 
        let right = right + right.abs() * RESERVE;
        // println!("right: {right}");


    
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


        let bottom = 
            if let Some(btm) = 
                self.dots.iter()
                    .map(|el| {
                        el.0.iter()
                            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().1
                    })
                    .min_by(|a, b| a.partial_cmp(b).unwrap()) {
                        btm
            } else {
                series.iter()
                    .map(|series| {
                        series.0.iter().map(|el| el.1).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
                    })
                    .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
            };
        let bottom = bottom - bottom.abs() * RESERVE;
        // println!("bottom: {bottom}");

        let top = 
            if let Some(t) = 
                self.dots.iter()
                .map(|el| {
                    el.0.iter()
                        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().1
                })
                .max_by(|a, b| a.partial_cmp(b).unwrap()) {
                    t
            } else {
                series.iter()
                    .map(|series| {
                        series.0.iter().map(|el| el.1).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
                    })
                    .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
            };
        let top = top + top.abs() * RESERVE;
        // println!("top: {top}");
    
        
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
                    dots.iter().map(|dot| Circle::new(*dot, 3, ShapeStyle::from(color).filled()))
                ).unwrap();
                // ctx.draw_series(
                //     LineSeries::new(dots.iter().map(|el| *el), color)
                // ).unwrap();
            });
    }
}