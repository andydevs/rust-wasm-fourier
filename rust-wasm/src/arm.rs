use wasm_bindgen::prelude::*;
use super::phasor::PhasorArray;
type Complex = num_complex::Complex<f64>;
const I: Complex = Complex::I;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct ArmPoint {
    pub x: f64,
    pub y: f64,
    pub r: f64
}

impl From<Complex> for ArmPoint {
    fn from(c: Complex) -> Self {
        Self { x: c.re, y: c.im, r: 0.0 }
    }
}

pub fn get_arm_state(p: &PhasorArray, origin_x: f64, origin_y: f64) -> Vec<ArmPoint> {
    let origin = Complex::new(origin_x, origin_y);
    let mut arm_xy: Vec<Complex> = p.iter()
        .scan(origin, |s, p| { *s = *s + p; Some(*s) })
        .collect();
    arm_xy.insert(0, origin);
    let mut arm_radii: Vec<f64> = p.iter()
        .map(|p| { p.norm() })
        .collect();
    arm_radii.push(0.0);
    std::iter::zip(arm_xy, arm_radii)
        .map(|(q, r)| { ArmPoint { x: q.re, y: q.im, r: r } })
        .collect()
}

pub fn get_last_point(p: &PhasorArray, origin_x: f64, origin_y: f64) -> ArmPoint {
    let origin = Complex::new(origin_x, origin_y);
    p.iter().fold(origin, |c, p| { c + p }).into()
}