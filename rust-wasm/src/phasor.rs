use wasm_bindgen::prelude::*;
use std::f64::consts::PI;
type Complex = num_complex::Complex<f64>;
const I: Complex = Complex::I;

const PHASOR_NUMBER: i32 = 10;
const NUM_SAMPLES: usize = 100;

pub struct PhasorArray {
    phasors: Vec<Complex>
}

impl PhasorArray {
    pub fn frequencies() -> impl Iterator<Item = f64> {
        let a = (1..PHASOR_NUMBER).flat_map(|i| { [i, -i] });
        let z = std::iter::once(0);
        z.chain(a).map(|i| { i as f64 })
    }

    pub fn build_from_map(f: impl (Fn(f64) -> Complex)) -> Self {
        Self { 
            phasors: Self::frequencies().map(f).collect()
        }
    }

    pub fn fourier_series(f: impl (Fn(f64) -> Complex)) -> Self {
        Self::build_from_map(|n| {
            // 'proximate it or something
            let dth = 2. * PI / (NUM_SAMPLES as f64);
            (0..NUM_SAMPLES).map(|i| { (i as f64) * dth })
                .map(|th| { f(th)*(-I*n*th).exp()*dth / (2. * PI) })
                .sum()
        })
    }

    pub fn update(&mut self, dth: f64) {
        // Determine rotors
        let rotors: Vec<Complex> = Self::frequencies()
            .map(|n| { (-n*dth*Complex::i()).exp() })
            .collect();

        // Rotate Phasors
        self.phasors = std::iter::zip(&self.phasors, rotors)
            .map(|(p, r)| { p * r })
            .collect();
    }

    pub fn iter(&self) -> impl Iterator<Item=&Complex> {
        self.phasors.iter()
    }
}