extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::amstel_gold_race::repository::*;
    use parcoursdb::classic::Classic;

    #[test]
    fn test_amstel_gold_race_1966() {
         let edition = amstel_gold_race_1966();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Breda (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1967() {
         let edition = amstel_gold_race_1967();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Helmond (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Berg en Terblijt (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1968() {
         let edition = amstel_gold_race_1968();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Helmond (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Elsloo (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1969() {
         let edition = amstel_gold_race_1969();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Helmond (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1970() {
         let edition = amstel_gold_race_1970();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Helmond (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1971() {
         let edition = amstel_gold_race_1971();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1972() {
         let edition = amstel_gold_race_1972();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1973() {
         let edition = amstel_gold_race_1973();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1974() {
         let edition = amstel_gold_race_1974();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1975() {
         let edition = amstel_gold_race_1975();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1976() {
         let edition = amstel_gold_race_1976();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1977() {
         let edition = amstel_gold_race_1977();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1978() {
         let edition = amstel_gold_race_1978();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1979() {
         let edition = amstel_gold_race_1979();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1980() {
         let edition = amstel_gold_race_1980();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1981() {
         let edition = amstel_gold_race_1981();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1982() {
         let edition = amstel_gold_race_1982();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1983() {
         let edition = amstel_gold_race_1983();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1984() {
         let edition = amstel_gold_race_1984();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1985() {
         let edition = amstel_gold_race_1985();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1986() {
         let edition = amstel_gold_race_1986();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1987() {
         let edition = amstel_gold_race_1987();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1988() {
         let edition = amstel_gold_race_1988();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1989() {
         let edition = amstel_gold_race_1989();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1990() {
         let edition = amstel_gold_race_1990();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Meerssen (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1991() {
         let edition = amstel_gold_race_1991();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1992() {
         let edition = amstel_gold_race_1992();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1993() {
         let edition = amstel_gold_race_1993();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1994() {
         let edition = amstel_gold_race_1994();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1995() {
         let edition = amstel_gold_race_1995();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1996() {
         let edition = amstel_gold_race_1996();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1997() {
         let edition = amstel_gold_race_1997();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Heerlen (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1998() {
         let edition = amstel_gold_race_1998();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_1999() {
         let edition = amstel_gold_race_1999();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2000() {
         let edition = amstel_gold_race_2000();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2001() {
         let edition = amstel_gold_race_2001();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2002() {
         let edition = amstel_gold_race_2002();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Maastricht (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2003() {
         let edition = amstel_gold_race_2003();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Valkenburg (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2004() {
         let edition = amstel_gold_race_2004();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Valkenburg (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2005() {
         let edition = amstel_gold_race_2005();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Valkenburg (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2006() {
         let edition = amstel_gold_race_2006();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Valkenburg (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2007() {
         let edition = amstel_gold_race_2007();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Valkenburg (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2008() {
         let edition = amstel_gold_race_2008();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Valkenburg (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2009() {
         let edition = amstel_gold_race_2009();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Valkenburg (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2010() {
         let edition = amstel_gold_race_2010();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Valkenburg (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2011() {
         let edition = amstel_gold_race_2011();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Valkenburg (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2012() {
         let edition = amstel_gold_race_2012();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Valkenburg (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2013() {
         let edition = amstel_gold_race_2013();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Berg en Terblijt (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2014() {
         let edition = amstel_gold_race_2014();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Berg en Terblijt (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2015() {
         let edition = amstel_gold_race_2015();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Berg en Terblijt (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2016() {
         let edition = amstel_gold_race_2016();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Berg en Terblijt (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2017() {
         let edition = amstel_gold_race_2017();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Berg en Terblijt (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2018() {
         let edition = amstel_gold_race_2018();
         assert_eq!(edition.name(), "Amstel Gold Race");
         assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
         assert_eq!(edition.finish().qualified_place(), "Berg en Terblijt (Netherlands)");
    }

    #[test]
    fn test_amstel_gold_race_2019() {
        let edition = amstel_gold_race_2019();
        assert_eq!(edition.name(), "Amstel Gold Race");
        assert_eq!(edition.start().qualified_place(), "Maastricht (Netherlands)");
        assert_eq!(edition.finish().qualified_place(), "Berg en Terblijt (Netherlands)");
    }
}
