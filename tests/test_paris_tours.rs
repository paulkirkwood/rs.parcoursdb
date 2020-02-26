extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::paris_tours::repository::*;
    use parcoursdb::classic::{Classic,GravelClassic,HillyClassic};

    #[test]
    fn test_paris_tours_1896() {
        let edition = paris_tours_1896();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1901() {
        let edition = paris_tours_1901();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1906() {
        let edition = paris_tours_1906();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1907() {
        let edition = paris_tours_1907();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1908() {
        let edition = paris_tours_1908();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1909() {
        let edition = paris_tours_1909();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1910() {
        let edition = paris_tours_1910();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1911() {
        let edition = paris_tours_1911();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1912() {
        let edition = paris_tours_1912();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1913() {
        let edition = paris_tours_1913();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1914() {
        let edition = paris_tours_1914();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1917() {
        let edition = paris_tours_1917();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1918() {
        let edition = paris_tours_1918();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1919() {
        let edition = paris_tours_1919();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1920() {
        let edition = paris_tours_1920();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1921() {
        let edition = paris_tours_1921();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1922() {
        let edition = paris_tours_1922();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1923() {
        let edition = paris_tours_1923();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1924() {
        let edition = paris_tours_1924();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1925() {
        let edition = paris_tours_1925();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1926() {
        let edition = paris_tours_1926();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1927() {
        let edition = paris_tours_1927();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1928() {
        let edition = paris_tours_1928();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1929() {
        let edition = paris_tours_1929();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1930() {
        let edition = paris_tours_1930();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1931() {
        let edition = paris_tours_1931();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1932() {
        let edition = paris_tours_1932();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1933() {
        let edition = paris_tours_1933();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1934() {
        let edition = paris_tours_1934();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1935() {
        let edition = paris_tours_1935();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1936() {
        let edition = paris_tours_1936();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1937() {
        let edition = paris_tours_1937();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1938() {
        let edition = paris_tours_1938();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1939() {
        let edition = paris_tours_1939();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1941() {
        let edition = paris_tours_1941();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1942() {
        let edition = paris_tours_1942();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1943() {
        let edition = paris_tours_1943();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1944() {
        let edition = paris_tours_1944();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1945() {
        let edition = paris_tours_1945();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1946() {
        let edition = paris_tours_1946();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1947() {
        let edition = paris_tours_1947();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1948() {
        let edition = paris_tours_1948();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1949() {
        let edition = paris_tours_1949();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1950() {
        let edition = paris_tours_1950();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1951() {
        let edition = paris_tours_1951();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1952() {
        let edition = paris_tours_1952();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1953() {
        let edition = paris_tours_1953();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1954() {
        let edition = paris_tours_1954();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1955() {
        let edition = paris_tours_1955();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1956() {
        let edition = paris_tours_1956();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1957() {
        let edition = paris_tours_1957();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1958() {
        let edition = paris_tours_1958();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1959() {
        let edition = paris_tours_1959();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1960() {
        let edition = paris_tours_1960();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1961() {
        let edition = paris_tours_1961();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1962() {
        let edition = paris_tours_1962();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1963() {
        let edition = paris_tours_1963();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1964() {
        let edition = paris_tours_1964();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1965() {
        let edition = paris_tours_1965();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1966() {
        let edition = paris_tours_1966();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1967() {
        let edition = paris_tours_1967();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1968() {
        let edition = paris_tours_1968();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1969() {
        let edition = paris_tours_1969();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1970() {
        let edition = paris_tours_1970();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1971() {
        let edition = paris_tours_1971();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1972() {
        let edition = paris_tours_1972();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1973() {
        let edition = paris_tours_1973();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1974() {
        let edition = paris_tours_1974();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Tours (France)");
        assert_eq!(edition.finish().qualified_place(), "Versailles (France)");
    }

    #[test]
    fn test_paris_tours_1975() {
        let edition = paris_tours_1975();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Tours (France)");
        assert_eq!(edition.finish().qualified_place(), "Versailles (France)");
    }

    #[test]
    fn test_paris_tours_1976() {
        let edition = paris_tours_1976();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Tours (France)");
        assert_eq!(edition.finish().qualified_place(), "Versailles (France)");
    }

    #[test]
    fn test_paris_tours_1977() {
        let edition = paris_tours_1977();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Tours (France)");
        assert_eq!(edition.finish().qualified_place(), "Versailles (France)");
    }

    #[test]
    fn test_paris_tours_1978() {
        let edition = paris_tours_1978();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Blois (France)");
        assert_eq!(edition.finish().qualified_place(), "Montlhery (France)");
    }

    #[test]
    fn test_paris_tours_1979() {
        let edition = paris_tours_1979();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Blois (France)");
        assert_eq!(edition.finish().qualified_place(), "Chaville (France)");
    }

    #[test]
    fn test_paris_tours_1980() {
        let edition = paris_tours_1980();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Blois (France)");
        assert_eq!(edition.finish().qualified_place(), "Chaville (France)");
    }

    #[test]
    fn test_paris_tours_1981() {
        let edition = paris_tours_1981();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Blois (France)");
        assert_eq!(edition.finish().qualified_place(), "Chaville (France)");
    }

    #[test]
    fn test_paris_tours_1982() {
        let edition = paris_tours_1982();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Blois (France)");
        assert_eq!(edition.finish().qualified_place(), "Chaville (France)");
    }

    #[test]
    fn test_paris_tours_1983() {
        let edition = paris_tours_1983();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Blois (France)");
        assert_eq!(edition.finish().qualified_place(), "Chaville (France)");
    }

    #[test]
    fn test_paris_tours_1984() {
        let edition = paris_tours_1984();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Blois (France)");
        assert_eq!(edition.finish().qualified_place(), "Chaville (France)");
    }

    #[test]
    fn test_paris_tours_1985() {
        let edition = paris_tours_1985();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Creteil (France)");
        assert_eq!(edition.finish().qualified_place(), "Chaville (France)");
    }

    #[test]
    fn test_paris_tours_1986() {
        let edition = paris_tours_1986();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Creteil (France)");
        assert_eq!(edition.finish().qualified_place(), "Chaville (France)");
    }

    #[test]
    fn test_paris_tours_1987() {
        let edition = paris_tours_1987();
        assert_eq!(edition.name(), "Grand Prix d'Automne");
        assert_eq!(edition.start().qualified_place(), "Creteil (France)");
        assert_eq!(edition.finish().qualified_place(), "Chaville (France)");
    }

    #[test]
    fn test_paris_tours_1988() {
        let edition = paris_tours_1988();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Chaville (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1989() {
        let edition = paris_tours_1989();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Chaville (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1990() {
        let edition = paris_tours_1990();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Chaville (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1991() {
        let edition = paris_tours_1991();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1992() {
        let edition = paris_tours_1992();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1993() {
        let edition = paris_tours_1993();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1994() {
        let edition = paris_tours_1994();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1995() {
        let edition = paris_tours_1995();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1996() {
        let edition = paris_tours_1996();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1997() {
        let edition = paris_tours_1997();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1998() {
        let edition = paris_tours_1998();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_1999() {
        let edition = paris_tours_1999();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2000() {
        let edition = paris_tours_2000();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2001() {
        let edition = paris_tours_2001();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2002() {
        let edition = paris_tours_2002();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2003() {
        let edition = paris_tours_2003();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2004() {
        let edition = paris_tours_2004();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2005() {
        let edition = paris_tours_2005();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2006() {
        let edition = paris_tours_2006();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2007() {
        let edition = paris_tours_2007();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2008() {
        let edition = paris_tours_2008();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2009() {
        let edition = paris_tours_2009();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Chartres (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2010() {
        let edition = paris_tours_2010();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "La Loupe (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2011() {
        let edition = paris_tours_2011();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2012() {
        let edition = paris_tours_2012();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2013() {
        let edition = paris_tours_2013();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2014() {
        let edition = paris_tours_2014();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Saint-Arnoult-en-Yvelines (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2015() {
        let edition = paris_tours_2015();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2016() {
        let edition = paris_tours_2016();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2017() {
        let edition = paris_tours_2017();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Brou (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");
    }

    #[test]
    fn test_paris_tours_2018() {
        let edition = paris_tours_2018();
        assert_eq!(edition.name(), "Paris-Tours");
        assert_eq!(edition.start().qualified_place(), "Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Tours (France)");

        let cotes = [
            "61.0 km,Côte de Nazelles-Négron,103m",
            "39.5 km,Côte de Gogueme,104m",
            "35.0 km,Côte de Montfort,106m",
            "28.9 km,Côte de Rochère,92m",
            "22.5 km,Côte de la Vallée Chartier,95m",
            "17.0 km,Côte de Vouvray,91m",
            "12.0 km,Côte de Rochecorbon,101m"
        ];

        assert_eq!(edition.cotes(), cotes);

        let gravel = [
            "9,49.5 km,Château de Valmer,0.5 km",
            "8,48.0 km,Vallee de Raye,1.5 km",
            "7,39.5 km,Grosse Pierre,2.5 km",
            "6,35.0 km,Montfort,2.0 km",
            "5,30.5 km,Épinettes,1.2 km",
            "4,28.0 km,Coudraie,0.9 km",
            "3,25.0 km,Solidarité,1.5 km",
            "2,22.5 km,Peu Morier,1.6 km",
            "1,14.0 km,Rochecorbon,0.8 km"
        ];

        assert_eq!(edition.gravel(), gravel);
    }
}
