use crate::country::Country;
use crate::location::Location;

pub fn herning() -> Location { danish_location("Herning") }

pub fn horsens() -> Location { danish_location("Horsens") }

fn danish_location(name: &'static str) -> Location {
    Location::new(name.to_string(), Country::Denmark, None)
}
