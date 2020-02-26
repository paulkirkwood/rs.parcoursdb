use crate::classic::{ClassicBuilder,LaFlecheWallonne,LaFlecheWallonneBuilder};

pub fn la_fleche_wallonne_1974() -> LaFlecheWallonne {
    LaFlecheWallonneBuilder::new(1974, 9, 9, 120.0).build()
}

pub fn la_fleche_wallonne_1975() -> LaFlecheWallonne {
     LaFlecheWallonneBuilder::new(1975, 6, 30, 120.0).build()
}

pub fn la_fleche_wallonne_1976() -> LaFlecheWallonne {
    LaFlecheWallonneBuilder::new(1976, 5, 28, 168.0).build()
}

pub fn la_fleche_wallonne_1977() -> LaFlecheWallonne {
    LaFlecheWallonneBuilder::new(1977, 5, 13, 160.0).build()
}

pub fn all() -> Vec<LaFlecheWallonne> {
    vec![ la_fleche_wallonne_1974(), la_fleche_wallonne_1975(), la_fleche_wallonne_1976(), la_fleche_wallonne_1977() ]
}

pub fn get(year:i32) -> Option<LaFlecheWallonne> {
    match year {
        1974 => Some(la_fleche_wallonne_1974()),
        1975 => Some(la_fleche_wallonne_1975()),
        1976 => Some(la_fleche_wallonne_1976()),
        1977 => Some(la_fleche_wallonne_1977()),
        _    => None
    }
}
