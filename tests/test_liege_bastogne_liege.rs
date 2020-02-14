extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::liege_bastogne_liege::*;
    use parcoursdb::classic::{Classic,HillyClassic};

    #[test]
    fn test_liege_bastogne_liege_2018() {
        let edition = liege_bastogne_liege_2018();
        assert_eq!(edition.name(), "Liège-Bastogne-Liège");
        assert_eq!(edition.start().qualified_place(), "Liège (Belgium)");
        assert_eq!(edition.finish().qualified_place(), "Ans (Belgium)");

        let cotes = [
            "186.0 km,Cote de Bonnerue,493m",
            "149.0 km,Cote de Saint-Roch,456m",
            "106.0 km,Cote de Mont-le-Soie,558m",
            "90.0 km,Cote de Pont,443m",
            "86.0 km,Cote de Bellevaux,421m",
            "78.0 km,Cote de la Ferme Libert,502m",
            "60.0 km,Cote du Rosier,565m",
            "47.0 km,Col du Maquisard,367m",
            "35.5 km,Cote de la Redoute,314m",
            "19.0 km,Cote de la Roche-aux-Faucons,225m",
            "5.5 km,Cote de Saint-Nicolas,175m"
        ];

        assert_eq!(edition.cotes(), cotes);
    }
}
