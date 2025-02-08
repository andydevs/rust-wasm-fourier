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
use std::{rc::Rc, cell::RefCell};
use std::f64::consts::PI;
use phasor::PhasorArray;
use arm::{ArmPoint, Arm};
type Complex = num_complex::Complex<f64>;
const I: Complex = Complex::I;

// 'proximate it or something
#[allow(dead_code)]
fn fourier_transform(n: f64, n_samples: usize, f: impl Fn(f64) -> Complex) -> Complex {
    let dt = 1. / (n_samples as f64);
    (0..n_samples).map(|s| { (s as f64) * dt })
        .map(|t| { f(t) * (-I*2.*PI*n*t).exp() * dt })
        .sum()
}

fn linear(z_0: &Complex, z_1: &Complex, n: f64) -> Complex {
    if n == 0.0 { 
        (z_0 + z_1) / 2.
    } else { 
        I*(z_1 - z_0) / (2.*PI*n)
    }
}

fn linear_integral(z_0: &Complex, z_1: &Complex, n: f64) -> Complex {
    fourier_transform(n, 40, |t| { z_0*(1. - t) + z_1*t })
}

#[wasm_bindgen]
pub struct PhasorAnim {
    phasors: Rc<RefCell<PhasorArray>>,
    arm: Arm
}

#[wasm_bindgen]
impl PhasorAnim {
    fn new(p: PhasorArray) -> Self {
        let prc = Rc::new(RefCell::new(p));
        let arm = Arm::new(&prc);
        PhasorAnim { phasors: prc, arm: arm }
    }
    
    pub fn line(num_phasors: i32, x_0: f64, y_0: f64, x_1: f64, y_1: f64, use_integral: bool) -> Self {
        let z_1 = Complex::new(x_1, y_1);
        let z_0 = Complex::new(x_0, y_0);
        let phasors = PhasorArray::fourier_series(num_phasors, |n| { 
            if use_integral {
                linear_integral(&z_0, &z_1, n)
            } else {
                linear(&z_0, &z_1, n)
            }
        });
        Self::new(phasors)
    }

    pub fn rectangle(num_phasors: i32, width: f64, height: f64, use_integral: bool) -> Self {
        let x = width / 2.;
        let y = height / 2.;
        let zs_0 = vec![
            Complex::new(x, -y),
            Complex::new(x, y),
            Complex::new(-x, y),
            Complex::new(-x, -y)
        ];
        let zs_1 = (0..zs_0.len())
            .map(|i| { zs_0[(i + 1) % zs_0.len()] })
            .collect::<Vec<Complex>>();
        let omega = 1. / (zs_0.len() as f64);
        let phasors = PhasorArray::fourier_series(num_phasors, |n| {
            std::iter::zip(&zs_0, &zs_1).enumerate()
                .map(|(i, (z_0, z_1))| {
                    let k = i as f64;
                    let q = if use_integral {
                        linear_integral(&z_0, &z_1, n * omega)
                    } else {
                        linear(&z_0, &z_1, n * omega)
                    };
                    omega * (-I*2.*PI*n*omega*k).exp()* q
                })
                .sum()
        });
        Self::new(phasors)
    }

    pub fn update(&mut self, dt: f64) {
        let mut p = self.phasors.borrow_mut();
        p.update(dt);
    }

    pub fn get_arm_state(&self, origin_x: f64, origin_y: f64) -> Vec<ArmPoint> {
        self.arm.get_state(origin_x, origin_y)
    }

    pub fn get_last_point(&mut self, origin_x: f64, origin_y: f64) -> ArmPoint {
        self.arm.get_last_point(origin_x, origin_y)
    }
}