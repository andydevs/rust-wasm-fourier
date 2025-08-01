use std::f64::consts::PI;
type Complex = num_complex::Complex<f64>;
const I: Complex = Complex::I;

#[allow(dead_code)]
fn fourier_transform(n: f64, n_samples: usize, f: impl Fn(f64) -> Complex) -> Complex {
    let dt = 1. / (n_samples as f64);
    (0..n_samples).map(|s| { (s as f64) * dt })
        .map(|t| { f(t) * (-I*2.*PI*n*t).exp() * dt })
        .sum()
}

pub trait PathElement {
    fn fourier(&self, n_phasors: usize, n: f64) -> Complex;
}

#[derive(Clone)]
pub struct Line {
    pub z_0: Complex,
    pub z_1: Complex
}

impl PathElement for Line {
    fn fourier(&self, n_phasors: usize, n: f64) -> Complex {
        fourier_transform(n, n_phasors, |t| { 
            self.z_0*(1. - t) + self.z_1*t 
        })
    }
}

#[derive(Clone)]
pub struct CubicBezier {
    pub z_0: Complex,
    pub z_1: Complex,
    pub z_2: Complex,
    pub z_3: Complex,
}

impl PathElement for CubicBezier {
    fn fourier(&self, n_phasors: usize, n: f64) -> Complex {
        fourier_transform(n, n_phasors, |t| {
            let l_1 = self.z_0*(1. - t) + self.z_1*t;
            let l_2 = self.z_1*(1. - t) + self.z_2*t;
            let l_3 = self.z_2*(1. - t) + self.z_3*t;
            let q_1 = l_1*(1. - t) + l_2*t;
            let q_2 = l_2*(1. - t) + l_3*t;
            q_1*(1. - t) + q_2*t
        })
    }
}