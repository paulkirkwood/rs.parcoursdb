use crate::country::Country;
use crate::location::Location;

pub fn esch_sur_alzette() -> Location { luxembourg_location("Esch-sur-Alzette") }

pub fn luxembourg_city() -> Location { luxembourg_location("Luxembourg City") }

pub fn mondorf_les_bains() -> Location { luxembourg_location("Mondorf-les-Bains") }

fn luxembourg_location(name: &'static str) -> Location {
    Location::new(name.to_string(), Country::Luxembourg, None)
}
