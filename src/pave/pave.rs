use std::fmt;

pub struct Pave {
    name: String,
    length: f64
}

impl Pave {

    pub fn new(name: String, length: f64) -> Pave {
        Pave { name: name, length: length }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn length(&self) -> &f64 {
        &self.length
    }
}

#[derive(Debug,Copy,Clone)]
pub enum PaveRating {
    OneStar,
    TwoStar,
    ThreeStar,
    FourStar,
    FiveStar
}

impl fmt::Display for PaveRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           PaveRating::OneStar   => write!(f, "*"),
           PaveRating::TwoStar   => write!(f, "**"),
           PaveRating::ThreeStar => write!(f, "***"),
           PaveRating::FourStar  => write!(f, "****"),
           PaveRating::FiveStar  => write!(f, "*****"),
       }
    }
}

pub struct ParisRoubaixPave {
    name: String,
    length: f64,
    rating: PaveRating
}

impl ParisRoubaixPave {

    pub fn new(name: String, length: f64, rating: PaveRating) -> ParisRoubaixPave {
        ParisRoubaixPave { name: name, length: length, rating: rating }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn length(&self) -> &f64 {
        &self.length
    }

    pub fn rating(&self) -> &PaveRating {
        &self.rating
    }
}
