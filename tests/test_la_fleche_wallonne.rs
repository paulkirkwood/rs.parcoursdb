extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::la_fleche_wallonne::repository::*;
    use parcoursdb::classic::Classic;

    #[test]
    fn test_la_fleche_wallonne_1974() {
        let edition = la_fleche_wallonne_1974();
        assert_eq!(edition.name(), "La Flèche Wallonne");
        assert_eq!(edition.start().qualified_place(), "Verviers (Belgium)");
        assert_eq!(edition.finish().qualified_place(), "Verviers (Belgium)");
    }

    #[test]
    fn test_la_fleche_wallonne_1975() {
        let edition = la_fleche_wallonne_1975();
        assert_eq!(edition.name(), "La Flèche Wallonne");
        assert_eq!(edition.start().qualified_place(), "Verviers (Belgium)");
        assert_eq!(edition.finish().qualified_place(), "Verviers (Belgium)");
    }
}
