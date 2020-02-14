use std::fmt;

#[derive(Debug,Copy,Clone)]
pub enum Pavement {
    Asphalt,
    Cobbles,
}

impl fmt::Display for Pavement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Pavement::Asphalt => write!(f, "Asphalt"),
           Pavement::Cobbles => write!(f, "Cobbles"),
       }
    }
}

pub struct Berg {
    name: String,
    length: f64,
    pavement: Pavement
}

impl Berg {

    pub fn new(name: String, length: f64, pavement: Pavement) -> Berg {
        Berg { name: name, length: length, pavement: pavement }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn length(&self) -> &f64 {
        &self.length
    }

    pub fn pavement(&self) -> &Pavement {
        &self.pavement
    }
}
