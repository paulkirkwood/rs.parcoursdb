extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::omloop_het_volk::repository::*;
    use parcoursdb::classic::{BergClassic,CobbledClassic,Classic};

    #[test]
    fn test_omloop_het_volk_2018() {
        let edition = omloop_het_volk_2018();
        assert_eq!(edition.name(), "Omloop Het Nieuwsblad");
        assert_eq!(edition.start().qualified_place(), "Ghent (Belgium)");
        assert_eq!(edition.finish().qualified_place(), "Ghent (Belgium)");

        let cobbles = [
            "1,103.9 km,Hulsepontweg,1.8 km",
            "2,99.7 km,Korte Aststraat,0.2 km",
            "3,98.2 km,Lange Aststraat,0.5 km",
            "4,89.4 km,Holleweg,0.2 km",
            "5,63.1 km,Volkegem,0.05 km",
            "6,61.4 km,Ruiterstraat,0.8 km",
            "7,58.7 km,Jagerji,0.8 km",
            "8,44.6 km,Haaghoek,2 km"
        ];

        assert_eq!(edition.cobbles(), cobbles);

        let bergs = [
            "1,131.0 km,Leberg,0.9 km,Asphalt",
            "2,98.0 km,Den Ast,0.5 km,Asphalt",
            "3,90.0 km,Katternburg,1.0 km,Cobbles",
            "4,78.0 km,Leberg,0.9 km,Asphalt",
            "5,65.0 km,Kokkerelle,1.2 km,Asphalt",
            "6,61.0 km,Wolvenberg,0.6 km,Asphalt",
            "7,52.0 km,Molenberg,0.5 km,Cobbles",
            "8,42.0 km,Leberg,0.9 km,Asphalt",
            "9,37.0 km,Berendries,0.9 km,Asphalt",
            "10,32.0 km,Valkenberg,0.5 km,Cobbles",
            "11,26.0 km,Tenbosse,0.5 km,Asphalt",
            "12,16.0 km,Muur van Geraardsbergen,0.5 km,Cobbles",
            "13,12.0 km,Borberg,1.0 km,Cobbles"
        ];

        assert_eq!(edition.bergs(), bergs);
    }
}
