use crate::country::Country;
use crate::location::Location;

pub fn basel() -> Location { swiss_location("Basel", None) } 
pub fn bern() -> Location { swiss_location("Bern", None) } 
pub fn crans_montana() -> Location { swiss_location("Crans-Montana", Some(1670)) }
pub fn fribourg() -> Location { swiss_location("Fribourg", None) } 
pub fn geneva() -> Location { swiss_location("Geneva", None) } 
pub fn la_chaux_de_fonds() -> Location { swiss_location("La Chaux-de-Fonds", None) } 
pub fn lausanne() -> Location { swiss_location("Lausanne", None) } 
pub fn lenzerheide() -> Location { swiss_location("Lenzerheide", None) } 
pub fn leukerbad() -> Location { swiss_location("Leukerbad", None) } 
pub fn locarno() -> Location { swiss_location("Locarno", None) } 
pub fn lugano() -> Location { swiss_location("Lugano", None) } 
pub fn martigny() -> Location { swiss_location("Martigny", None) } 
pub fn melide() -> Location { swiss_location("Melide", None) } 
pub fn mendrisio() -> Location { swiss_location("Mendrisio", None) } 
pub fn mohin() -> Location { swiss_location("Mohin", None) } 
pub fn neuchatel() -> Location { swiss_location("Neuchatel", None) } 
pub fn saas_fee() -> Location { swiss_location("Saas Fee", None) } 
pub fn saint_moritz() -> Location { swiss_location("Saint Moritz", Some(1822)) } 
pub fn sion() -> Location { swiss_location("Sion", None) } 
pub fn verbier() -> Location { swiss_location("Verbier", Some(1468)) } 
pub fn zurich() -> Location { swiss_location("Zurich", None) }

fn swiss_location(name: &'static str, elevation: Option<i32>) -> Location {
    Location::new(name.to_string(), Country::Switzerland, elevation)
}
