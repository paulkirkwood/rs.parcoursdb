use crate::country::Country;
use crate::location::Location;

pub fn armagh()  -> Location { Location::new("Armagh".to_string(), Country::NorthernIreland, None) }
pub fn belfast() -> Location { Location::new("Belfast".to_string(), Country::NorthernIreland, None) }
