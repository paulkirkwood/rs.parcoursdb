use crate::country::Country;
use crate::location::Location;

pub fn andorran_col(name: &'static str, height: i32) -> Location {
    Location::new(name.to_string(), Country::Andorra, Some(height))
}

pub fn austrian(name: &'static str, height: i32) -> Location {
    Location::new(name.to_string(), Country::Austria, Some(height))
}

pub fn french_col(name: &'static str, height: i32) -> Location {
    Location::new(name.to_string(), Country::France, Some(height))
}

pub fn israeli_col(name: &'static str, height: i32) -> Location {
    Location::new(name.to_string(), Country::Israel, Some(height))
}

pub fn italian_col(name: &'static str, height: i32) -> Location {
    Location::new(name.to_string(), Country::Italy, Some(height))
}

pub fn spanish_col(name: &'static str, height: i32) -> Location {
    Location::new(name.to_string(), Country::Spain, Some(height))
}

pub fn swiss_col(name: &'static str, height: i32) -> Location {
    Location::new(name.to_string(), Country::Switzerland, Some(height))
}
