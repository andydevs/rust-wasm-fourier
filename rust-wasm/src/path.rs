mod element;

use wasm_bindgen::prelude::*;
use element::*;
use std::f64::consts::PI;
type Complex = num_complex::Complex<f64>;
const I: Complex = Complex::I;
use crate::phasor::PhasorArray;

#[wasm_bindgen]
pub struct Path {
    elements: Vec<Box<dyn PathElement>>,
    path_start: Complex,
    path_current: Complex,
}

#[wasm_bindgen]
impl Path {
    pub fn new() -> Self {
        Self { 
            elements: Vec::<Box<dyn PathElement>>::new(),
            path_start: Complex::new(0.0, 0.0),
            path_current: Complex::new(0.0, 0.0)
        }
    }

    fn add_element(&mut self, p: Box<dyn PathElement>) {
        self.elements.push(p);
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.path_start = Complex::new(x, y);
        self.path_current = Complex::new(x, y);
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        let z1 = Complex::new(x, y);
        let line = Line { z_0: self.path_current.clone(), z_1: z1.clone() };
        self.add_element(Box::new(line));
        self.path_current = z1;
    }

    pub fn curve_to(&mut self, x_0: f64, y_0: f64, x_1: f64, y_1: f64, x: f64, y: f64) {
        let z_0 = self.path_current.clone();
        let z_1 = Complex::new(x_0, y_0);
        let z_2 = Complex::new(x_1, y_1);
        let z_3 = Complex::new(x, y);
        self.path_current = z_3.clone();
        let curve = CubicBezier { z_0: z_0, z_1: z_1, z_2: z_2, z_3: z_3 };
        self.add_element(Box::new(curve));
    }

    pub fn close(&mut self) {
        let line = Line { z_0: self.path_current.clone(), z_1: self.path_start.clone() };
        self.add_element(Box::new(line));
        self.path_current = self.path_start.clone();
    }

    pub(crate) fn to_phasors(&self, n_phasors: usize) -> PhasorArray {
        PhasorArray::fourier_series(n_phasors as i32, |n| {
            let omega = 1. / (self.elements.len() as f64);
            self.elements.iter()
                .enumerate()
                .map(|(i, elem)| {
                    let k = i as f64;
                    let q = elem.fourier(n_phasors, n * omega);
                    omega * (-I*2.*PI*n*omega*k).exp() * q
                })
                .sum()
        })
    }
}