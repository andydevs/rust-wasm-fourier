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


#[derive(Clone)]
pub struct Line {
    z_0: Complex,
    z_1: Complex
}

impl Line {
    pub fn from_xys(x_0: f64, y_0: f64, x_1: f64, y_1: f64) -> Self {
        Self { z_0: Complex::new(x_0, y_0), z_1: Complex::new(x_1, y_1) }
    }

    pub fn fourier(&self, n: f64) -> Complex {
        if n == 0.0 { 
            (self.z_0 + self.z_1) / 2.
        } else { 
            I*(self.z_1 - self.z_0) / (2.*PI*n)
        }
    }

    pub fn fourier_integral(&self, n: f64) -> Complex {
        fourier_transform(n, 40, |t| { 
            self.z_0*(1. - t) + self.z_1*t 
        })
    }
}

pub struct Composite {
    pub(crate) elements: Vec<Line>
}

impl Composite {
    pub fn fourier(&self, n: f64) -> Complex {
        let omega = 1. / (self.elements.len() as f64);
        self.elements.iter()
            .enumerate()
            .map(|(i, elem)| {
                let k = i as f64;
                let q = elem.fourier(n * omega);
                omega * (-I*2.*PI*n*omega*k).exp() * q
            })
            .sum()
    }

    pub fn fourier_integral(&self, n: f64) -> Complex {
        let omega = 1. / (self.elements.len() as f64);
        self.elements.iter()
            .enumerate()
            .map(|(i, elem)| {
                let k = i as f64;
                let q = elem.fourier_integral(n * omega);
                omega * (-I*2.*PI*n*omega*k).exp() * q
            })
            .sum()
    }
}