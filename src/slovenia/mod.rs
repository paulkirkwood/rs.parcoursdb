use crate::country::Country;
use crate::location::Location;

pub fn bled()      -> Location { Location::new("Bled".to_string(), Country::Slovenia, None) }
pub fn kranj()     -> Location { Location::new("Kranj".to_string(), Country::Slovenia, None) }
pub fn ljubljana() -> Location { Location::new("Ljubljana".to_string(), Country::Slovenia, None) }
