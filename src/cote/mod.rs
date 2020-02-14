#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Cote {
    name: String,
    height: i32
}

impl Cote {
    pub fn new(n: String, h: i32) -> Cote {
        Cote { name: n, height: h }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}
