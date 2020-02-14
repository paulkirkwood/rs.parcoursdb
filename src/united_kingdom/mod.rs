use crate::country::Country;
use crate::location::Location;

pub fn brighton() -> Location { british_location("Brighton") }
pub fn cambridge() -> Location { british_location("Cambridge") }
pub fn canterbury() -> Location { british_location("Canterbury") }
pub fn dover() -> Location { british_location("Dover") }
pub fn harrogate() -> Location { british_location("Harrogate") } 
pub fn leeds() -> Location { british_location("Leeds") }
pub fn london() -> Location { british_location("London") }
pub fn plymouth() -> Location { british_location("Plymouth") }
pub fn portsmouth() -> Location { british_location("Portsmouth") }
pub fn sheffield() -> Location { british_location("Sheffield") }
pub fn york() -> Location { british_location("York") }

fn british_location(name: &'static str) -> Location {
    Location::new(name.to_string(), Country::UnitedKingdom, None)
}
