mod utils;
mod phasor;
mod arm;
mod pathelem;
mod builder;

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
use builder::Builder;

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
        let mut builder = Builder::new();
        builder.line_xys(x_0, y_0, x_1, y_1);

        let phasors = builder.to_phasors(num_phasors, use_integral);
        Self::new(phasors)
    }

    pub fn rectangle(num_phasors: i32, width: f64, height: f64, use_integral: bool) -> Self {
        let x = width / 2.;
        let y = height / 2.;

        let mut builder = Builder::new();
        builder.line_xys( x, -y,  x,  y);
        builder.line_xys( x,  y, -x,  y);
        builder.line_xys(-x,  y, -x, -y);
        builder.line_xys(-x, -y,  x, -y);
        
        let phasors = builder.to_phasors(num_phasors, use_integral);
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