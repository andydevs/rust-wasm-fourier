type Complex = num_complex::Complex<f64>;

pub struct PhasorArray {
    num_phasors: i32,
    phasors: Vec<Complex>
}

impl PhasorArray {
    pub fn frequencies(num_phasors: i32) -> impl Iterator<Item = i32> {
        let a = (1..num_phasors).flat_map(|i| { [i, -i] });
        let z = std::iter::once(0);
        z.chain(a)
    }

    pub fn fourier_series(num_phasors: i32, f: impl (Fn(f64) -> Complex)) -> Self {
        Self { 
            num_phasors: num_phasors, 
            phasors: Self::frequencies(num_phasors)
                .map(|i| { i as f64 })
                .map(f)
                .collect() 
        }
    }

    pub fn update(&mut self, dth: f64) {
        // determine rotors
        let base_rotor = (-dth*Complex::i()).exp();
        let rotors: Vec<Complex> = Self::frequencies(self.num_phasors)
            .map(|n| { base_rotor.powi(n) })
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