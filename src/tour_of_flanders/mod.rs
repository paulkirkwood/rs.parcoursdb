use crate::classic::{ClassicBuilder,TourOfFlanders,TourOfFlandersBuilder};
use crate::distance::Distance;

const OUDE_KWAREMONT: &'static str = "Oude-Kwaremont";
const PATERBERG: &'static str = "Paterberg";

pub fn tour_of_flanders_2018() -> TourOfFlanders {

    let mut builder = TourOfFlandersBuilder::new(2018,4,1,267.0)
        .pave("Lippenhovestraat".to_string(), 1.3, Distance::new(87,0))
        .pave("Paddestraat".to_string(), 2.3, Distance::new(89,0))
        .pave("Holleweg".to_string(), 0.35, Distance::new(142,0))
        .pave("Haaghoek".to_string(), 2.0, Distance::new(148,0))
        .pave("Mariaborrestraat".to_string(), 2.0, Distance::new(225,0));

    /*
     * Three ascents of Oude-Kwaremont
     */
    builder = builder.cobbled_berg(OUDE_KWAREMONT.to_string(), 2.2, Distance::new(121,0));
    builder = builder.cobbled_berg(OUDE_KWAREMONT.to_string(), 2.2, Distance::new(211,0));
    builder = builder.cobbled_berg(OUDE_KWAREMONT.to_string(), 2.2, Distance::new(250,0));

    /*
     * Two ascents of the Paterberg
     */
    builder = builder.cobbled_berg(PATERBERG.to_string(), 0.36, Distance::new(214,0));
    builder = builder.cobbled_berg(PATERBERG.to_string(), 0.36, Distance::new(253,0));

    /*
     * One ascent of everything else
     */
    builder = builder.asphalt_berg("Kortkeer".to_string(),               1.0,  Distance::new(132,0));
    builder = builder.asphalt_berg("Edelare".to_string(),                1.0,  Distance::new(137,0));
    builder = builder.cobbled_berg("Wolvenberg".to_string(),             1.5,  Distance::new(142,0));
    builder = builder.asphalt_berg("Leberg".to_string(),                 0.95, Distance::new(151,0));
    builder = builder.asphalt_berg("Berendries".to_string(),             0.94, Distance::new(155,0));
    builder = builder.asphalt_berg("Tenbosse".to_string(),               0.45, Distance::new(160,0));
    builder = builder.asphalt_berg("Muur-Kapelmuur".to_string(),         0.75, Distance::new(170,0));
    builder = builder.asphalt_berg("Pottelberg".to_string(),             1.5,  Distance::new(189,0));
    builder = builder.asphalt_berg("Kanarieberg".to_string(),            1.0,  Distance::new(195,0));
    builder = builder.cobbled_berg("Koppenberg".to_string(),             0.6,  Distance::new(221,0));
    builder = builder.asphalt_berg("Steenbeekdries".to_string(),         0.6,  Distance::new(226,0));
    builder = builder.cobbled_berg("Taaienberg".to_string(),             0.5,  Distance::new(229,0));
    builder = builder.cobbled_berg("Kruisberg (Oudestraat)".to_string(), 0.45, Distance::new(240,0));

    builder.build()
}
