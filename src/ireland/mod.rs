use crate::country::Country;
use crate::location::Location;

pub fn cork() -> Location { irish_location("Cork") }

pub fn dublin() -> Location { irish_location("Dublin") }

pub fn enniscorthy() -> Location { irish_location("Enniscorthy") }

fn irish_location(name: &'static str) -> Location {
    Location::new(name.to_string(), Country::Ireland, None)
}
