use crate::classic::{ClassicBuilder,LiegeBastogneLiege,LiegeBastogneLiegeBuilder};
use crate::distance::Distance;

pub fn liege_bastogne_liege_2018() -> LiegeBastogneLiege {

    LiegeBastogneLiegeBuilder::new(2018, 4, 22, 258.0)
        .cote("Cote de Bonnerue".to_string(),             493, Distance::new(72,0))
        .cote("Cote de Saint-Roch".to_string(),           456, Distance::new(109,0))
        .cote("Cote de Mont-le-Soie".to_string(),         558, Distance::new(152,0))
        .cote("Cote de Pont".to_string(),                 443, Distance::new(168,0))
        .cote("Cote de Bellevaux".to_string(),            421, Distance::new(172,0))
        .cote("Cote de la Ferme Libert".to_string(),      502, Distance::new(180,0))
        .cote("Cote du Rosier".to_string(),               565, Distance::new(198,0))
        .cote("Col du Maquisard".to_string(),             367, Distance::new(211,0))
        .cote("Cote de la Redoute".to_string(),           314, Distance::new(222,5))
        .cote("Cote de la Roche-aux-Faucons".to_string(), 225, Distance::new(239,0))
        .cote("Cote de Saint-Nicolas".to_string(),        175, Distance::new(252,5))
        .build()
}

pub fn all() -> Vec<LiegeBastogneLiege> {
    vec![ liege_bastogne_liege_2018() ]
}

pub fn get(year:i32) -> Option<LiegeBastogneLiege> {
    match year {
        2018 => Some(liege_bastogne_liege_2018()),
        _    => None
    }
}
