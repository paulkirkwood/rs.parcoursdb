use crate::location::Location;
use std::fmt;

#[derive(Copy,Clone,PartialEq, Debug)]
pub enum ColCategory {
    HC,
    C1,
    C2,
    C3,
    C4,
    UC,
}

impl fmt::Display for ColCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            ColCategory::HC => write!(f, "HC"),
            ColCategory::C1 => write!(f, "C.1"),
            ColCategory::C2 => write!(f, "C.2"),
            ColCategory::C3 => write!(f, "C.3"),
            ColCategory::C4 => write!(f, "C.4"),
            ColCategory::UC => write!(f, "Uncategorised"),
        }
    }
}

pub struct Col {
    location: Location,
    category: ColCategory,
    length: Option<f64>,
    average_gradient: Option<f64>,
    maximum_gradient: Option<f64>
}

impl Col {
    pub fn new(loc: Location
              , category: ColCategory
              , length: Option<f64>
              , average_gradient: Option<f64>
              , maximum_gradient: Option<f64>) -> Col {
        Col { location: Location::new(loc.name().to_string(), *loc.country(), loc.elevation())
            , category: category
            , length: length
            , average_gradient: average_gradient
            , maximum_gradient: maximum_gradient
        }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn category(&self) -> ColCategory {
        self.category
    }

    pub fn length(&self) -> Option<f64> {
        self.length
    }

    pub fn average_gradient(&self) -> Option<f64> {
        self.average_gradient
    }

    pub fn maximum_gradient(&self) -> Option<f64> {
        self.maximum_gradient
    }

    pub fn clone(&self) -> Col {
        Col::new(self.location.clone(), self.category.clone(), self.length, self.average_gradient, self.maximum_gradient)
    }
}
