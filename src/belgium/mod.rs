use crate::country::Country;
use crate::location::Location;

pub fn ans() -> Location { belgian_location("Ans") }

pub fn antwerp() -> Location { belgian_location("Antwerp") }

pub fn arlon() -> Location { belgian_location("Arlon") }

pub fn bastogne() -> Location { belgian_location("Bastogne") }

pub fn beauraing() -> Location { belgian_location("Beauraing") }

pub fn beringen() -> Location { belgian_location("Beringen") }

pub fn beveren() -> Location { belgian_location("Beveren") }

pub fn bornem() -> Location { belgian_location("Bornem") }

pub fn brasschaat() -> Location { belgian_location("Brasschaat") }

pub fn bruges() -> Location { belgian_location("Bruges") }

pub fn brussels() -> Location { belgian_location("Brussels") }

pub fn charleroi() -> Location { belgian_location("Charleroi") }

pub fn circuit_zolder() -> Location { belgian_location("Circuit Zolder") }

pub fn deinze() -> Location { belgian_location("Deinze") }

pub fn dinant() -> Location { belgian_location("Dinant") }

pub fn esneux() -> Location { belgian_location("Esneux") }

pub fn evergem() -> Location { belgian_location("Evergem") }

pub fn forest() -> Location { belgian_location( "Forest" ) }

pub fn gentbrugge() -> Location { belgian_location("Gentbrugge") }

pub fn ghent() -> Location { belgian_location("Ghent") }

pub fn harelbeke() -> Location { belgian_location("Harelbeke") }

pub fn hasselt() -> Location { belgian_location("Hasselt") }

pub fn herentals() -> Location { belgian_location("Herentals") }

pub fn hotton() -> Location { belgian_location("Hotton") }

pub fn huy() -> Location { belgian_location("Huy") }

pub fn jambes() -> Location { belgian_location("Jambes") }

pub fn leuven() -> Location { belgian_location("Leuven") }

pub fn liege() -> Location { belgian_location("Liège") }

pub fn lokeren() -> Location { belgian_location("Lokeren") }

pub fn marche_en_famenne() -> Location { belgian_location("Marche-en-Famenne") }

pub fn mariakerke() -> Location { belgian_location("Mariakerke") }

pub fn marcinelle() -> Location { belgian_location("Marcinelle") }

pub fn meerbeke() -> Location { belgian_location("Merebeke") }

pub fn molenbeek() -> Location { belgian_location("Molenbeek") }

pub fn mons() -> Location { belgian_location("Mons") }

pub fn mouscron() -> Location { belgian_location("Mouscron") }
 
pub fn namur() -> Location { belgian_location("Namur") }

pub fn oudenaarde() -> Location { belgian_location("Oudenaarde") }

pub fn perwez() -> Location { belgian_location("Perwez") }

pub fn rochefort() -> Location { belgian_location("Rochefort") }

pub fn rocourt() -> Location { belgian_location("Rocourt") }

pub fn seraing() -> Location { belgian_location("Seraing") }

pub fn sint_niklaas() -> Location { belgian_location("Sint-Niklaas") }

pub fn sint_pieters_woluwe() -> Location { belgian_location("Sint-Pieters-Woluwe") }

pub fn spa() -> Location { belgian_location("Spa") }

pub fn tournai() -> Location { belgian_location("Tournai") }

pub fn verviers() -> Location { belgian_location("Verviers") }

pub fn vise() -> Location { belgian_location("Vise") }

pub fn wanze() -> Location { belgian_location("Wanze") }

pub fn waregem() -> Location { belgian_location("Waregem") }

pub fn waterloo() -> Location { belgian_location("Waterloo") }

pub fn wetteren() -> Location { belgian_location("Wetteren") }

pub fn wevelgem() -> Location { belgian_location("Wevelgem") }

pub fn ypres() -> Location { belgian_location("Ypres") }

fn belgian_location(name: &'static str) -> Location {
    Location::new(name.to_string(), Country::Belgium, None)
}
