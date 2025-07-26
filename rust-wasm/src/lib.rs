mod utils;
mod phasor;
mod arm;
mod path;

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
use phasor::PhasorArray;
use arm::{Arm, ArmPoint};
use path::Path;

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

    pub fn from_path(num_phasors: usize, path: &Path) -> Self {
        let phasors = path.to_phasors(num_phasors);
        Self::new(phasors)
    }

    pub fn update(&mut self, dt: f64) {
        self.phasors.borrow_mut().update(dt);
    }

    pub fn get_arm_state(&self, origin_x: f64, origin_y: f64) -> Vec<ArmPoint> {
        self.arm.get_state(origin_x, origin_y)
    }

    pub fn get_last_point(&mut self, origin_x: f64, origin_y: f64) -> ArmPoint {
        self.arm.get_last_point(origin_x, origin_y)
    }
}