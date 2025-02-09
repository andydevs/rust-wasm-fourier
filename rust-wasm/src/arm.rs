use wasm_bindgen::prelude::*;
use super::phasor::PhasorArray;
use std::{rc::Rc, cell::RefCell};
type Complex = num_complex::Complex<f64>;

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

#[wasm_bindgen]
pub struct Arm {
    #[wasm_bindgen(skip)]
    phasors: Rc<RefCell<PhasorArray>>
}

#[wasm_bindgen]
impl Arm {
    pub(crate) fn new(prc: &Rc<RefCell<PhasorArray>>) -> Self {
        Self { phasors: Rc::clone(&prc) }
    }

    fn origin_from_xy(&self, origin_x: f64, origin_y: f64) -> Complex {
        Complex::new(origin_x, origin_y)
    }
    
    pub fn get_state(&self, origin_x: f64, origin_y: f64) -> Vec<ArmPoint> {
        let p = self.phasors.borrow();
        let origin = self.origin_from_xy(origin_x, origin_y);
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
    
    pub fn get_last_point(&self, origin_x: f64, origin_y: f64) -> ArmPoint {
        let origin = self.origin_from_xy(origin_x, origin_y);
        self.phasors.borrow().iter().fold(origin, |c, p| { c + p }).into()
    }
}
