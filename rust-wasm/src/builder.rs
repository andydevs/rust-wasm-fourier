use wasm_bindgen::prelude::*;
use crate::phasor::PhasorArray;
use crate::pathelem::{PathElem, Line};
use std::f64::consts::PI;
type Complex = num_complex::Complex<f64>;
const I: Complex = Complex::I;

#[wasm_bindgen]
pub struct Builder {
    elements: Vec<Box<dyn PathElem>>,
    path_start: Complex,
    path_current: Complex,
}

#[wasm_bindgen]
impl Builder {
    pub fn new() -> Self {
        Self { 
            elements: Vec::<Box<dyn PathElem>>::new(),
            path_start: Complex::new(0.0, 0.0),
            path_current: Complex::new(0.0, 0.0)
        }
    }

    fn add_element(&mut self, p: Box<dyn PathElem>) {
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