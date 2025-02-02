mod utils;

use wasm_bindgen::prelude::*;
use std::f64::consts::PI;
use js_sys::Math::random;

const PHASOR_NUMBER: usize = 25;
const FUNDAMENTAL_FREQ: f64 = 2.*PI / 5.;
const MAX_RAD: f64 = 100.0;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
struct Phasor {
    pub real: f64,
    pub imag: f64
}

#[wasm_bindgen]
impl Phasor {
    pub fn from_polar(a: f64, m: f64) -> Self {
        let (s, c) = a.sin_cos();
        Self { real: c*m, imag: s*m }
    }

    pub fn magnitude(&self) -> f64 {
        (self.real*self.real + self.imag*self.imag).sqrt()
    }

    pub fn rotate(&self, n: f64, dt: f64) -> Self {
        let (s, c) = (-FUNDAMENTAL_FREQ*n*dt).sin_cos();
        let new_real = self.real*c - self.imag*s;
        let new_imag = self.real*s + self.imag*c;
        Self { real: new_real, imag: new_imag }
    }
}

#[wasm_bindgen]
struct PhasorAnimation {
    phasors: Vec<Phasor>
}

#[wasm_bindgen]
impl PhasorAnimation {
    pub fn randomized() -> Self {
        let phasors = (0..PHASOR_NUMBER)
            .map(|i| { 
                Phasor::from_polar(
                    2.*PI * random(), 
                    if i == 0 { 0.0 } else { MAX_RAD / (i as f64) }
                )
            })
            .collect();
        Self { phasors: phasors }
    }

    pub fn update(&mut self, dt: f64) {
        self.phasors = self.phasors.iter()
            .enumerate()
            .map(|(i, p)| { p.rotate(i as f64, dt) })
            .collect();
    }

    pub fn get_state(&self) -> Box<[Phasor]> {
        self.phasors.clone().into_boxed_slice()
    }
}