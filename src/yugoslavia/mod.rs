use crate::country::Country;
use crate::location::Location;

pub fn ljubljana() -> Location { Location::new("Ljubljana".to_string(), Country::Yugoslavia, None) }
