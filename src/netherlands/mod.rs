use crate::country::Country;
use crate::location::Location;

pub fn arnhem() -> Location { dutch_location("Arnhem") }
pub fn amsterdam() -> Location { dutch_location("Amsterdam") }
pub fn apeldoorn() -> Location { dutch_location("Apeldoorn") }
pub fn assen() -> Location { dutch_location("Assen") }
pub fn berg_en_terblijt() -> Location { dutch_location("Berg en Terblijt") }
pub fn breda() -> Location { dutch_location("Breda") }
pub fn elsloo() -> Location { dutch_location("Elsloo") }
pub fn emmen() -> Location { dutch_location("Emmen") }
pub fn groningen() -> Location { dutch_location("Groningen") }
pub fn heerlen() -> Location { dutch_location("Heerlen") }
pub fn helmond() -> Location { dutch_location("Helmond") }
pub fn leiden() -> Location { dutch_location("Leiden") }
pub fn maastricht() -> Location { dutch_location("Maastricht") }
pub fn meerssen() -> Location { dutch_location("Meerssen") }
pub fn middleburg() -> Location { dutch_location("Middleburg") }
pub fn nijmegen() -> Location { dutch_location("Nijmegen") }
pub fn rotterdam() -> Location { dutch_location("Rotterdam") }
pub fn scheveningen() -> Location { dutch_location("Scheveningen") }
pub fn s_hertogenbosch() -> Location { dutch_location("'s-Hertogenbosch") }
pub fn sint_willebrord() -> Location { dutch_location("Sint Willebrord") }
pub fn utrecht() -> Location { dutch_location("Utrecht") }
pub fn valkenburg() -> Location { dutch_location("Valkenburg") }
pub fn venlo() -> Location { dutch_location("Venlo") }
pub fn zeeland() -> Location { dutch_location("Zeeland") }
pub fn zutphen() -> Location { dutch_location("Zutphen") }

fn dutch_location(name: &'static str) -> Location {
    Location::new(name.to_string(), Country::Netherlands, None)
}
