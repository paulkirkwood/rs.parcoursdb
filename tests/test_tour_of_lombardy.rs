extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::tour_of_lombardy::repository::*;
    use parcoursdb::classic::{Classic,HillyClassic};

    #[test]
    fn test_tour_of_lombardy_2018() {
        let edition = tour_of_lombardy_2018();
        assert_eq!(edition.name(), "Il Lombardia");
        assert_eq!(edition.start().qualified_place(), "Bergamo (Italy)");
        assert_eq!(edition.finish().qualified_place(), "Como (Italy)");

        let cotes = [
            "186.3 km,Colle Gallio,763m",
            "127.1 km,Colle Brianza,533m",
            "60.8 km,Madonna del Ghisallo,754m",
            "47.4 km,Colma di Sormano,1124m",
            "13.6 km,Civiglio,613m"
        ];

        assert_eq!(edition.cotes(), cotes);
    }
}
