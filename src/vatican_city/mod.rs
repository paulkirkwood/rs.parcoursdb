use crate::country::Country;
use crate::location::Location;

pub fn vatican_city() -> Location { Location::new("Vatican City".to_string(), Country::VaticanCity, None) }
