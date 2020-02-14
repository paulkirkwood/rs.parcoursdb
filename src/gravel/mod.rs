pub struct Gravel {
    name: String,
    length: f64
}

impl Gravel {
    pub fn new(n: String, l: f64) -> Gravel {
        Gravel { name: n, length: l }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn length(&self) -> &f64 {
        &self.length
    }
}
