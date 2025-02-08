use crate::phasor::PhasorArray;
use crate::pathelem::{Line, Composite};

pub struct Builder {
    elements: Vec<Line>
}

impl Builder {
    pub(crate) fn new() -> Self {
        Self { elements: Vec::<Line>::new() }
    }

    pub fn add_element(&mut self, l: Line) {
        self.elements.push(l);
    }

    pub fn line_xys(&mut self, x_0: f64, y_0: f64, x_1: f64, y_1: f64) {
        self.add_element(Line::from_xys(x_0, y_0, x_1, y_1));
    }

    pub fn to_composite(&self) -> Composite {
        Composite { elements: self.elements.clone() }
    }

    pub fn to_phasors(&self, n_phasors: i32, use_integral: bool) -> PhasorArray {
        PhasorArray::fourier_series(n_phasors, |n| {
            if use_integral {
                self.to_composite().fourier_integral(n)
            } else {
                self.to_composite().fourier(n)
            }
        })
    }
}