mod utils;
mod phasor;
mod arm;

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
use phasor::PhasorArray;
use arm::{ArmPoint, get_arm_state, get_last_point};
type Complex = num_complex::Complex<f64>;
const I: Complex = Complex::I;

#[wasm_bindgen]
pub struct PhasorAnim {
    phasors: PhasorArray
}

#[wasm_bindgen]
impl PhasorAnim {
    fn fourier_series(f: impl (Fn(f64) -> Complex)) -> Self {
        Self { phasors: PhasorArray::fourier_series(f) }
    }

    pub fn line(x_0: f64, y_0: f64, x_1: f64, y_1: f64) -> Self {
        let z_1 = Complex::new(x_1, y_1);
        let z_0 = Complex::new(x_0, y_0);
        Self::fourier_series(|theta: f64| {
            let t = theta / (2.*PI);
            z_0 * (1. - t) + z_1 * t
        })
    }

    pub fn rectangle(width: f64, height: f64) -> Self {
        let x = width / 2.;
        let y = height / 2.;
        let zs = [
            Complex::new(x, -y),
            Complex::new(x, y),
            Complex::new(-x, y),
            Complex::new(-x, -y)
        ];
        let n_zs = 4;
        let omega: f64 = 2.*PI / (n_zs as f64);

        Self::fourier_series(|theta: f64| {
            let q = theta / omega;
            let k = q.floor() as usize;
            let t = q.fract();
            let z_a = zs[k];
            let z_b = zs[(k + 1) % n_zs];
            z_a * (1. - t) + z_b * t
        })
    }

    pub fn update(&mut self, dt: f64) {
        self.phasors.update(dt);
    }

    pub fn get_arm_state(&self, origin_x: f64, origin_y: f64) -> Vec<ArmPoint> {
        get_arm_state(&self.phasors, origin_x, origin_y)
    }

    pub fn get_last_point(&mut self, origin_x: f64, origin_y: f64) -> ArmPoint {
        get_last_point(&self.phasors, origin_x, origin_y)
    }
}