use crate::country::Country;
use crate::location::Location;

pub fn athens() -> Location { greek_location("Athens") }

pub fn eleusis() -> Location { greek_location("Eleusis") }

pub fn ioannina() -> Location { greek_location("Ioannina") }

pub fn missolonghi() -> Location { greek_location("Missolonghi") }

pub fn naupactus() -> Location { greek_location("Naupactus") }

fn greek_location(name: &'static str) -> Location { 
    Location::new(name.to_string(), Country::Greece, None)
}
