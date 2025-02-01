mod utils;

use wasm_bindgen::prelude::*;
use std::f64::consts::PI;
use js_sys::Math::random;

const PHASOR_NUMBER: usize = 10;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
struct Phasor {
    pub real: f64,
    pub imag: f64
}

#[wasm_bindgen]
impl Phasor {
    pub fn zero() -> Self {
        Self { real: 0., imag: 0. }
    }
    
    pub fn from_angle(a: f64) -> Self {
        let (c, s) = a.sin_cos();
        Self { real: s, imag: c }
    }

    pub fn scale(&self, n: f64) -> Self {
        Self { real: self.real*n, imag: self.imag*n }
    }

    pub fn abs(&self) -> f64 {
        (self.real*self.real + self.imag*self.imag).sqrt()
    }
}

#[wasm_bindgen]
struct PhasorAnimation {
    phasors: Vec<Phasor>
}

#[wasm_bindgen]
impl PhasorAnimation {
    pub fn randomized() -> Self {
        Self {
            phasors: (0..PHASOR_NUMBER)
                .map(|_i| { random() * 2. * PI })
                .map(|f| { Phasor::from_angle(f).scale(random()*100.) })
                .collect()
        }
    }

    pub fn get_state(&self) -> Box<[Phasor]> {
        self.phasors.clone().into_boxed_slice()
    }
}