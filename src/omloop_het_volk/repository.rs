use crate::classic::{ClassicBuilder,OmloopHetVolk,OmloopHetVolkBuilder};
use crate::distance::Distance;

pub fn omloop_het_volk_2018() -> OmloopHetVolk {

    let mut builder = OmloopHetVolkBuilder::new(2018,2,24,196.0);

    builder = builder.pave("Hulsepontweg".to_string(), 1.8, Distance::new(92,1));
    builder = builder.pave("Korte Aststraat".to_string(), 0.2, Distance::new(96,3));
    builder = builder.pave("Lange Aststraat".to_string(), 0.5, Distance::new(97,8));
    builder = builder.pave("Holleweg".to_string(), 0.2, Distance::new(106,6));
    builder = builder.pave("Volkegem".to_string(), 0.05, Distance::new(132,9));
    builder = builder.pave("Ruiterstraat".to_string(), 0.8, Distance::new(134,6));
    builder = builder.pave("Jagerji".to_string(), 0.8, Distance::new(137,3));
    builder = builder.pave("Haaghoek".to_string(), 2.0, Distance::new(151,4));

    builder = builder.asphalt_berg("Leberg".to_string(), 0.95, Distance::new(65,0));
    builder = builder.asphalt_berg("Den Ast".to_string(), 0.45, Distance::new(98,0));
    builder = builder.cobbled_berg("Katternburg".to_string(), 1.0, Distance::new(106,0));
    builder = builder.asphalt_berg("Leberg".to_string(), 0.95, Distance::new(118,0));
    builder = builder.asphalt_berg("Kokkerelle".to_string(), 1.2, Distance::new(131,0));
    builder = builder.asphalt_berg("Wolvenberg".to_string(), 0.645, Distance::new(135,0));
    builder = builder.cobbled_berg("Molenberg".to_string(), 0.463, Distance::new(144,0));
    builder = builder.asphalt_berg("Leberg".to_string(), 0.95, Distance::new(154,0));
    builder = builder.asphalt_berg("Berendries".to_string(), 0.94, Distance::new(159,0));
    builder = builder.cobbled_berg("Valkenberg".to_string(), 0.54, Distance::new(164,0));
    builder = builder.asphalt_berg("Tenbosse".to_string(), 0.45, Distance::new(170,0));
    builder = builder.cobbled_berg("Muur van Geraardsbergen".to_string(), 0.475, Distance::new(180,0));
    builder = builder.cobbled_berg("Borberg".to_string(), 0.98, Distance::new(184,0));

    builder.build()
}

pub fn all() -> Vec<OmloopHetVolk> {
    vec![ omloop_het_volk_2018() ]
}

pub fn get(year:i32) -> Option<OmloopHetVolk> {
    match year {
        2018 => Some(omloop_het_volk_2018()),
        _    => None,
    }
}
