use crate::country::Country;
use crate::location::Location;

pub fn evora()     -> Location { portuguese_location("Evora") }
pub fn estoril()   -> Location { portuguese_location("Estoril") }
pub fn lisbon()    -> Location { portuguese_location("Lisbon") }
pub fn loule()     -> Location { portuguese_location("Loule") }
pub fn vilamoura() -> Location { portuguese_location("Vilamoura") }

fn portuguese_location(name: &'static str) -> Location {
    Location::new(name.to_string(), Country::Portugal, None)
}
