use crate::classic::{ClassicBuilder,TourOfLombardy,TourOfLombardyBuilder};
use crate::distance::Distance;

pub fn tour_of_lombardy_2018() -> TourOfLombardy {

    TourOfLombardyBuilder::new(2018, 10, 13, 241.0)
        .cote("Colle Gallio".to_string(),         763, Distance::new(54,7))
        .cote("Colle Brianza".to_string(),        533, Distance::new(113,9))
        .cote("Madonna del Ghisallo".to_string(), 754, Distance::new(180,2))
        .cote("Colma di Sormano".to_string(),    1124, Distance::new(193,6))
        .cote("Civiglio".to_string(),             613, Distance::new(227,4))
        .build()
}
