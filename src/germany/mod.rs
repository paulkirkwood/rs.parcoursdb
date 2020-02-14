use crate::country::Country;
use crate::location::Location;

pub fn cologne() -> Location { german_location("Cologne") }
pub fn dusseldorf() -> Location { german_location("Dusseldorf") }
pub fn freiburg() -> Location { german_location("Freiburg") }
pub fn koblenz() -> Location { german_location("Koblenz") }
pub fn munster() -> Location { german_location("Münster") }
pub fn saarbrucken() -> Location { german_location("Saarbrücken") }
pub fn pforzheim() -> Location { german_location("Pforzheim") }
pub fn karlsruhe() -> Location { german_location("Karlsruhe") }

fn german_location(name: &'static str) -> Location { 
    Location::new(name.to_string(), Country::Germany, None)
}
