mod utils;

// console.log
extern crate web_sys;
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

macro_rules! safe_div {
    ( $a:expr, $b:expr, $z:literal ) => {
        (if ($b == $z) { $a } else { $a / $b })
    }
}

use wasm_bindgen::prelude::*;
use std::f64::consts::PI;
use js_sys::Math::random;

const PHASOR_NUMBER: i32 = 24;
const FUNDAMENTAL_FREQ: f64 = 1.;
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
#[derive(Clone, Debug)]
struct ArmPoint {
    pub x: f64,
    pub y: f64,
    pub r: f64
}

#[wasm_bindgen]
struct PhasorAnimation {
    phasors: Vec<Phasor>,
}

#[wasm_bindgen]
impl PhasorAnimation {
    fn frequencies() -> impl Iterator<Item = i32> {
        let a = (1..PHASOR_NUMBER).flat_map(|i| { [i, -i] });
        let z = std::iter::once(0 as i32);
        z.chain(a)
    }
    
    fn build_from_map(f: impl (Fn(i32) -> Phasor)) -> Self
    {
        Self { phasors: PhasorAnimation::frequencies().map(f).collect() }
    }
    
    pub fn simple() -> Self {
        Self::build_from_map(|i| { 
            Phasor::from_polar(PI/2., safe_div!(MAX_RAD, (i as f64), 0.0))
        })
    }
    
    pub fn randomized() -> Self {
        Self::build_from_map(|i| { 
            Phasor::from_polar(2.*PI * random(), safe_div!(MAX_RAD, (i as f64), 0.0))
        })
    }

    pub fn update(&mut self, dt: f64) {
        self.phasors = PhasorAnimation::frequencies()
            .zip(self.phasors.iter())
            .map(|(n, p)| { p.rotate(n as f64, dt) })
            .collect();
    }

    pub fn get_arm_state(&self) -> Vec<ArmPoint> {
        self.phasors.iter()
            .scan((0., 0.), |s, p| {
                let (x, y) = *s;
                let new_x = x + p.real;
                let new_y = y + p.imag;
                *s = (new_x, new_y);
                return Some((*s, p.magnitude()))
            })
            .map(|(s, m)| { ArmPoint { x: s.0, y: s.1, r: m } })
            .collect()
    }
}