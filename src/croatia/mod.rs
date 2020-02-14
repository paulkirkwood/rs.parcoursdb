use crate::country::Country;
use crate::location::Location;

pub fn porec() -> Location { croatian_location("Porec") }

pub fn pula() -> Location { croatian_location("Pula") }

fn croatian_location(name: &'static str) -> Location {
    Location::new(name.to_string(), Country::Croatia, None)
}
