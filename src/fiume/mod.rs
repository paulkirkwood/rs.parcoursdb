use crate::country::Country;
use crate::location::Location;

pub fn fiume() -> Location { 
    Location::new("Fiume".to_string(), Country::Fiume, None)
}
