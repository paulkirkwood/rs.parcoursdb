use crate::country::Country;
use crate::location::Location;

pub fn beersheba() -> Location { israeli_location("Beersheba") }

pub fn eilat() -> Location { israeli_location("Eilat") }

pub fn haifa() -> Location { israeli_location("Haifa") }

pub fn jerusalem() -> Location { israeli_location("Jerusalem") }

pub fn tel_aviv() -> Location { israeli_location("Tel Aviv") }

fn israeli_location(name: &'static str) -> Location {
    Location::new(name.to_string(), Country::Israel, None)
}
