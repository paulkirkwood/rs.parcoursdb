extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::e3_harelbeke::*;
    use parcoursdb::classic::Classic;

    #[test]
    fn test_e3_harelbeke_1958() {
         let edition = e3_harelbeke_1958();
         assert_eq!(edition.name(), "Harelbeke-Antwerp-Harelbeke");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1959() {
         let edition = e3_harelbeke_1959();
         assert_eq!(edition.name(), "Harelbeke-Antwerp-Harelbeke");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1960() {
         let edition = e3_harelbeke_1960();
         assert_eq!(edition.name(), "Harelbeke-Antwerp-Harelbeke");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1961() {
         let edition = e3_harelbeke_1961();
         assert_eq!(edition.name(), "Harelbeke-Antwerp-Harelbeke");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1962() {
         let edition = e3_harelbeke_1962();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1963() {
         let edition = e3_harelbeke_1963();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1964() {
         let edition = e3_harelbeke_1964();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1965() {
         let edition = e3_harelbeke_1965();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1966() {
         let edition = e3_harelbeke_1966();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1967() {
         let edition = e3_harelbeke_1967();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1968() {
         let edition = e3_harelbeke_1968();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1969() {
         let edition = e3_harelbeke_1969();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1970() {
         let edition = e3_harelbeke_1970();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1971() {
         let edition = e3_harelbeke_1971();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1972() {
         let edition = e3_harelbeke_1972();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1973() {
         let edition = e3_harelbeke_1973();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1974() {
         let edition = e3_harelbeke_1974();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1975() {
         let edition = e3_harelbeke_1975();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1976() {
         let edition = e3_harelbeke_1976();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1977() {
         let edition = e3_harelbeke_1977();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1978() {
         let edition = e3_harelbeke_1978();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1979() {
         let edition = e3_harelbeke_1979();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1980() {
         let edition = e3_harelbeke_1980();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1981() {
         let edition = e3_harelbeke_1981();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1982() {
         let edition = e3_harelbeke_1982();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1983() {
         let edition = e3_harelbeke_1983();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1984() {
         let edition = e3_harelbeke_1984();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1985() {
         let edition = e3_harelbeke_1985();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1986() {
         let edition = e3_harelbeke_1986();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1987() {
         let edition = e3_harelbeke_1987();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1988() {
         let edition = e3_harelbeke_1988();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1989() {
         let edition = e3_harelbeke_1989();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1990() {
         let edition = e3_harelbeke_1990();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1991() {
         let edition = e3_harelbeke_1991();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1992() {
         let edition = e3_harelbeke_1992();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1993() {
         let edition = e3_harelbeke_1993();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1994() {
         let edition = e3_harelbeke_1994();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1995() {
         let edition = e3_harelbeke_1995();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1996() {
         let edition = e3_harelbeke_1996();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1997() {
         let edition = e3_harelbeke_1997();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1998() {
         let edition = e3_harelbeke_1998();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_1999() {
         let edition = e3_harelbeke_1999();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2000() {
         let edition = e3_harelbeke_2000();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2001() {
         let edition = e3_harelbeke_2001();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2002() {
         let edition = e3_harelbeke_2002();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2003() {
         let edition = e3_harelbeke_2003();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2004() {
         let edition = e3_harelbeke_2004();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2005() {
         let edition = e3_harelbeke_2005();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2006() {
         let edition = e3_harelbeke_2006();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2007() {
         let edition = e3_harelbeke_2007();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2008() {
         let edition = e3_harelbeke_2008();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2009() {
         let edition = e3_harelbeke_2009();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2010() {
         let edition = e3_harelbeke_2010();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2011() {
         let edition = e3_harelbeke_2011();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2012() {
         let edition = e3_harelbeke_2012();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2013() {
         let edition = e3_harelbeke_2013();
         assert_eq!(edition.name(), "E3 Prijs Vlaanderen");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2014() {
         let edition = e3_harelbeke_2014();
         assert_eq!(edition.name(), "E3 Harelbeke");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2015() {
         let edition = e3_harelbeke_2015();
         assert_eq!(edition.name(), "E3 Harelbeke");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2016() {
         let edition = e3_harelbeke_2016();
         assert_eq!(edition.name(), "E3 Harelbeke");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2017() {
         let edition = e3_harelbeke_2017();
         assert_eq!(edition.name(), "E3 Harelbeke");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2018() {
         let edition = e3_harelbeke_2018();
         assert_eq!(edition.name(), "E3 Harelbeke");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }

    #[test]
    fn test_e3_harelbeke_2019() {
         let edition = e3_harelbeke_2019();
         assert_eq!(edition.name(), "E3 BinckBank Classic");
         assert_eq!(edition.start().qualified_place(), "Harelbeke (Belgium)");
         assert_eq!(edition.finish().qualified_place(), "Harelbeke (Belgium)");
    }
}
