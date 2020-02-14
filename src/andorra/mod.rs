use crate::country::Country;
use crate::location::Location;

pub fn andorra_la_vella() -> Location { andorran_location("Andorra la Vella", None ) }

pub fn envalira() -> Location { andorran_location("Envalira", None ) }

pub fn estacio_de_pal() -> Location { andorran_location("Estacio de Pal", None ) }

pub fn estacio_d_esqui_d_ordino_alcalis() -> Location { andorran_location("Estacio d'Esqui d'Ordino-Alcalis", None ) }

pub fn escaldes_engordany() -> Location { andorran_location("Escaldes-Engordany", None ) }

pub fn naturlandia() -> Location { andorran_location("Naturlandia", None ) }

pub fn ordino() -> Location { andorran_location("Ordino", None ) }

pub fn pal() -> Location { andorran_location("Pal", None ) }

fn andorran_location(name: &'static str, elevation: Option<i32>) -> Location {
   Location::new(name.to_string(), Country::Andorra, elevation)
}
