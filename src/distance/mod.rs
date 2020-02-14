#[derive(Copy,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Distance {
    integral: u64,
    fractional: u64
}

impl Distance {
    pub fn new(i: u64, f: u64) -> Distance {
        Distance { integral: i, fractional: f }
    }

    pub fn integral(&self) -> u64 {
        self.integral
    }

    pub fn fractional(&self) -> u64 {
        self.fractional
    }

    pub fn to_f64(&self) -> f64 {
        format!("{}.{}", self.integral, self.fractional).parse().unwrap()
    }
}
