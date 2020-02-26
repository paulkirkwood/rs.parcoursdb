extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::tour_of_flanders::repository::*;
    use parcoursdb::classic::{BergClassic,CobbledClassic,Classic};

    #[test]
    fn test_tour_of_flanders_2018() {
        let edition = tour_of_flanders_2018();
        assert_eq!(edition.name(), "Tour of Flanders");
        assert_eq!(edition.start().qualified_place(), "Antwerp (Belgium)");
        assert_eq!(edition.finish().qualified_place(), "Oudenaarde (Belgium)");

        let cobbles = [
            "1,180.0 km,Lippenhovestraat,1.3 km",
            "2,178.0 km,Paddestraat,2.3 km",
            "3,125.0 km,Holleweg,0.35 km",
            "4,119.0 km,Haaghoek,2 km",
            "5,42.0 km,Mariaborrestraat,2 km"
        ];

        assert_eq!(edition.cobbles(), cobbles);

        let bergs = [
            "1,146.0 km,Oude-Kwaremont,2.2 km,Cobbles",
            "2,135.0 km,Kortkeer,1 km,Asphalt",
            "3,130.0 km,Edelare,1 km,Asphalt",
            "4,125.0 km,Wolvenberg,1.5 km,Cobbles",
            "5,116.0 km,Leberg,0.95 km,Asphalt",
            "6,112.0 km,Berendries,0.94 km,Asphalt",
            "7,107.0 km,Tenbosse,0.45 km,Asphalt",
            "8,97.0 km,Muur-Kapelmuur,0.75 km,Asphalt",
            "9,78.0 km,Pottelberg,1.5 km,Asphalt",
            "10,72.0 km,Kanarieberg,1 km,Asphalt",
            "11,56.0 km,Oude-Kwaremont,2.2 km,Cobbles",
            "12,53.0 km,Paterberg,0.36 km,Cobbles",
            "13,46.0 km,Koppenberg,0.6 km,Cobbles",
            "14,41.0 km,Steenbeekdries,0.6 km,Asphalt",
            "15,38.0 km,Taaienberg,0.5 km,Cobbles",
            "16,27.0 km,Kruisberg (Oudestraat),0.45 km,Cobbles",
            "17,17.0 km,Oude-Kwaremont,2.2 km,Cobbles",
            "18,14.0 km,Paterberg,0.36 km,Cobbles",
        ];

        assert_eq!(edition.bergs(), bergs);
    }
}
