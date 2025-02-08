type Complex = num_complex::Complex<f64>;

pub struct PhasorArray {
    num_phasors: i32,
    phasors: Vec<Complex>
}

impl PhasorArray {
    pub fn frequencies(num_phasors: i32) -> impl Iterator<Item = f64> {
        let a = (1..num_phasors).flat_map(|i| { [i, -i] });
        let z = std::iter::once(0);
        z.chain(a).map(|i| { i as f64 })
    }

    pub fn fourier_series(num_phasors: i32, f: impl (Fn(f64) -> Complex)) -> Self {
        Self { 
            num_phasors: num_phasors, 
            phasors: Self::frequencies(num_phasors).map(f).collect() 
        }
    }

    pub fn update(&mut self, dth: f64) {
        // Determine rotors
        let rotors: Vec<Complex> = Self::frequencies(self.num_phasors)
            .map(|n| { (-n*dth*Complex::i()).exp() })
            .collect();

        // Rotate Phasors
        self.phasors = std::iter::zip(&self.phasors, rotors)
            .map(|(p, r)| { p * r })
            .collect();
    }

    pub fn iter(&self) -> impl Iterator<Item=&Complex> {
        self.phasors.iter()
    }
}