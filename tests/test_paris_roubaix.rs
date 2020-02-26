extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::paris_roubaix::repository::*;
    use parcoursdb::classic::{CobbledClassic,Classic};

    #[test]
    fn test_paris_roubaix_1896() {
        let edition = paris_roubaix_1896();
        assert_eq!(edition.name(), "Paris-Roubaix");
        assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1897() {
        let edition = paris_roubaix_1897();
        assert_eq!(edition.name(), "Paris-Roubaix");
        assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
        assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1898() {
        let edition = paris_roubaix_1898();
        assert_eq!(edition.name(), "Paris-Roubaix");
        assert_eq!(edition.start().qualified_place(), "Chatou (France)");
        assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1899() {
        let edition = paris_roubaix_1899();
        assert_eq!(edition.name(), "Paris-Roubaix");
        assert_eq!(edition.start().qualified_place(), "Chatou (France)");
        assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1900() {
         let edition = paris_roubaix_1900();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Germain (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1901() {
         let edition = paris_roubaix_1901();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1902() {
         let edition = paris_roubaix_1902();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1903() {
         let edition = paris_roubaix_1903();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1904() {
         let edition = paris_roubaix_1904();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1905() {
         let edition = paris_roubaix_1905();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1906() {
         let edition = paris_roubaix_1906();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1907() {
         let edition = paris_roubaix_1907();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1908() {
         let edition = paris_roubaix_1908();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1909() {
         let edition = paris_roubaix_1909();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1910() {
         let edition = paris_roubaix_1910();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1911() {
         let edition = paris_roubaix_1911();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1912() {
         let edition = paris_roubaix_1912();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1913() {
         let edition = paris_roubaix_1913();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chatou (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1914() {
         let edition = paris_roubaix_1914();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1919() {
         let edition = paris_roubaix_1919();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1920() {
         let edition = paris_roubaix_1920();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1921() {
         let edition = paris_roubaix_1921();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1922() {
         let edition = paris_roubaix_1922();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1923() {
         let edition = paris_roubaix_1923();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1924() {
         let edition = paris_roubaix_1924();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1925() {
         let edition = paris_roubaix_1925();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1926() {
         let edition = paris_roubaix_1926();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1927() {
         let edition = paris_roubaix_1927();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1928() {
         let edition = paris_roubaix_1928();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Suresnes, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1929() {
         let edition = paris_roubaix_1929();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1930() {
         let edition = paris_roubaix_1930();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1931() {
         let edition = paris_roubaix_1931();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1932() {
         let edition = paris_roubaix_1932();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1933() {
         let edition = paris_roubaix_1933();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1934() {
         let edition = paris_roubaix_1934();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1935() {
         let edition = paris_roubaix_1935();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1936() {
         let edition = paris_roubaix_1936();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1937() {
         let edition = paris_roubaix_1937();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1938() {
         let edition = paris_roubaix_1938();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Argenteuil (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1939() {
         let edition = paris_roubaix_1939();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Porte Maillot, Paris (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1943() {
         let edition = paris_roubaix_1943();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1944() {
         let edition = paris_roubaix_1944();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1945() {
         let edition = paris_roubaix_1945();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1946() {
         let edition = paris_roubaix_1946();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1947() {
         let edition = paris_roubaix_1947();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1948() {
         let edition = paris_roubaix_1948();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1949() {
         let edition = paris_roubaix_1949();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1950() {
         let edition = paris_roubaix_1950();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1951() {
         let edition = paris_roubaix_1951();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1952() {
         let edition = paris_roubaix_1952();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1953() {
         let edition = paris_roubaix_1953();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1954() {
         let edition = paris_roubaix_1954();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1955() {
         let edition = paris_roubaix_1955();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1956() {
         let edition = paris_roubaix_1956();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1957() {
         let edition = paris_roubaix_1957();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1958() {
         let edition = paris_roubaix_1958();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1959() {
         let edition = paris_roubaix_1959();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1960() {
         let edition = paris_roubaix_1960();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1961() {
         let edition = paris_roubaix_1961();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1962() {
         let edition = paris_roubaix_1962();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1963() {
         let edition = paris_roubaix_1963();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1964() {
         let edition = paris_roubaix_1964();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1965() {
         let edition = paris_roubaix_1965();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Saint-Denis (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1966() {
         let edition = paris_roubaix_1966();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1967() {
         let edition = paris_roubaix_1967();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1968() {
         let edition = paris_roubaix_1968();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1969() {
         let edition = paris_roubaix_1969();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1970() {
         let edition = paris_roubaix_1970();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1971() {
         let edition = paris_roubaix_1971();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1972() {
         let edition = paris_roubaix_1972();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1973() {
         let edition = paris_roubaix_1973();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1974() {
         let edition = paris_roubaix_1974();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1975() {
         let edition = paris_roubaix_1975();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1976() {
         let edition = paris_roubaix_1976();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Chantilly (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1977() {
         let edition = paris_roubaix_1977();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1978() {
         let edition = paris_roubaix_1978();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1979() {
         let edition = paris_roubaix_1979();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1980() {
         let edition = paris_roubaix_1980();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1981() {
         let edition = paris_roubaix_1981();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1982() {
         let edition = paris_roubaix_1982();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1983() {
         let edition = paris_roubaix_1983();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1984() {
         let edition = paris_roubaix_1984();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1985() {
         let edition = paris_roubaix_1985();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1986() {
         let edition = paris_roubaix_1986();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1987() {
         let edition = paris_roubaix_1987();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1988() {
         let edition = paris_roubaix_1988();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1989() {
         let edition = paris_roubaix_1989();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1990() {
         let edition = paris_roubaix_1990();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1991() {
         let edition = paris_roubaix_1991();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1992() {
         let edition = paris_roubaix_1992();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1993() {
         let edition = paris_roubaix_1993();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1994() {
         let edition = paris_roubaix_1994();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1995() {
         let edition = paris_roubaix_1995();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1996() {
         let edition = paris_roubaix_1996();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1997() {
         let edition = paris_roubaix_1997();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1998() {
         let edition = paris_roubaix_1998();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_1999() {
         let edition = paris_roubaix_1999();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2000() {
         let edition = paris_roubaix_2000();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2001() {
         let edition = paris_roubaix_2001();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2002() {
         let edition = paris_roubaix_2002();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2003() {
         let edition = paris_roubaix_2003();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2004() {
         let edition = paris_roubaix_2004();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2005() {
         let edition = paris_roubaix_2005();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2006() {
         let edition = paris_roubaix_2006();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2007() {
         let edition = paris_roubaix_2007();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2008() {
         let edition = paris_roubaix_2008();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2009() {
         let edition = paris_roubaix_2009();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2010() {
         let edition = paris_roubaix_2010();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2011() {
         let edition = paris_roubaix_2011();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2012() {
         let edition = paris_roubaix_2012();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2013() {
         let edition = paris_roubaix_2013();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2014() {
         let edition = paris_roubaix_2014();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2015() {
         let edition = paris_roubaix_2015();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2016() {
         let edition = paris_roubaix_2016();
         assert_eq!(edition.name(), "Paris-Roubaix");
         assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
         assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");
    }

    #[test]
    fn test_paris_roubaix_2017() {
        let edition = paris_roubaix_2017();
        assert_eq!(edition.name(), "Paris-Roubaix");
        assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
        assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");

        let cobbles = [
            "29,160.0 km,Troisvilles à Inchy,2.2 km,***",
            "28,153.5 km,Viesly à Quiévy,1.8 km,***",
            "27,151.0 km,Quiévy à Saint Python,3.7 km,****",
            "26,144.5 km,Viesly à Biastre,3.0 km,***",
            "25,141.0 km,Biastre à Solesmes,0.8 km,**",
            "24,132.5 km,Vertain à Saint-Martin-sur-Ecaillon,2.3 km,***",
            "23,122.5 km,Verchain-Maugré à Quérénaing,1.6 km,***",
            "22,119.5 km,Quérénaing à Maing,2.5 km,***",
            "21,116.5 km,Maing à Monchaux-sur-Ecaillon,1.6 km,***",
            "20,103.5 km,Haveluy à Wallers,2.5 km,****",
            "19,95.5 km,Trouée d'Arenberg,2.4 km,*****",
            "18,89.0 km,Wallers à Hélesmes,1.6 km,***",
            "17,82.5 km,Hornaing à Wandignies,3.7 km,****",
            "16,75.0 km,Warlaing à Brillion,2.4 km,***",
            "15,71.5 km,Tilloy à Sars-et-Rosières,2.4 km,****",
            "14,65.0 km,Beuvry-la-Forêt à Orchies,1.4 km,***",
            "13,60.0 km,Orchies,1.7 km,***",
            "12,54.0 km,Auchy-lez-Orchies à Bersée,2.7 km,****",
            "11,48.5 km,Mons-en-Pévèle,3.0 km,*****",
            "10,42.5 km,Mérignies à Avelin,0.7 km,**",
            "9,39.0 km,Pont-Thibault à Ennevelin,1.4 km,***",
            "8,33.0 km,Templeuve,0.5 km,**",
            "7,26.5 km,Cysoing à Bourghelles,1.3 km,***",
            "6,24.0 km,Bourghelles à Wannehain,1.1 km,***",
            "5,19.5 km,Camphin-en-Pévèle,1.8 km,****",
            "4,17.0 km,Carrefour de l'Arbre,2.1 km,*****",
            "3,14.5 km,Grusson,1.1 km,**",
            "2,8.0 km,Willems à Hem,1.4 km,***",
            "1,1.0 km,Roubaix,0.3 km,*"
        ];

        assert_eq!(edition.cobbles(), cobbles);
    }

    #[test]
    fn test_paris_roubaix_2018() {
        let edition = paris_roubaix_2018();
        assert_eq!(edition.name(), "Paris-Roubaix");
        assert_eq!(edition.start().qualified_place(), "Compiegne (France)");
        assert_eq!(edition.finish().qualified_place(), "Roubaix (France)");

        let cobbles = [
            "29,163.5 km,Troisvilles,2.2 km,***",
            "28,157.0 km,Briastre,3.0 km,***",
            "27,148.0 km,Saint-Python,1.5 km,***",
            "26,145.5 km,Quiévy,3.7 km,****",
            "25,138.0 km,Saint-Vaast,1.5 km,***",
            "24,127.0 km,Verchain-Maugré,1.5 km,***",
            "23,122.5 km,Quérénaing,1.6 km,***",
            "22,119.5 km,Maing,2.5 km,***",
            "21,116.5 km,Monchaux-sur-Ecaillon,1.6 km,***",
            "20,103.5 km,Haveluy,2.5 km,****",
            "19,95.0 km,Trouée d'Arenberg,2.4 km,*****",
            "18,89.0 km,Hellesmes, dit Pont-Gibus,1.6 km,***",
            "17,82.5 km,Wandignies,3.7 km,****",
            "16,75.0 km,Brillon,2.4 km,***",
            "15,71.5 km,Sars-et-Rosières,2.4 km,****",
            "14,65.0 km,Beuvry-la-Forêt,1.4 km,***",
            "13,60.0 km,Orchies,1.7 km,***",
            "12,54.0 km,Bersée,2.7 km,****",
            "11,48.5 km,Mons-en-Pévèle,3.0 km,*****",
            "10,42.5 km,Mérignies à Avelin,0.7 km,**",
            "9,39.0 km,Pont-Thibaut,1.4 km,***",
            "8,33.5 km,Templeuve-L'Epinette,0.2 km,*",
            "8,33.0 km,Templeuve, Moulin de Vertain,0.5 km,**",
            "7,26.5 km,Cysoing à Bourghelles,1.3 km,***",
            "6,24.0 km,Bourghelles à Wannehain,1.1 km,***",
            "5,19.5 km,Camphin-en-Pévèle,1.8 km,*****",
            "4,17.0 km,Carrefour de l'Arbre,2.1 km,*****",
            "3,14.5 km,Grouson,1.1 km,**",
            "2,8.0 km,Hem,1.4 km,***",
            "1,1.0 km,Roubaix, espace Charles-Crupelandt,0.3 km,*"
        ];

        assert_eq!(edition.cobbles(), cobbles);
    }
}
