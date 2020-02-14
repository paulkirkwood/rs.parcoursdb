use crate::country::Country;
use crate::location::Location;

pub fn monaco()      -> Location { Location::new("Monaco".to_string(), Country::Monaco, None) }
pub fn monte_carlo() -> Location { Location::new("Monte Carlo".to_string(), Country::Monaco, None) }
