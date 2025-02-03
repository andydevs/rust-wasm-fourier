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
type Complex = num_complex::Complex<f64>;
const I: Complex = Complex::I;

const PHASOR_NUMBER: i32 = 10;
const FUNDAMENTAL_FREQ: f64 = 1.;
const MAX_RAD: f64 = 50.0;
const TRAIL_MAX_POINTS: usize = 100;

#[wasm_bindgen]
#[derive(Clone, Debug)]
struct ArmPoint {
    pub x: f64,
    pub y: f64,
    pub r: f64
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
struct TrailPoint {
    pub x: f64,
    pub y: f64
}

impl From<Complex> for TrailPoint {
    fn from(q: Complex) -> Self {
        Self { x: q.re, y: q.im }
    }
}

impl std::ops::Add<TrailPoint> for TrailPoint {
    type Output = Self;

    fn add(self, other: TrailPoint) -> Self::Output {
        TrailPoint {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> std::ops::Add<&'a TrailPoint> for TrailPoint {
    type Output = TrailPoint;

    fn add(self, other: &TrailPoint) -> Self::Output {
        TrailPoint {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> std::ops::Add for &'a TrailPoint {
    type Output = TrailPoint;

    fn add(self, other: Self) -> Self::Output {
        TrailPoint {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Default for TrailPoint {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

struct Ring<T, const N: usize> {
    data: [T; N],
    cursor: usize,
}

impl<T: Default + Copy + Clone, const N: usize> Ring<T, N> {
    fn init() -> Self {
        Self {
            data: [T::default(); N],
            cursor: 0
        }
    }

    fn push(&mut self, t: T) {
        self.data[self.cursor] = t;
        self.cursor = (self.cursor + 1) % N;
    }
}

struct RingIterator<'t, T> {
    data: &'t [T],
    offset: usize,
    max: usize,
    index: usize
}

impl<'t, T> RingIterator<'t, T> {
    fn new(data: &'t [T], offset: usize, max: usize) -> Self {
        Self { data: data, offset: offset, max: max, index: 0 }
    }
}

impl<'t, T: Clone> Iterator for RingIterator<'t, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index > self.max { return None }
        let o = (self.index + self.offset) % self.max;
        let e: T = self.data[o].clone();
        self.index = self.index + 1;
        Some(e)
    }
}

impl<'t, T: Default + Copy + 't, const N: usize> IntoIterator for Ring<T, N> {
    type Item = T;
    type IntoIter = RingIterator<'t, T>;

    fn into_iter(self) -> Self::IntoIter {
        RingIterator::new(&self.data, self.cursor, N)
    }
}


#[wasm_bindgen]
struct PhasorAnimation {
    phasors: Vec<Complex>,
    trail: Ring<TrailPoint, { TRAIL_MAX_POINTS }>
}

#[wasm_bindgen]
impl PhasorAnimation {
    fn frequencies() -> impl Iterator<Item = f64> {
        let a = (1..PHASOR_NUMBER).flat_map(|i| { [i, -i] });
        let z = std::iter::once(0);
        z.chain(a).map(|i| { i as f64 })
    }
    
    fn build_from_map(f: impl (Fn(f64) -> Complex)) -> Self
    {
        Self { 
            phasors: PhasorAnimation::frequencies().map(f).collect(),
            trail: Ring::init()
        }
    }
    
    pub fn simple() -> Self {
        Self::build_from_map(|n| { 
            (PI/2.*I).exp() * safe_div!(MAX_RAD, n, 0.0)
        })
    }
    
    pub fn randomized() -> Self {
        Self::build_from_map(|n| { 
            (2.*PI * random() * I).exp() * safe_div!(MAX_RAD, n, 0.0)
        })
    }

    pub fn update(&mut self, dt: f64) {
        // Determine rotors
        let rotors: Vec<Complex> = PhasorAnimation::frequencies()
            .map(|n| { (-FUNDAMENTAL_FREQ*n*dt*Complex::i()).exp() })
            .collect();
        
        // Rotate Phasors
        self.phasors = std::iter::zip(&self.phasors, rotors)
            .map(|(p, r)| { p * r })
            .collect();

        // Get last point
        let trail_point: TrailPoint = self.phasors
            .iter().sum::<Complex>().into();

        // Append to trail
        self.trail.push(trail_point);
    }

    pub fn get_arm_state(&self, origin_x: f64, origin_y: f64) -> Vec<ArmPoint> {
        let origin = Complex::new(origin_x, origin_y);
        let mut arm_xy: Vec<Complex> = self.phasors.iter()
            .scan(origin, |s, p| { *s = *s + p; Some(*s) })
            .collect();
        arm_xy.insert(0, origin);
        let mut arm_radii: Vec<f64> = self.phasors.iter()
            .map(|p| { p.norm() })
            .collect();
        arm_radii.push(0.0);
        std::iter::zip(arm_xy, arm_radii)
            .map(|(q, r)| { ArmPoint { x: q.re, y: q.im, r: r } })
            .collect()
    }

    pub fn get_trail_state(&self, origin_x: f64, origin_y: f64) -> Vec<TrailPoint> {
        let origin = TrailPoint { x: origin_x, y: origin_y };
        self.trail.iter().map(|t| { &origin + t }).collect()
    }
}