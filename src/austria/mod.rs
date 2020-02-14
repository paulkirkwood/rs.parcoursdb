use crate::country::Country;
use crate::location::Location;

pub fn grossglockner() -> Location { austrian_location("Großglockner", Some(1908)) }

pub fn innsbruck() -> Location { austrian_location("Innsbruck", None) }

pub fn klagenfurt() -> Location { austrian_location("Klagenfurt", None) }

pub fn lienz() -> Location { austrian_location("Lienz", None) }

pub fn mayrhofen() -> Location { austrian_location("Mayrhofen", None) }

pub fn velden_am_worther() -> Location { austrian_location("Velden am Worther", None) }

fn austrian_location(name: &'static str, elevation: Option<i32>) -> Location {
    Location::new(name.to_string(), Country::Austria, elevation)
}
