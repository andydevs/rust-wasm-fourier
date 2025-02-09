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

pub trait PathElem {
    fn fourier(&self, n_phasors: usize, n: f64) -> Complex;
}


#[derive(Clone)]
pub struct Line {
    pub z_0: Complex,
    pub z_1: Complex
}

impl PathElem for Line {
    fn fourier(&self, n_phasors: usize, n: f64) -> Complex {
        fourier_transform(n, n_phasors, |t| { 
            self.z_0*(1. - t) + self.z_1*t 
        })
    }
}