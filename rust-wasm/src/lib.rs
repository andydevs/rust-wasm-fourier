mod utils;

// console.log
extern crate web_sys;
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

use wasm_bindgen::prelude::*;
use std::f64::consts::PI;
use js_sys::Math::random;

const PHASOR_NUMBER: usize = 25;
const FUNDAMENTAL_FREQ: f64 = 2.*PI / 5.;
const MAX_RAD: f64 = 100.0;

#[wasm_bindgen]
#[derive(Clone, Debug)]
struct Point {
    pub x: f64,
    pub y: f64
}

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
    phasors: Vec<Phasor>,
    arm_points: Vec<Point>
}

fn build_arm_from_phasors(phasors: &Vec<Phasor>) -> Vec<Point> {
    let arm = phasors.iter()
        .scan((0., 0.), |s, p| {
            let (x, y) = *s;
            let new_x = x + p.real;
            let new_y = y + p.imag;
            *s = (new_x, new_y);
            return Some(*s)
        })
        .map(|s| { Point { x: s.0, y: s.1 } })
        .collect();
    arm
}

#[wasm_bindgen]
impl PhasorAnimation {
    pub fn simple() -> Self {
        let phasors: Vec<Phasor> = (0..PHASOR_NUMBER)
            .map(|_i| { Phasor::from_polar(0.0, MAX_RAD) })
            .collect();
        let arm_points = build_arm_from_phasors(&phasors);
        Self { phasors: phasors, arm_points: arm_points }
    }
    pub fn randomized() -> Self {
        let phasors: Vec<Phasor> = (0..PHASOR_NUMBER)
            .map(|i| { 
                Phasor::from_polar(
                    2.*PI * random(), 
                    if i == 0 { 0.0 } else { MAX_RAD / (i as f64) }
                )
            })
            .collect();
        let arm_points = build_arm_from_phasors(&phasors);
        Self { phasors: phasors, arm_points: arm_points }
    }

    pub fn update(&mut self, dt: f64) {
        self.phasors = self.phasors.iter()
            .enumerate()
            .map(|(i, p)| { p.rotate(i as f64, dt) })
            .collect();
        self.arm_points = build_arm_from_phasors(&self.phasors);
    }

    pub fn get_arm_state(&self) -> Box<[Point]> {
        self.arm_points.clone().into_boxed_slice()
    }

    pub fn get_arm_radii(&self) -> Box<[f64]> {
        self.phasors.iter().map(Phasor::magnitude).collect::<Vec<f64>>().into_boxed_slice()
    }
}