use crate::country::Country;
use crate::location::Location;

pub fn cologne()     -> Location { west_german_location("Cologne") } 
pub fn felsberg()    -> Location { west_german_location("Felsberg") } 
pub fn frankfurt()   -> Location { west_german_location("Frankfurt") } 
pub fn freiburg()    -> Location { west_german_location("Freiburg") } 
pub fn karlsruhe()   -> Location { west_german_location("Karlsruhe") } 
pub fn pforzheim()   -> Location { west_german_location("Pforzheim") } 
pub fn saarlouis()   -> Location { west_german_location("Saarlouis") } 
pub fn stuttgart()   -> Location { west_german_location("Stuttgart") } 
pub fn west_berlin() -> Location { west_german_location("West Berlin") } 
pub fn wiesbaden()   -> Location { west_german_location("Wiesbaden") }

fn west_german_location(name: &'static str) -> Location {
    Location::new(name.to_string(), Country::WestGermany, None)
}
