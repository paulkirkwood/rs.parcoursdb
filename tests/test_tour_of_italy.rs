extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::tour_of_italy::*;

    #[test]
    fn test_tour_of_italy_1909() {

        let route = [
            "1,13 May,Milano to Bologna,397.0 km,Road stage",
            "2,16 May,Bologna to Chieti,375.8 km,Road stage",
            "3,18 May,Chieti to Naples,242.8 km,Road stage",
            "4,20 May,Naples to Rome,228.1 km,Road stage",
            "5,23 May,Rome to Florence,346.5 km,Road stage",
            "6,25 May,Florence to Genoa,294.1 km,Road stage",
            "7,27 May,Genoa to Turin,354.9 km,Road stage",
            "8,30 May,Turin to Milano,206.0 km,Road stage"
        ];

        let edition = tour_of_italy_1909();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "8 stages");
    }

    #[test]
    fn test_tour_of_italy_1910() {
        let route = [
            "1,18 May,Milano to Udine,388.0 km,Road stage",
            "2,20 May,Udine to Bologna,322.4 km,Road stage",
            "3,22 May,Bologna to Teramo,345.2 km,Road stage",
            "4,24 May,Teramo to Naples,319.5 km,Road stage",
            "5,26 May,Naples to Rome,192.3 km,Road stage",
            "6,28 May,Rome to Florence,327.5 km,Road stage",
            "7,30 May,Florence to Genoa,263.5 km,Road stage",
            "8,1 June,Genoa to Mondovi,218.1 km,Road stage",
            "9,2 June,Mondovi to Turin,333.4 km,Road stage",
            "10,5 June,Turin to Milano,277.5 km,Road stage"
        ];

        let edition = tour_of_italy_1910();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "10 stages");
    }

    #[test]
    fn test_tour_of_italy_1911() {
        let route = [
            "1,15 May,Rome to Florence,394.1 km,Road stage",
            "2,17 May,Florence to Genoa,261.5 km,Road stage",
            "3,19 May,Genoa to Oneglia,274.9 km,Road stage",
            "4,21 May,Oneglia to Mondovi,190.3 km,Road stage",
            "5,23 May,Mondovi to Turin,302.0 km,Road stage",
            "6,25 May,Turin to Milano,236.2 km,Road stage",
            "7,27 May,Milano to Bologna,394.0 km,Road stage",
            "8,29 May,Bologna to Ancona,283.4 km,Road stage",
            "9,31 May,Ancona to Sulmona,218.7 km,Road stage",
            "10,2 June,Sulmona to Bari,363.1 km,Road stage",
            "11,4 June,Bari to Pompeii,345.2 km,Road stage",
            "12,6 June,Naples to Rome,266.9 km,Road stage"
        ];

        let edition = tour_of_italy_1911();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "12 stages");
    }

    #[test]
    fn test_tour_of_italy_1912() {
        let route = [
            "1,19 May,Milano to Padua,398.8 km,Road stage",
            "2,21 May,Padua to Bologna,328.8 km,Road stage",
            "3,23 May,Bologna to Pescara,362.5 km,Road stage",
            "4,25 May,Pescara to Rome,294.0 km,Road stage",
            "5,27 May,Rome to Florence,337.0 km,Road stage",
            "6,29 May,Florence to Genoa,267.5 km,Road stage",
            "7,31 May,Genoa to Turin,230.0 km,Road stage",
            "8,2 June,Turin to Milano,280.0 km,Road stage",
            "9,4 June,Milano to Bergamo,235.0 km,Road stage"
        ];

        let edition = tour_of_italy_1912();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "9 stages");
    }

    #[test]
    fn test_tour_of_italy_1913() {
        let route = [
            "1,6 May,Milano to Genoa,341.0 km,Road stage",
            "2,8 May,Genoa to Siena,332.0 km,Road stage",
            "3,10 May,Siena to Rome,317.9 km,Road stage",
            "4,12 May,Rome to Salerno,341.0 km,Road stage",
            "5,14 May,Salerno to Bari,295.6 km,Road stage",
            "6,16 May,Bari to Campobasso,256.0 km,Road stage",
            "7,18 May,Campobasso to Ascoli Piceno,313.2 km,Road stage",
            "8,20 May,Ascoli Piceno to Rovigo,413.8 km,Road stage",
            "9,22 May,Rovigo to Milano,321.5 km,Road stage"
        ];

        let edition = tour_of_italy_1913();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "9 stages");
    }

    #[test]
    fn test_tour_of_italy_1914() {
        let route = [
            "1,24 May,Milano to Cuneo,420.0 km,Road stage",
            "2,26 May,Cuneo to Lucca,340.5 km,Road stage",
            "3,28 May,Lucca to Rome,430.0 km,Road stage",
            "4,30 May,Rome to Avellino,365.4 km,Road stage",
            "5,1 June,Avellino to Bari,328.7 km,Road stage",
            "6,3 June,Bari to L'Aquila,428.0 km,Road stage",
            "7,5 June,L'Aquila to Lugo,429.1 km,Road stage",
            "8,7 June,Lugo to Milano,420.3 km,Road stage"
        ];

        let edition = tour_of_italy_1914();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "8 stages");
    }

    #[test]
    fn test_tour_of_italy_1919() {
        let route = [
            "1,21 May,Milano to Trento,302.8 km,Road stage",
            "2,23 May,Trento to Trieste,334.3 km,Road stage",
            "3,25 May,Trieste to Ferrara,282.0 km,Road stage",
            "4,27 May,Ferrara to Pescara,411.2 km,Road stage",
            "5,29 May,Pescara to Naples,312.5 km,Road stage",
            "6,31 May,Naples to Rome,203.8 km,Road stage",
            "7,2 June,Rome to Florence,350.8 km,Road stage",
            "8,4 June,Florence to Genoa,261.8 km,Road stage",
            "9,6 June,Genoa to Turin,248.0 km,Road stage",
            "10,8 June,Turin to Milano,277.0 km,Road stage"
        ];

        let edition = tour_of_italy_1919();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "10 stages");
    }

    #[test]
    fn test_tour_of_italy_1920() {
        let route = [
            "1,23 May,Milano to Turin,348.0 km,Road stage",
            "2,25 May,Turin to Lucca,378.0 km,Road stage",
            "3,27 May,Lucca to Rome,386.0 km,Road stage",
            "4,29 May,Rome to Chieti,234.0 km,Road stage",
            "5,31 May,Chieti to Macerata,236.0 km,Road stage",
            "6,2 June,Macerata to Bologna,282.0 km,Road stage",
            "7,4 June,Bologna to Trieste,349.0 km,Road stage",
            "8,6 June,Trieste to Milano,421.0 km,Road stage"
        ];

        let edition = tour_of_italy_1920();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "8 stages");
    }

    #[test]
    fn test_tour_of_italy_1921() {
        let route = [
            "1,25 May,Milano to Merano,333.0 km,Road stage",
            "2,27 May,Merano to Bologna,348.0 km,Road stage",
            "3,29 May,Bologna to Perugia,321.0 km,Road stage",
            "4,31 May,Perugia to Chieti,328.0 km,Road stage",
            "5,2 June,Chieti to Naples,264.0 km,Road stage",
            "6,4 June,Naples to Rome,299.0 km,Road stage",
            "7,6 June,Rome to Livorno,341.0 km,Road stage",
            "8,8 June,Livorno to Parma,242.0 km,Road stage",
            "9,10 June,Parma to Turin,320.0 km,Road stage",
            "10,12 June,Turin to Milano,305.0 km,Road stage"
        ];

        let edition = tour_of_italy_1921();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "10 stages");
    }

    #[test]
    fn test_tour_of_italy_1922() {
        let route = [
            "1,24 May,Milano to Padua,326.0 km,Road stage",
            "2,26 May,Padua to Portorose,268.0 km,Road stage",
            "3,28 May,Portorose to Bologna,375.0 km,Road stage",
            "4,30 May,Bologna to Pescara,367.0 km,Road stage",
            "5,1 June,Pescara to Naples,267.0 km,Road stage",
            "6,3 June,Naples to Rome,254.0 km,Road stage",
            "7,5 June,Rome to Florence,319.0 km,Road stage",
            "8,7 June,Florence to Santa Margherita Ligure,292.0 km,Road stage",
            "9,9 June,Genoa to Turin,277.0 km,Road stage",
            "10,11 June,Turin to Milano,348.0 km,Road stage"
        ];

        let edition = tour_of_italy_1922();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "10 stages");
    }

    #[test]
    fn test_tour_of_italy_1923() {
        let route = [
            "1,23 May,Milano to Turin,328.0 km,Road stage",
            "2,25 May,Turin to Genoa,312.9 km,Road stage",
            "3,27 May,Genoa to Florence,265.0 km,Road stage",
            "4,29 May,Florence to Rome,288.7 km,Road stage",
            "5,31 May,Rome to Naples,281.5 km,Road stage",
            "6,2 June,Naples to Chieti,283.1 km,Road stage",
            "7,4 June,Chieti to Bologna,383.0 km,Road stage",
            "8,6 June,Bologna to Trieste,362.2 km,Road stage",
            "9,8 June,Trieste to Mantua,357.0 km,Road stage",
            "10,10 June,Mantua to Milano,341.3 km,Road stage"
        ];

        let edition = tour_of_italy_1923();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "10 stages");
    }

    #[test]
    fn test_tour_of_italy_1924() {
        let route = [
            "1,10 May,Milano to Genoa,300.3 km,Road stage",
            "2,12 May,Genoa to Florence,307.9 km,Road stage",
            "3,14 May,Florence to Rome,284.4 km,Road stage",
            "4,16 May,Rome to Naples,249.3 km,Road stage",
            "5,18 May,Potenza to Taranto,265.3 km,Road stage",
            "6,20 May,Taranto to Foggia,230.3 km,Road stage",
            "7,22 May,Foggia to L'Aquila,304.3 km,Road stage",
            "8,24 May,L'Aquila to Perugia,296.0 km,Road stage",
            "9,26 May,Perugia to Bologna,280.7 km,Road stage",
            "10,28 May,Bologna to Fiume (Free State of Fiume),415.0 km,Road stage",
            "11,30 May,Fiume (Free State of Fiume) to Verona,366.5 km,Road stage",
            "12,1 June,Verona to Milano,313.0 km,Road stage"
        ];

        let edition = tour_of_italy_1924();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "12 stages");
    }

    #[test]
    fn test_tour_of_italy_1925() {
        let route = [
            "1,16 May,Milano to Turin,278.1 km,Road stage",
            "2,18 May,Turin to Arenzano,279.2 km,Road stage",
            "3,20 May,Arenzano to Pisa,315.0 km,Road stage",
            "4,22 May,Pisa to Rome,337.1 km,Road stage",
            "5,24 May,Rome to Naples,260.0 km,Road stage",
            "6,26 May,Naples to Bari,314.2 km,Road stage",
            "7,28 May,Bari to Benevento,234.9 km,Road stage",
            "8,30 May,Benevento to Sulmona,275.0 km,Road stage",
            "9,1 June,Sulmona to Arezzo,376.8 km,Road stage",
            "10,3 June,Arezzo to Forlì,224.3 km,Road stage",
            "11,5 June,Forlì to Verona,318.0 km,Road stage",
            "12,7 June,Verona to Milano,307.9 km,Road stage"
        ];

        let edition = tour_of_italy_1925();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "12 stages");
    }

    #[test]
    fn test_tour_of_italy_1926() {
        let route = [
            "1,15 May,Milano to Turin,275.0 km,Road stage",
            "2,17 May,Turin to Genoa,250.5 km,Road stage",
            "3,19 May,Genoa to Florence,312.0 km,Road stage",
            "4,21 May,Florence to Rome,287.2 km,Road stage",
            "5,23 May,Rome to Naples,232.1 km,Road stage",
            "6,25 May,Naples to Foggia,262.9 km,Road stage",
            "7,27 May,Foggia to Sulmona,250.8 km,Road stage",
            "8,29 May,Sulmona to Terni,266.5 km,Road stage",
            "9,31 May,Terni to Bologna,357.8 km,Road stage",
            "10,2 June,Bologna to Udine,355.2 km,Road stage",
            "11,4 June,Udine to Verona,291.7 km,Road stage",
            "12,6 June,Verona to Milano,288.0 km,Road stage"
        ];

        let edition = tour_of_italy_1926();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "12 stages");
    }

    #[test]
    fn test_tour_of_italy_1927() {
        let route = [
            "1,15 May,Milano to Turin,288.0 km,Road stage",
            "2,17 May,Turin to Reggio Emilia,321.0 km,Road stage",
            "3,19 May,Reggio Emilia to Lucca,207.0 km,Road stage",
            "4,21 May,Lucca to Grosseto,240.0 km,Road stage",
            "5,22 May,Grosseto to Rome,257.6 km,Road stage",
            "6,23 May,Rome to Naples,256.8 km,Road stage",
            "7,24 May,Naples to Avellino,153.4 km,Road stage",
            "8,26 May,Avellino to Bari,271.8 km,Road stage",
            "9,27 May,Bari to Campobasso,243.6 km,Road stage",
            "10,29 May,Campobasso to Pescara,220.2 km,Road stage",
            "11,30 May,Pescara to Pesaro,218.0 km,Road stage",
            "12,1 June,Pesaro to Treviso,305.6 km,Road stage",
            "13,2 June,Treviso to Trieste,208.2 km,Road stage",
            "14,4 June,Trieste to Verona,275.6 km,Road stage",
            "15,6 June,Verona to Milano,291.5 km,Road stage"
        ];

        let edition = tour_of_italy_1927();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_of_italy_1928() {
        let route = [
            "1,12 May,Milano to Trento,233.1 km,Road stage",
            "2,14 May,Trento to Forlì,312.6 km,Road stage",
            "3,16 May,Predappio to Arezzo,148.0 km,Road stage",
            "4,18 May,Arezzo to Sulmona,327.9 km,Road stage",
            "5,20 May,Sulmona to Foggia,254.6 km,Road stage",
            "6,22 May,Foggia to Naples,248.3 km,Road stage",
            "7,24 May,Naples to Rome,275.0 km,Road stage",
            "8,26 May,Rome to Pistoia,323.0 km,Road stage",
            "9,28 May,Pistoia to Modena,206.0 km,Road stage",
            "10,30 May,Modena to Genoa,270.0 km,Road stage",
            "11,1 June,Genoa to Turin,195.1 km,Road stage",
            "12,3 June,Turin to Milano,251.0 km,Road stage"
        ];

        let edition = tour_of_italy_1928();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "12 stages");
    }

    #[test]
    fn test_tour_of_italy_1929() {
        let route = [
            "1,19 May,Rome to Naples,235.0 km,Road stage",
            "2,21 May,Naples to Foggia,185.0 km,Road stage",
            "3,23 May,Foggia to Lecce,282.0 km,Road stage",
            "4,25 May,Lecce to Potenza,270.0 km,Road stage",
            "5,27 May,Potenza to Cosenza,264.0 km,Road stage",
            "6,29 May,Cosenza to Salerno,295.0 km,Road stage",
            "7,31 May,Salerno to Formia,220.0 km,Road stage",
            "8,2 June,Formia to Rome,198.0 km,Road stage",
            "9,3 June,Rome to Orvieto,120.0 km,Road stage",
            "10,4 June,Orvieto to Siena,150.0 km,Road stage",
            "11,5 June,Siena to La Spezia,192.0 km,Road stage",
            "12,7 June,La Spezia to Parma,135.0 km,Road stage",
            "13,8 June,Parma to Alessandria,152.0 km,Road stage",
            "14,9 June,Alessandria to Milano,216.0 km,Road stage"
        ];

        let edition = tour_of_italy_1929();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "14 stages");
    }

    #[test]
    fn test_tour_of_italy_1930() {
        let route = [
            "1,17 May,Messina to Catania,174.0 km,Road stage",
            "2,18 May,Catania to Palermo,280.0 km,Road stage",
            "3,20 May,Palermo to Messina,257.0 km,Road stage",
            "4,22 May,Reggio Calabria to Catanzaro,173.0 km,Road stage",
            "5,23 May,Catanzaro to Cosenza,118.0 km,Road stage",
            "6,25 May,Cosenza to Salerno,292.0 km,Road stage",
            "7,27 May,Salerno to Naples,180.0 km,Road stage",
            "8,28 May,Naples to Rome,247.0 km,Road stage",
            "9,30 May,Rome to Teramo,203.0 km,Road stage",
            "10,31 May,Teramo to Ancona,185.0 km,Road stage",
            "11,2 June,Ancona to Forlì,182.0 km,Road stage",
            "12,3 June,Forlì to Rovigo,188.0 km,Road stage",
            "13,5 June,Rovigo to Asiago,150.0 km,Road stage",
            "14,6 June,Asiago to Brescia,186.0 km,Road stage",
            "15,8 June,Brescia to Milano,280.0 km,Road stage"
        ];

        let edition = tour_of_italy_1930();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_of_italy_1931() {
        let route = [
            "1,10 May,Milano to Mantua,206.0 km,Road stage",
            "2,11 May,Mantua to Ravenna,216.0 km,Road stage",
            "3,13 May,Ravenna to Macerata,288.0 km,Road stage",
            "4,15 May,Macerata to Pescara,234.0 km,Road stage",
            "5,17 May,Pescara to Naples,282.0 km,Road stage",
            "6,19 May,Naples to Rome,256.0 km,Road stage",
            "7,21 May,Rome to Perugia,247.0 km,Road stage",
            "8,23 May,Perugia to Montecatini Terme,246.0 km,Road stage",
            "9,25 May,Montecatini Terme to Genoa,248.0 km,Road stage",
            "10,27 May,Genoa to Cuneo,263.0 km,Road stage",
            "11,29 May,Cuneo to Turin,252.0 km,Road stage",
            "12,31 May,Turin to Milano,263.0 km,Road stage"
        ];

        let edition = tour_of_italy_1931();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "12 stages");
    }

    #[test]
    fn test_tour_of_italy_1932() {
        let route = [
            "1,14 May,Milano to Vicenza,207.0 km,Road stage",
            "2,15 May,Vicenza to Udine,183.0 km,Road stage",
            "3,17 May,Udine to Ferrara,225.0 km,Road stage",
            "4,18 May,Ferrara to Rimini,215.0 km,Road stage",
            "5,20 May,Rimini to Teramo,286.0 km,Road stage",
            "6,22 May,Teramo to Lanciano,220.0 km,Road stage",
            "7,24 May,Lanciano to Foggia,280.0 km,Road stage",
            "8,26 May,Foggia to Naples,217.0 km,Road stage",
            "9,28 May,Naples to Rome,265.0 km,Road stage",
            "10,30 May,Rome to Florence,321.0 km,Road stage",
            "11,1 June,Florence to Genoa,276.0 km,Road stage",
            "12,3 June,Genoa to Turin,267.0 km,Road stage",
            "13,5 June,Turin to Milano,271.0 km,Road stage"
        ];

        let edition = tour_of_italy_1932();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "13 stages");
    }

    #[test]
    fn test_tour_of_italy_1933() {
        let route = [
            "1,6 May,Milano to Turin,169.0 km,Road stage",
            "2,7 May,Turin to Genoa,216.0 km,Road stage",
            "3,8 May,Genoa to Pisa,191.0 km,Road stage",
            ",9 May,Rest day",
            "4,10 May,Pisa to Florence,184.0 km,Road stage",
            "5,11 May,Florence to Grosseto,193.0 km,Road stage",
            "6,12 May,Grosseto to Rome,212.0 km,Road stage",
            ",13 May,Rest day",
            "7,14 May,Rome to Naples,228.0 km,Road stage",
            "8,15 May,Naples to Foggia,195.0 km,Road stage",
            ",16 May,Rest day",
            "9,17 May,Foggia to Chieti,248.0 km,Road stage",
            "10,18 May,Chieti to Ascoli Piceno,158.0 km,Road stage",
            ",19 May,Rest day",
            "11,20 May,Ascoli Piceno to Riccione,208.0 km,Road stage",
            "12,21 May,Riccione to Bologna,189.0 km,Road stage",
            "13,22 May,Bologna to Ferrara,62.0 km,Individual time trial",
            ",23 May,Rest day",
            "14,24 May,Ferrara to Udine,242.0 km,Road stage",
            "15,25 May,Udine to Bassano del Grappa,213.0 km,Road stage",
            "16,26 May,Bassano del Grappa to Bolzano,148.0 km,Road stage",
            ",27 May,Rest day",
            "17,28 May,Bolzano to Milano,284.0 km,Road stage"
        ];
        let edition = tour_of_italy_1933();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages");
    }

    #[test]
    fn test_tour_of_italy_1934() {
        let route = [
            "1,19 May,Milano to Turin,169.2 km,Road stage",
            "2,20 May,Turin to Genoa,206.5 km,Road stage",
            ",21 May,Rest day",
            "3,22 May,Genoa to Livorno,220.5 km,Road stage",
            "4,23 May,Livorno to Pisa,45.0 km,Individual time trial",
            "5,24 May,Pisa to Rome,333.0 km,Road stage",
            ",25 May,Rest day",
            "6,26 May,Rome to Naples,228.0 km,Road stage",
            "7,27 May,Naples to Bari,339.0 km,Road stage",
            ",28 May,Rest day",
            "8,29 May,Bari to Campobasso,245.0 km,Road stage",
            "9,30 May,Campobasso to Teramo,283.0 km,Road stage",
            "10,31 May,Teramo to Ancona,214.0 km,Road stage",
            ",1 June,Rest day",
            "11,2 June,Ancona to Rimini,213.0 km,Road stage",
            "12,3 June,Rimini to Florence,176.5 km,Road stage",
            "13,4 June,Florence to Bologna,120.0 km,Road stage",
            ",5 June,Rest day",
            "14,6 June,Bologna to Ferrara,59.0 km,Individual time trial",
            "15,7 June,Ferrara to Trieste,273.0 km,Road stage",
            "16,8 June,Trieste to Bassano del Grappa,273.0 km,Road stage",
            ",9 June,Rest day",
            "17,10 June,Bassano del Grappa to Milano,315.0 km,Road stage"
        ];
        let edition = tour_of_italy_1934();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages");
    }

    #[test]
    fn test_tour_of_italy_1935() {
        let route = [
            "1,18 May,Milano to Cremona,165.0 km,Road stage",
            "2,19 May,Cremona to Mantua,175.0 km,Road stage",
            "3,20 May,Mantua to Rovigo,162.0 km,Road stage",
            "4,21 May,Rovigo to Cesenatico,140.0 km,Road stage",
            "5a,22 May,Cesena to Riccione,35.0 km,Individual time trial",
            "5b,22 May,Riccione to Portocivitanova,136.0 km,Road stage",
            ",23 May,Rest day",
            "6,24 May,Portocivitanova to L'Aquila,171.0 km,Road stage",
            "7,25 May,L'Aquila to Lanciano,146.0 km,Road stage",
            "8,26 May,Lanciano to Bari,308.0 km,Road stage",
            ",27 May,Rest day",
            "9,28 May,Bari to Naples,333.0 km,Road stage",
            ",29 May,Rest day",
            "10,30 May,Naples to Rome,250.0 km,Road stage",
            "11,31 May,Rome to Florence,317.0 km,Road stage",
            ",1 June,Rest day",
            "12,2 June,Florence to Montecatini Terme,134.0 km,Road stage",
            "13a,3 June,Montecatini Terme to Lucca,99.0 km,Road stage",
            "13b,3 June,Lucca to Viareggio,55.0 km,Individual time trial",
            "14,4 June,Viareggio to Genoa,172.0 km,Road stage",
            ",5 June,Rest day",
            "15,6 June,Genoa to Cuneo,148.0 km,Road stage",
            "16,7 June,Cuneo to Asti,91.0 km,Road stage",
            "17,8 June,Asti to Turin,250.0 km,Road stage",
            "18,9 June,Turin to Milano,290.0 km,Road stage"
        ];
        let edition = tour_of_italy_1935();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "18 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_italy_1936() {
        let route = [
            "1,16 May,Milano to Turin,161.0 km,Road stage",
            "2,17 May,Turin to Genoa,206.0 km,Road stage",
            "3,18 May,Genoa to Montecatini Terme,206.0 km,Road stage",
            ",19 May,Rest day",
            "4,20 May,Montecatini Terme to Grosseto,220.0 km,Road stage",
            "5,21 May,Grosseto to Rome,248.0 km,Road stage",
            ",22 May,Rest day",
            "6,23 May,Rome to Naples,230.0 km,Road stage",
            "7,24 May,Naples to Bari,283.0 km,Road stage",
            ",25 May,Rest day",
            "8,26 May,Bari to Campobasso,243.0 km,Road stage",
            "9,27 May,Campobasso to L'Aquila,204.0 km,Road stage",
            "10,28 May,L'Aquila to Rieti,117.0 km,Road stage",
            "11,29 May,Rieti to Monte Terminillo,20.0 km,Individual time trial",
            "12,30 May,Rieti to Florence,292.0 km,Road stage",
            "13,31 May,Florence to Cesenatico,139.0 km,Road stage",
            "14,1 June,Cesenatico to Ferrara,155.0 km,Road stage",
            "15a,2 June,Ferrara to Padua,106.0 km,Road stage",
            "15b,2 June,Padua to Venice,39.0 km,Individual time trial",
            ",3 June,Rest day",
            "16,4 June,Venice to Legnago,183.0 km,Road stage",
            "17a,5 June,Legnago to Riva del Garda,139.0 km,Road stage",
            "17b,5 June,Riva del Garda to Gardone Riviera,100.0 km,Road stage",
            "18,6 June,Gardone Riviera to Salsomaggiore Terme,206.0 km,Road stage",
            "19,7 June,Salsomaggiore Terme to Milano,248.0 km,Road stage"
        ];
        let edition = tour_of_italy_1936();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_italy_1937() {
        let route = [
            "1,8 May,Milano to Turin,165.0 km,Road stage",
            "2,9 May,Turin to Acqui Terme,148.0 km,Road stage",
            "3,10 May,Acqui Terme to Genoa,158.0 km,Road stage",
            "4,11 May,Genoa to Viareggio,186.0 km,Road stage",
            "5a,12 May,Viareggio to Marina di Massa,60.0 km,Team time trial",
            "5b,12 May,Marina di Massa to Livorno,114.0 km,Road stage",
            ",13 May,Rest day",
            "6,14 May,Livorno to Arezzo,190.0 km,Road stage",
            "7,15 May,Arezzo to Rieti,206.0 km,Road stage",
            "8a,16 May,Rieti to Monte Terminillo,20.0 km,Individual time trial",
            "8b,16 May,Rieti to Rome,152.0 km,Road stage",
            "9,17 May,Rome to Naples,250.0 km,Road stage",
            ",18 May,Rest day",
            "10,19 May,Naples to Foggia,166.0 km,Road stage",
            "11a,20 May,Foggia to San Severo,186.0 km,Road stage",
            "11b,20 May,San Severo to Campobasso,105.0 km,Road stage",
            "12,21 May,Campobasso to Pescara,258.0 km,Road stage",
            "13,22 May,Pescara to Ancona,194.0 km,Road stage",
            "14,23 May,Ancona to Forlì,178.0 km,Road stage",
            ",24 May,Rest day",
            "15,25 May,Forlì to Vittorio Veneto,266.0 km,Road stage",
            "16,26 May,Vittorio Veneto to Merano,227.0 km,Road stage",
            "17,27 May,Merano to Gardone Riviera,190.0 km,Road stage",
            ",28 May,Rest day",
            "18,29 May,Gardone Riviera to San Pellegrino Terme,129.0 km,Road stage",
            "19a,30 May,San Pellegrino Terme to Como,151.0 km,Road stage",
            "19b,30 May,Como to Milano,141.0 km,Road stage"
        ];
        let edition = tour_of_italy_1937();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages including 4 split stages");
    }

    #[test]
    fn test_tour_of_italy_1938() {
        let route = [
            "1,7 May,Milano to Turin,182.0 km,Road stage",
            "2,8 May,Turin to San Remo,204.0 km,Road stage",
            "3,9 May,San Remo to Santa Margherita Ligure,172.0 km,Road stage",
            "4a,10 May,Santa Margherita Ligure to La Spezia,81.0 km,Road stage",
            "4b,10 May,La Spezia to Montecatini Terme,110.0 km,Road stage",
            ",11 May,Rest day",
            "5,12 May,Montecatini Terme to Chianciano Terme,184.0 km,Road stage",
            "6,13 May,Chianciano Terme to Rieti,160.0 km,Road stage",
            "7a,14 May,Rieti to Monte Terminillo,19.8 km,Individual time trial",
            "7b,14 May,Rieti to Rome,152.0 km,Road stage",
            "8,15 May,Rome to Naples,234.0 km,Road stage",
            ",16 May,Rest day",
            "9,17 May,Naples to Lanciano,221.0 km,Road stage",
            "10,18 May,Lanciano to Ascoli Piceno,149.0 km,Road stage",
            "11,19 May,Ascoli Piceno to Ravenna,268.0 km,Road stage",
            "12,20 May,Ravenna to Treviso,199.0 km,Road stage",
            ",21 May,Rest day",
            "13,22 May,Treviso to Trieste,207.0 km,Road stage",
            "14,23 May,Trieste to Belluno,243.0 km,Road stage",
            ",24 May,Rest day",
            "15,25 May,Belluno to Recoaro Terme,154.0 km,Road stage",
            ",26 May,Rest day",
            "16,27 May,Recoaro Terme to Bergamo,272.0 km,Road stage",
            "17,28 May,Bergamo to Varese,154.0 km,Road stage",
            "18a,29 May,Varese to Locarno (Switzerland),100.0 km,Road stage",
            "18b,29 May,Locarno (Switzerland) to Milano,180.0 km,Road stage"
        ];
        let edition = tour_of_italy_1938();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "18 stages including 3 split stages");
    }

    #[test]
    fn test_tour_of_italy_1939() {
        let route = [
            "1,28 April,Milano to Turin,180.0 km,Road stage",
            "2,29 April,Turin to Genoa,226.7 km,Road stage",
            "3,30 April,Genoa to Pisa,187.0 km,Road stage",
            "4,1 May,Pisa to Grosseto,154.0 km,Road stage",
            "5,2 May,Grosseto to Rome,222.0 km,Road stage",
            ",3 May,Rest day",
            "6a,4 May,Rome to Rieti,85.7 km,Road stage",
            "6b,4 May,Rieti to Monte Terminillo,14.0 km,Individual time trial",
            "7,5 May,Rieti to Pescara,191.3 km,Road stage",
            "8,6 May,Pescara to Senigallia,177.0 km,Road stage",
            "9a,7 May,Senigallia to Forlì,116.5 km,Road stage",
            "9b,7 May,Forlì to Florence,106.6 km,Road stage",
            ",8 May,Rest day",
            "10,9 May,Florence to Bologna,120.0 km,Road stage",
            "11,10 May,Bologna to Venezia,231.8 km,Road stage",
            "12,11 May,Venezia to Trieste,173.8 km,Road stage",
            ",12 May,Rest day",
            "13,13 May,Trieste to Gorizia,39.8 km,Individual time trial",
            "14,14 May,Gorizia to Cortina d'Ampezzo,195.0 km,Road stage",
            "15,15 May,Cortina d'Ampezzo to Trento,256.2 km,Road stage",
            ",16 May,Rest day",
            "16,17 May,Trento to Sondrio,166.0 km,Road stage",
            "17,18 May,Sondrio to Milano,168.0 km,Road stage"
        ];
        let edition = tour_of_italy_1939();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_italy_1940() {
        let route = [
            "1,17 May,Milano to Turin,180.0 km,Road stage",
            "2,18 May,Turin to Genoa,226.0 km,Road stage",
            "3,19 May,Genoa to Pisa,188.0 km,Road stage",
            "4,20 May,Pisa to Grosseto,154.0 km,Road stage",
            "5,21 May,Grosseto to Rome,224.0 km,Road stage",
            ",22 May,Rest day",
            "6,23 May,Rome to Naples,238.0 km,Road stage",
            "7,24 May,Naples to Fiuggi,178.0 km,Road stage",
            "8,25 May,Fiuggi to Terni,183.0 km,Road stage",
            "9,26 May,Terni to Arezzo,183.0 km,Road stage",
            "10,27 May,Arezzo to Florence,91.0 km,Road stage",
            ",28 May,Rest day",
            "11,29 May,Florence to Modena,184.0 km,Road stage",
            "12,30 May,Modena to Ferrara,184.0 km,Road stage",
            "13,31 May,Ferrara to Treviso,125.0 km,Road stage",
            "14,1 June,Treviso to Abbazia,215.0 km,Road stage",
            "15,2 June,Abbazia to Trieste,179.0 km,Road stage",
            ",3 June,Rest day",
            "16,4 June,Trieste to Pieve di Cadore,202.0 km,Road stage",
            "17,5 June,Pieve di Cadore to Ortisei,110.0 km,Road stage",
            ",6 June,Rest day",
            "18,7 June,Ortisei to Trento,186.0 km,Road stage",
            "19,8 June,Trento to Verona,149.0 km,Road stage",
            "20,9 June,Verona to Milano,180.0 km,Road stage"
        ];
        let edition = tour_of_italy_1940();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages");
    }

    #[test]
    fn test_tour_of_italy_1946() {
        let route = [
            "1,15 June,Milano to Turin,185.0 km,Road stage",
            "2,16 June,Turin to Genoa,190.0 km,Road stage",
            "3,17 June,Genoa to Montecatini Terme,222.0 km,Road stage",
            ",18 June,Rest day",
            "4a,19 June,Montecatini Terme to Prato,30.0 km,Individual time trial",
            "4b,19 June,Prato to Bologna,112.0 km,Road stage",
            "5a,20 June,Bologna to Cesena,80.0 km,Road stage",
            "5b,20 June,Cesena to Ancona,128.0 km,Road stage",
            ",21 June,Rest day",
            "6,22 June,Ancona to Chieti,170.0 km,Road stage",
            "7,23 June,Chieti to Naples,244.0 km,Road stage",
            ",24 June,Rest day",
            "8,25 June,Naples to Rome,226.0 km,Road stage",
            "9,26 June,Rome to Perugia,191.0 km,Road stage",
            "10,27 June,Perugia to Florence,165.0 km,Road stage",
            ",28 June,Rest day",
            "11,29 June,Florence to Rovigo,245.0 km,Road stage",
            "12,30 June,Rovigo to Trieste,132.0 km,Road stage",
            ",1 July,Rest day",
            "13,2 July,Udine to Auronzo di Cadore,124.5 km,Road stage",
            "14,3 July,Auronzo di Cadore to Bassano del Grappa,203.0 km,Road stage",
            ",4 July,Rest day",
            "15,5 July,Bassano del Grappa to Trento,186.0 km,Road stage",
            "16a,6 July,Trento to Verona,90.0 km,Road stage",
            "16b,6 July,Verona to Mantua,72.0 km,Road stage",
            "17,7 July,Mantua to Milano,176.0 km,Road stage"
        ];
        let edition = tour_of_italy_1946();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages including 3 split stages");
    }

    #[test]
    fn test_tour_of_italy_1947() {
        let route = [
            "1,24 May,Milano to Turin,190.0 km,Road stage",
            "2,25 May,Turin to Genoa,206.0 km,Road stage",
            "3,26 May,Genoa to Reggio Emilia,220.0 km,Road stage",
            "4,27 May,Reggio Emilia to Prato,190.0 km,Road stage",
            ",28 May,Rest day",
            "5a,29 May,Prato to Bagni di Casciana Terme,84.0 km,Road stage",
            "5b,29 May,Bagni di Casciana Terme to Florence,141.0 km,Road stage",
            "6,30 May,Florence to Perugia,161.0 km,Road stage",
            "7,31 May,Perugia to Rome,240.0 km,Road stage",
            "8,1 June,Rome to Naples,231.0 km,Road stage",
            ",2 June,Rest day",
            "9,3 June,Naples to Bari,288.0 km,Road stage",
            "10,4 June,Bari to Foggia,288.0 km,Road stage",
            "11,5 June,Foggia to Pescara,223.0 km,Road stage",
            ",6 June,Rest day",
            "12,7 June,Pescara to Cesenatico,267.0 km,Road stage",
            "13,8 June,Cesenatico to Padua,175.0 km,Road stage",
            "14,9 June,Padua to Vittorio Veneto,132.0 km,Road stage",
            "15,10 June,Vittorio Veneto to Pieve di Cadore,200.0 km,Road stage",
            ",11 June,Rest day",
            "16,12 June,Pieve di Cadore to Trento,194.0 km,Road stage",
            "17,13 June,Trento to Brescia Sant'Eufemia,114.0 km,Road stage",
            "18,14 June,Brescia Sant'Eufemia to Lugano (Switzerland),180.0 km,Road stage",
            "19,15 June,Lugano (Switzerland) to Milano,278.0 km,Road stage"
        ];
        let edition = tour_of_italy_1947();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_italy_1948() {
        let route = [
            "1,15 May,Milano to Turin,190.0 km,Road stage",
            "2,16 May,Turin to Genoa,226.0 km,Road stage",
            "3,17 May,Genoa to Parma,243.0 km,Road stage",
            "4,18 May,Parma to Viareggio,266.0 km,Road stage",
            ",19 May,Rest day",
            "5,20 May,Viareggio to Siena,165.0 km,Road stage",
            "6,21 May,Siena to Rome,256.0 km,Road stage",
            "7,22 May,Rome to Pescara,230.0 km,Road stage",
            "8,23 May,Pescara to Bari,347.0 km,Road stage",
            ",24 May,Rest day",
            "9,25 May,Bari to Naples,306.0 km,Road stage",
            "10,26 May,Naples to Fiuggi,184.0 km,Road stage",
            "11,27 May,Fiuggi to Perugia,265.0 km,Road stage",
            ",28 May,Rest day",
            "12,29 May,Perugia to Florence,169.0 km,Road stage",
            "13,30 May,Florence to Bologna,194.0 km,Road stage",
            "14,31 May,Bologna to Udine,278.0 km,Road stage",
            "15,1 June,Udine to Auronzo di Cadore,125.0 km,Road stage",
            ",2 June,Rest day",
            "16,3 June,Auronzo di Cadore to Cortina d'Ampezzo,90.0 km,Road stage",
            "17,4 June,Cortina d'Ampezzo to Trento,160.0 km,Road stage",
            "18,5 June,Trento to Brescia,239.0 km,Road stage",
            "19,6 June,Brescia to Milano,231.0 km,Road stage"
        ];
        let edition = tour_of_italy_1948();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages");
    }

    #[test]
    fn test_tour_of_italy_1949() {
        let route = [
            "1,21 May,Palermo to Catania,261.0 km,Road stage",
            "2,22 May,Catania to Messina,163.0 km,Road stage",
            "3,23 May,Villa San Giovanni to Cosenza,214.0 km,Road stage",
            "4,24 May,Cosenza to Salerno,292.0 km,Road stage",
            ",25 May,Rest day",
            "5,26 May,Salerno to Naples,161.0 km,Road stage",
            "6,27 May,Naples to Rome,233.0 km,Road stage",
            "7,28 May,Rome to Pesaro,298.0 km,Road stage",
            "8,29 May,Pesaro to Venezia,273.0 km,Road stage",
            ",30 May,Rest day",
            "9,31 May,Venezia to Udine,249.0 km,Road stage",
            "10,1 June,Udine to Bassano del Grappa,154.0 km,Road stage",
            "11,2 June,Bassano del Grappa to Bolzano,237.0 km,Road stage",
            ",3 June,Rest day",
            "12,4 June,Bolzano to Modena,253.0 km,Road stage",
            "13,5 June,Modena to Montecatini Terme,160.0 km,Road stage",
            "14,6 June,Montecatini Terme to Genoa,228.0 km,Road stage",
            "15,7 June,Genoa to San Remo,136.0 km,Road stage",
            ",8 June,Rest day",
            "16,9 June,San Remo to Cuneo,190.0 km,Road stage",
            "17,10 June,Cuneo to Pinerolo,254.0 km,Road stage",
            "18,11 June,Pinerolo to Turin,65.0 km,Individual time trial",
            "19,12 June,Turin to Milano,267.0 km,Road stage"
        ];
        let edition = tour_of_italy_1949();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages");
    }

    #[test]
    fn test_tour_of_italy_1950() {
        let route = [
            "1,24 May,Milano to Salsomaggiore Terme,225.0 km,Road stage",
            "2,25 May,Salsomaggiore Terme to Florence,245.0 km,Road stage",
            "3,26 May,Florence to Livorno,148.0 km,Road stage",
            "4,27 May,Livorno to Genoa,216.0 km,Road stage",
            "5,28 May,Genoa to Turin,216.0 km,Road stage",
            "6,29 May,Turin to Locarno (Switzerland),220.0 km,Road stage",
            ",30 May,Rest day",
            "7,31 May,Locarno (Switzerland) to Brescia,293.0 km,Road stage",
            "8,1 June,Brescia to Vicenza,214.0 km,Road stage",
            "9,2 June,Vicenza to Bolzano,2272.0 km,Road stage",
            ",3 June,Rest day",
            "10,4 June,Bolzano to Milano,294.0 km,Road stage",
            "11,5 June,Milano to Ferrara,251.0 km,Road stage",
            "12,6 June,Ferrara to Rimini,144.0 km,Road stage",
            "13,7 June,Rimini to Arezzo,244.0 km,Road stage",
            "14,8 June,Arezzo to Perugia,185.0 km,Road stage",
            ",9 June,Rest day",
            "15,10 June,Perugia to L'Aquila,185.0 km,Road stage",
            "16,11 June,L'Aquila to Campobasso,203.0 km,Road stage",
            "17,12 June,Campobasso to Naples,167.0 km,Road stage",
            "18,13 June,Naples to Rome,230.0 km,Road stage"
        ];
        let edition = tour_of_italy_1950();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "18 stages");
    }

    #[test]
    fn test_tour_of_italy_1951() {
        let route = [
            "1,19 May,Milano to Turin,202.0 km,Road stage",
            "2,20 May,Turin to Alassio,202.0 km,Road stage",
            "3,21 May,Alassio to Genoa,252.0 km,Road stage",
            "4,22 May,Genoa to Florence,265.0 km,Road stage",
            "5,23 May,Florence to Perugia,192.0 km,Road stage",
            ",24 May,Rest day",
            "6,25 May,Perugia to Terni,81.0 km,Individual time trial",
            "7,26 May,Terni to Rome,290.0 km,Road stage",
            "8,27 May,Rome to Naples,234.0 km,Road stage",
            "9,28 May,Naples to Foggia,181.0 km,Road stage",
            "10,29 May,Foggia to Pescara,311.0 km,Road stage",
            ",30 May,Rest day",
            "11,31 May,Pescara to Rimini,246.0 km,Road stage",
            "12,1 June,Rimini to San Marino (San Marino),24.0 km,Individual time trial",
            "13,2 June,Rimini to Bologna,249.0 km,Road stage",
            "14,3 June,Bologna to Brescia,220.0 km,Road stage",
            "15,4 June,Brescia to Venice,188.0 km,Road stage",
            "16,5 June,Venice to Trieste,182.0 km,Road stage",
            ",6 June,Rest day",
            "17,7 June,Trieste to Cortina d'Ampezzo,255.0 km,Road stage",
            "18,8 June,Cortina d'Ampezzo to Bolzano,242.0 km,Road stage",
            "19,9 June,Bolzano to Saint Moritz (Switzerland),166.0 km,Road stage",
            "20,10 June,Saint Moritz (Switzerland) to Milano,172.0 km,Road stage"
        ];
        let edition = tour_of_italy_1951();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages");
    }

    #[test]
    fn test_tour_of_italy_1952() {
        let route = [
            "1,17 May,Milano to Bologna,217.0 km,Road stage",
            "2,18 May,Bologna to Montecatini Terme,197.0 km,Road stage",
            "3,19 May,Montecatini Terme to Siena,205.0 km,Road stage",
            "4,20 May,Siena to Rome,250.0 km,Road stage",
            ",21 May,Rest day",
            "5,22 May,Rome to Rocca di Papa,35.0 km,Individual time trial",
            "6,23 May,Rome to Naples,234.0 km,Road stage",
            "7,24 May,Naples to Roccaraso,140.0 km,Road stage",
            "8,25 May,Roccaraso to Ancona,224.0 km,Road stage",
            "9,26 May,Ancona to Riccione,250.0 km,Road stage",
            "10,27 May,Riccione to Venice,285.0 km,Road stage",
            ",28 May,Rest day",
            "11,29 May,Venice to Bolzano,276.0 km,Road stage",
            "12,30 May,Bolzano to Bergamo,226.0 km,Road stage",
            "13,31 May,Bergamo to Como,143.0 km,Road stage",
            "14,1 June,Erba to Como,65.0 km,Individual time trial",
            "15,2 June,Como to Genoa,247.0 km,Road stage",
            "16,3 June,Genoa to San Remo,141.0 km,Road stage",
            ",4 June,Rest day",
            "17,5 June,San Remo to Cuneo,190.0 km,Road stage",
            "18,6 June,Cuneo to Saint-Vincent,190.0 km,Road stage",
            "19,7 June,Saint-Vincent to Verbania,298.0 km,Road stage",
            "20,8 June,Verbania to Milano,147.0 km,Road stage"
        ];
        let edition = tour_of_italy_1952();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages");
    }

    #[test]
    fn test_tour_of_italy_1953() {
        let route = [
            "1,12 May,Milano to Abano Terme,263.0 km,Road stage",
            "2,13 May,Abano Terme to Rimini,278.0 km,Road stage",
            "3,14 May,Rimini to San Benedetto del Tronto,182.0 km,Road stage",
            "4,15 May,San Benedetto del Tronto to Roccaraso,171.0 km,Road stage",
            "5,16 May,Roccaraso to Naples,149.0 km,Road stage",
            "6,17 May,Naples to Rome,285.0 km,Road stage",
            "7a,18 May,Rome to Grosseto,178.0 km,Road stage",
            "7b,18 May,Grosseto to Follonica,48.0 km,Individual time trial",
            "8,19 May,Follonica to Pisa,114.0 km,Road stage",
            ",20 May,Rest day",
            "9,21 May,Pisa to Modena,189.0 km,Road stage",
            "10,22 May,Modena,30.0 km,Team time trial",
            "11,23 May,Modena to Genoa,278.0 km,Road stage",
            "12,24 May,Genoa to Bordighera,256.0 km,Road stage",
            "13,25 May,Bordighera to Turin,242.0 km,Road stage",
            "14,26 May,Turin to San Pellegrino Terme,232.0 km,Road stage",
            ",27 May,Rest day",
            "15,28 May,San Pellegrino Terme to Riva del Garda,279.0 km,Road stage",
            "16,29 May,Riva del Garda to Vicenza,166.0 km,Road stage",
            "17,30 May,Vicenza to Auronzo di Cadore,186.0 km,Road stage",
            "18,31 May,Auronzo di Cadore to Bolzano,164.0 km,Road stage",
            "19,1 June,Bolzano to Bormio,125.0 km,Road stage",
            "20,2 June,Bormio to Milano,220.0 km,Road stage"
        ];
        let edition = tour_of_italy_1953();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_italy_1954() {
        let route = [
            "1,21 May,Palermo,36.0 km,Team time trial",
            "2,22 May,Palermo to Taormina,280.0 km,Road stage",
            "3,23 May,Reggio Calabria to Catanzaro,172.0 km,Road stage",
            "4,24 May,Catanzaro to Bari,352.0 km,Road stage",
            ",25 May,Rest day",
            "5,26 May,Bari to Naples,279.0 km,Road stage",
            "6,27 May,Naples to L'Aquila,252.0 km,Road stage",
            "7,28 May,L'Aquila to Rome,150.0 km,Road stage",
            "8,29 May,Rome to Chianciano Terme,195.0 km,Road stage",
            "9,30 May,Chianciano Terme to Florence,180.0 km,Road stage",
            "10,31 May,Florence to Cesenatico,211.0 km,Road stage",
            "11,1 June,Cesenatico to Abetone,230.0 km,Road stage",
            "12,2 June,Abetone to Genoa,251.0 km,Road stage",
            "13,3 June,Genoa to Turin,211.0 km,Road stage",
            "14,4 June,Turin to Brescia,240.0 km,Road stage",
            ",5 June,Rest day",
            "15,6 June,Gardone Riviera to Riva del Garda,42.0 km,Individual time trial",
            "16,7 June,Riva del Garda to Abano Terme,131.0 km,Road stage",
            "17,8 June,Abano Terme to Padua,105.0 km,Road stage",
            "18,9 June,Padua to Grado,177.0 km,Road stage",
            "19,10 June,Grado to San Martino di Castrozza,247.0 km,Road stage",
            "20,11 June,San Martino di Castrozza to Bolzano,152.0 km,Road stage",
            "21,12 June,Bolzano to Saint Moritz (Switzerland),222.0 km,Road stage",
            "22,13 June,Saint Moritz (Switzerland) to Milano,222.0 km,Road stage"
        ];
        let edition = tour_of_italy_1954();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1955() {
        let route = [
            "1,14 May,Milano to Turin,163.0 km,Road stage",
            "2,15 May,Turin to Cannes (France),243.0 km,Road stage",
            "3,16 May,Cannes (France) to San Remo,123.0 km,Road stage",
            "4,17 May,San Remo to Acqui Terme,192.0 km,Road stage",
            "5,18 May,Acqui Terme to Genoa,170.0 km,Road stage",
            "6,19 May,Genoa to Lido d'Albaro,18.0 km,Team time trial",
            "7,20 May,Genoa to Viareggio,164.0 km,Road stage",
            ",21 May,Rest day",
            "8,22 May,Viareggio to Perugia,260.0 km,Road stage",
            "9,23 May,Perugia to Rome,174.0 km,Road stage",
            "10,24 May,Frascati,207.0 km,Road stage",
            "11,25 May,Rome to Naples,242.0 km,Road stage",
            "12,26 May,Naples to Scanno,216.0 km,Road stage",
            "13,27 May,Scanno to Ancona,251.0 km,Road stage",
            "14,28 May,Ancona to Pineta di Cervia,173.0 km,Road stage",
            "15,29 May,Pineta di Cervia to Ravenna,50.0 km,Individual time trial",
            "16,30 May,Ravenna to Lido di Jesolo,245.0 km,Road stage",
            "17,31 May,Lido di Jesolo to Trieste,150.0 km,Road stage",
            ",1 June,Rest day",
            "18,2 June,Trieste to Cortina d'Ampezzo,236.0 km,Road stage",
            "19,3 June,Cortina d'Ampezzo to Trento,227.0 km,Road stage",
            "20,4 June,Trento to San Pellegrino Terme,216.0 km,Road stage",
            "21,5 June,San Pellegrino Terme to Milano,141.0 km,Road stage"
        ];
        let edition = tour_of_italy_1955();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_italy_1956() {
        let route = [
            "1,19 May,Milano to Alessandria,210.0 km,Road stage",
            "2a,20 May,Alessandria to Genoa,96.0 km,Road stage",
            "2b,20 May,Circuito di Lido d'Albaro,12.0 km,Team time trial",
            "3,21 May,Genoa to Salice Terme,152.0 km,Road stage",
            "4,22 May,Voghera to Mantua,198.0 km,Road stage",
            "5a,23 May,Mantua to Rimini,228.0 km,Road stage",
            "5b,23 May,San Marino (San Marino),13.0 km,Individual time trial",
            "6,24 May,Rimini to Pescara,245.0 km,Road stage",
            "7,25 May,Pescara to Campobasso,205.0 km,Road stage",
            "8,26 May,Campobasso to Salerno,156.0 km,Road stage",
            ",27 May,Rest day",
            "9,28 May,Rome to Grosseto,198.0 km,Road stage",
            "10,29 May,Grosseto to Livorno,230.0 km,Road stage",
            ",30 May,Rest day",
            "11,31 May,Livorno to Lucca,45.0 km,Individual time trial",
            "12,1 June,Lucca to Bologna,168.0 km,Road stage",
            "13,2 June,Bologna to Madonna di San Luca,3.0 km,Individual time trial",
            "14,3 June,Bologna to Rapallo,271.0 km,Road stage",
            "15,4 June,Rapallo to Lecco,278.0 km,Road stage",
            "16,5 June,Lecco to Sondrio,98.0 km,Road stage",
            ",6 June,Rest day",
            "17,7 June,Sondrio to Merano,163.0 km,Road stage",
            "18,8 June,Merano to Monte Bondone,242.0 km,Road stage",
            "19,9 June,Trento to San Pellegrino Terme,191.0 km,Road stage",
            "20,10 June,San Pellegrino Terme to Milano,113.0 km,Road stage"
        ];
        let edition = tour_of_italy_1956();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages including 2 split stages");
    }
 
    #[test]
    fn test_tour_of_italy_1957() {
        let route = [
            "1,18 May,Milano to Verona,191.0 km,Road stage",
            "2,19 May,Verona to Bosco Chiesanuova,28.0 km,Individual time trial",
            "3,20 May,Verona to Ferrara,169.0 km,Road stage",
            "4,21 May,Ferrara to Cattolica,190.0 km,Road stage",
            "5,22 May,Cattolica to Loreto,235.0 km,Road stage",
            "6,23 May,Loreto to Terni,175.0 km,Road stage",
            "7,24 May,Terni to Pescara,221.0 km,Road stage",
            "8,25 May,Pescara to Naples,250.0 km,Road stage",
            "9,26 May,Naples to Frascati,220.0 km,Road stage",
            "10,27 May,Rome to Siena,227.0 km,Road stage",
            "11,28 May,Siena to Montecatini Terme,230.0 km,Road stage",
            ",29 May,Rest day",
            "12,30 May,Montecatini to Forte dei Marmi,58.0 km,Individual time trial",
            "13,31 May,Forte dei Marmi to Genoa,163.0 km,Road stage",
            "14,1 June,Genoa to Saint-Vincent,235.0 km,Road stage",
            "15,2 June,Saint-Vincent to Sion (Switzerland),134.0 km,Road stage",
            "16,3 June,Sion (Switzerland) to Campo dei Fiori,229.0 km,Road stage",
            "17a,4 June,Varese to Como,82.0 km,Road stage",
            "17b,4 June,Como,34.0 km,Road stage",
            ",5 June,Rest day",
            "18,6 June,Como to Monte Bondone,242.0 km,Road stage",
            "19,7 June,Trento to Levico Terme,199.0 km,Road stage",
            "20,8 June,Levico Terme to Abano Terme,157.0 km,Road stage",
            "21,9 June,Abano Terme to Milano,257.0 km,Road stage"
        ];
        let edition = tour_of_italy_1957();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1958() {
        let route = [
            "1,18 May,Milano to Varese,178.0 km,Road stage",
            "2,19 May,Varese to Comerio,26.0 km,Individual time trial",
            "3,20 May,Varese to Saint-Vincent,187.0 km,Road stage",
            "4,21 May,Saint-Vincent to Collina di Superga,132.0 km,Road stage",
            "5,22 May,Turin to Mondovi,193.0 km,Road stage",
            "6,23 May,Mondovi to Chiavari,258.0 km,Road stage",
            "7,24 May,Chiavari to Forte dei Marmi,115.0 km,Road stage",
            ",25 May,Rest day",
            "8,26 May,Viareggio,59.1 km,Individual time trial",
            "9,27 May,Florence to Viterbo,215.0 km,Road stage",
            "10,28 May,Viterbo to Rome,155.0 km,Road stage",
            "11,29 May,Rome to Scanno,225.0 km,Road stage",
            "12,30 May,Scanno to San Benedetto del Tronto,211.0 km,Road stage",
            "13,31 May,San Benedetto del Tronto to Cattolica,192.0 km,Road stage",
            "14,1 June,San Marino (San Marino),12.0 km,Individual time trial",
            "15,2 June,Cesena to Bosco Chiesanuova,249.0 km,Road stage",
            "16,3 June,Verona to Levico Terme,200.0 km,Road stage",
            ",4 June,Rest day",
            "17,5 June,Levico Terme to Bolzano,198.0 km,Road stage",
            "18,6 June,Bolzano to Trento,183.0 km,Road stage",
            "19,7 June,Trento to Gardone Riviera,176.0 km,Road stage",
            "20,8 June,Salo to Milano,177.0 km,Road stage"
        ];
        let edition = tour_of_italy_1958();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1959() {
        let route = [
            "1,16 May,Milano to Salsomaggiore Terme,135.0 km,Road stage",
            "2,17 May,Salsomaggiore Terme,22.0 km,Individual time trial",
            "3,18 May,Salsomaggiore Terme to Abetone,180.0 km,Road stage",
            "4,19 May,Abetone to Arezzo,178.0 km,Road stage",
            "5,20 May,Arezzo to Rome,243.0 km,Road stage",
            "6,21 May,Rome to Naples,213.0 km,Road stage",
            "7,22 May,Ercolano to Mount Vesuvius,8.0 km,Individual time trial",
            "8,23 May,Ischia,31.0 km,Individual time trial",
            "9,24 May,Naples to Vasto,206.0 km,Road stage",
            "10,25 May,Vasto to Teramo,148.0 km,Road stage",
            "11,26 May,Ascoli Piceno to Rimini,245.0 km,Road stage",
            "12,27 May,Rimini to San Marino (San Marino),141.0 km,Road stage",
            ",28 May,Rest day",
            "13,29 May,Rimini to Verona,233.0 km,Road stage",
            "14,30 May,Verona to Rovereto,143.0 km,Road stage",
            "15,31 May,Trento to Bolzano,198.0 km,Road stage",
            "16,1 June,Bolzano to San Pellegrino Terme,245.0 km,Road stage",
            "17,2 June,San Pellegrino Terme to Genoa,241.0 km,Road stage",
            "18,3 June,Genoa to Turin,180.0 km,Road stage",
            "19,4 June,Turin to Susa,51.0 km,Individual time trial",
            "20,5 June,Turin to Saint-Vincent,100.0 km,Road stage",
            "21,6 June,Aosta to Courmayeur,296.0 km,Road stage",
            "22,7 June,Courmayeur to Milano,220.0 km,Road stage"
        ];
        let edition = tour_of_italy_1959();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1960() {
        let route = [
            "1,19 May,Rome to Naples,212.0 km,Road stage",
            "2,20 May,Sorrento,25.0 km,Individual time trial",
            "3,21 May,Sorrento to Campobasso,186.0 km,Road stage",
            "4,22 May,Campobasso to Pescara,192.0 km,Road stage",
            "5,23 May,Pescara to Rieti,218.0 km,Road stage",
            "6,24 May,Terni to Rimini,230.0 km,Road stage",
            "7a,25 May,Igea Marina,5.0 km,Individual time trial",
            "7b,25 May,Bellaria to Forlì,81.0 km,Road stage",
            "8,26 May,Forlì to Livorno,206.0 km,Road stage",
            "9a,27 May,Livorno to Carrara,93.0 km,Road stage",
            "9b,27 May,Carrara to Cave di Carrara,2.2 km,Individual time trial",
            "10,28 May,Carrara to Sestri Levante,171.0 km,Road stage",
            "11,29 May,Sestri Levante to Asti,180.0 km,Road stage",
            "12,30 May,Asti to Cervinia,176.0 km,Road stage",
            "13,31 May,Saint-Vincent to Milano,225.0 km,Road stage",
            ",1 June,Rest day",
            "14,2 June,Seregno to Lecco,68.0 km,Individual time trial",
            "15,3 June,Lecco to Verona,150.0 km,Road stage",
            "16,4 June,Verona to Treviso,110.0 km,Road stage",
            "17,5 June,Treviso to Trieste,147.0 km,Road stage",
            "18,6 June,Trieste to Belluno,240.0 km,Road stage",
            "19,7 June,Belluno to Trento,110.0 km,Road stage",
            "20,8 June,Trento to Bormio,229.0 km,Road stage",
            "21,9 June,Bormio to Milano,225.0 km,Road stage"
        ];
        let edition = tour_of_italy_1960();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 2 split stages");
    }
 
    #[test]
    fn test_tour_of_italy_1961() {
        let route = [
            "1,20 May,Turin,115.0 km,Road stage",
            "2,21 May,Turin to San Remo,185.0 km,Road stage",
            "3,22 May,San Remo to Genoa,149.0 km,Road stage",
            "4,23 May,Cagliari,118.0 km,Road stage",
            "5,24 May,Marsala to Palermo,144.0 km,Road stage",
            ",25 May,Rest day",
            "6,26 May,Palermo to Milazzo,224.0 km,Road stage",
            "7,27 May,Reggio Calabria to Cosenza,221.0 km,Road stage",
            "8,28 May,Cosenza to Taranto,237.0 km,Road stage",
            "9,29 May,Castellana Grotte to Bari,53.0 km,Individual time trial",
            "10,30 May,Bari to Potenza,140.0 km,Road stage",
            "11,31 May,Potenza to Teano,252.0 km,Road stage",
            "12,1 June,Gaeta to Rome,149.0 km,Road stage",
            "13,2 June,Mentana to Castelfidardo,279.0 km,Road stage",
            "14,3 June,Ancona to Florence,250.0 km,Road stage",
            "15,4 June,Florence to Modena,178.0 km,Road stage",
            "16,5 June,Modena to Vicenza,207.0 km,Road stage",
            "17,6 June,Vicenza to Trieste,204.0 km,Road stage",
            ",7 June,Rest day",
            "18,8 June,Trieste to Vittorio Veneto,161.0 km,Road stage",
            "19,9 June,Vittorio Veneto to Trento,249.0 km,Road stage",
            "20,10 June,Trento to Bormio,275.0 km,Road stage",
            "21,11 June,Bormio to Milano,214.0 km,Road stage"
        ];
        let edition = tour_of_italy_1961();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1962() {
        let route = [
            "1,19 May,Milano to Tabiano Terme,185.0 km,Road stage",
            "2,20 May,Salsomaggiore Terme to Sestri Levante,158.0 km,Road stage",
            "3,21 May,Sestri Levante to Panicagliora (Marliana),225.0 km,Road stage",
            "4,22 May,Montecatini Terme to Perugia,248.0 km,Road stage",
            "5,23 May,Perugia to Rieti,258.0 km,Road stage",
            "6,24 May,Rieti to Fiuggi,193.0 km,Road stage",
            "7,25 May,Fiuggi to Montevergine di Mercogliano,224.0 km,Road stage",
            "8,26 May,Avellino to Foggia,110.0 km,Road stage",
            "9,27 May,Foggia to Chieti,205.0 km,Road stage",
            "10,28 May,Chieti to Fano,218.0 km,Road stage",
            "11,29 May,Fano to Castrocaro Terme,170.0 km,Road stage",
            "12,30 May,Forlì to Lignano Sabbiadoro,298.0 km,Road stage",
            "13,31 May,Lignano Sabbiadoro to Nevegal,173.0 km,Road stage",
            ",1 June,Rest day",
            "14,2 June,Belluno to Passo Rolle,160.0 km,Road stage",
            "15,3 June,Moena to Aprica,215.0 km,Road stage",
            "16,4 June,Aprica to Pian del Resinelli,123.0 km,Road stage",
            "17,5 June,Lecco to Casale Monferrato,194.0 km,Road stage",
            "18,6 June,Casale Monferrato to Frabosa Soprana,232.0 km,Road stage",
            "19,7 June,Frabosa Soprana to Saint-Vincent,193.0 km,Road stage",
            "20,8 June,Saint-Vincent,238.0 km,Road stage",
            "21,9 June,Saint-Vincent to Milano,160.0 km,Road stage"
        ];
        let edition = tour_of_italy_1962();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1963() {
        let route = [
            "1,19 May,Naples to Potenza,182.0 km,Road stage",
            "2,20 May,Potenza to Bari,185.0 km,Road stage",
            "3,21 May,Bari to Campobasso,252.0 km,Road stage",
            "4,22 May,Campobasso to Pescara,213.0 km,Road stage",
            "5,23 May,Pescara to Viterbo,263.0 km,Road stage",
            "6,24 May,Bolsena to Arezzo,192.0 km,Road stage",
            "7,25 May,Arezzo to Riolo Terme,173.0 km,Road stage",
            "8,26 May,Riolo Terme to Salsomaggiore Terme,203.0 km,Road stage",
            "9,27 May,Salsomaggiore Terme to La Spezia,173.0 km,Road stage",
            "10,28 May,La Spezia to Asti,225.0 km,Road stage",
            "11,29 May,Asti to Santuario di Oropa,130.0 km,Road stage",
            "12,30 May,Biella to Leukerbad (Switzerland),214.0 km,Road stage",
            "13,31 May,Leukerbad (Switzerland) to Saint-Vincent,152.0 km,Road stage",
            "14,1 June,Saint-Vincent to Cremona,260.0 km,Road stage",
            "15,2 June,Mantua to Treviso,155.0 km,Road stage",
            ",3 June,Rest day",
            "16,4 June,Treviso,56.0 km,Individual time trial",
            "17,5 June,Treviso to Gorizia,213.0 km,Road stage",
            "18,6 June,Gorizia to Belluno Nevegal,248.0 km,Road stage",
            "19,7 June,Belluno to Moena,198.0 km,Road stage",
            "20,8 June,Moena to Lumezzane,240.0 km,Road stage",
            "21,9 June,Brescia to Milano,136.0 km,Road stage"
        ];
        let edition = tour_of_italy_1963();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1964() {
        let route = [
            "1,16 May,Bolzano to Riva del Garda,173.0 km,Road stage",
            "2,17 May,Riva del Garda to Brescia,146.0 km,Road stage",
            "3,18 May,Brescia to San Pellegrino Terme,193.0 km,Road stage",
            "4,19 May,San Pellegrino Terme to Parma,189.0 km,Road stage",
            "5,20 May,Parma to Busseto,50.0 km,Individual time trial",
            "6,21 May,Parma to Verona,100.0 km,Road stage",
            "7,22 May,Verona to Lavarone,168.0 km,Road stage",
            "8,23 May,Lavarone to Pedavena,183.0 km,Road stage",
            "9,24 May,Feltre to Marina di Ravenna,260.0 km,Road stage",
            "10,25 May,Marina di Ravenna to San Marino (San Marino),135.0 km,Road stage",
            "11,26 May,Rimini to San Benedetto del Tronto,185.0 km,Road stage",
            "12,27 May,San Benedetto del Tronto to Roccaraso,257.0 km,Road stage",
            "13,28 May,Roccaraso to Caserta,188.0 km,Road stage",
            "14,29 May,Caserta to Castel Gandolfo,210.0 km,Road stage",
            "15,30 May,Rome to Montepulciano,214.0 km,Road stage",
            "16,31 May,Montepulciano to Livorno,199.0 km,Road stage",
            "17,1 June,Livorno to Santa Margherita Ligure,210.0 km,Road stage",
            ",2 June,Rest day",
            "18,3 June,Santa Margherita Ligure to Alessandria,205.0 km,Road stage",
            "19,4 June,Alessandria to Cuneo,205.0 km,Road stage",
            "20,5 June,Cuneo to Pinerolo,254.0 km,Road stage",
            "21,6 June,Turin to Biella,200.0 km,Road stage",
            "22,7 June,Biella to Milano,146.0 km,Road stage"
        ];
        let edition = tour_of_italy_1964();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1965() {
        let route = [
            "1,15 May,San Marino (San Marino) to Perugia,198.0 km,Road stage",
            "2,16 May,Perugia to L'Aquila,180.0 km,Road stage",
            "3,17 May,L'Aquila to Rocca di Cambio,199.0 km,Road stage",
            "4,18 May,Rocca di Cambio to Benevento,239.0 km,Road stage",
            "5,19 May,Benevento to Avellino,175.0 km,Road stage",
            "6,20 May,Avellino to Potenza,161.0 km,Road stage",
            "7,21 May,Potenza to Maratea,164.0 km,Road stage",
            "8,22 May,Maratea to Catanzaro,103.0 km,Road stage",
            "9,23 May,Catanzaro to Reggio Calabria,161.0 km,Road stage",
            "10,24 May,Messina to Palermo,260.0 km,Road stage",
            "11,25 May,Palermo to Agrigento,146.0 km,Road stage",
            "12,26 May,Agrigento to Siracusa,230.0 km,Road stage",
            "13,27 May,Catania to Taormina,50.0 km,Individual time trial",
            ",28 May,Rest day",
            "14,29 May,Milano to Novi Ligure,100.0 km,Road stage",
            "15,30 May,Novi Ligure to Diano Marina,223.0 km,Road stage",
            "16,31 May,Diano Marina to Turin,205.0 km,Road stage",
            "17,1 June,Turin to Biandronno,163.0 km,Road stage",
            "18,2 June,Biandronno to Saas Fee (Switzerland),178.0 km,Road stage",
            "19,3 June,Saas Fee (Switzerland) to Madesimo,282.0 km,Road stage",
            "20,4 June,Madesimo to Passo dello Stelvio,160.0 km,Road stage",
            "21,5 June,Bormio to Brescia,179.0 km,Road stage",
            "22,6 June,Brescia to Florence,295.0 km,Road stage"
        ];
        let edition = tour_of_italy_1965();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }

    #[test]
    fn test_tour_of_italy_1966() {
        let route = [
            "1,18 May,Monte Carlo (Monaco) to Diano Marina,149.0 km,Road stage",
            "2,19 May,Imperia to Monesi,60.0 km,Road stage",
            "3,20 May,Diano Marina to Genoa,120.0 km,Road stage",
            "4,21 May,Genoa to Viareggio,241.0 km,Road stage",
            "5,22 May,Viareggio to Chianciano Terme,222.0 km,Road stage",
            "6,23 May,Chianciano Terme to Rome,226.0 km,Road stage",
            "7,24 May,Rome to Rocca di Cambio,158.0 km,Road stage",
            "8,25 May,Rocca di Cambio to Naples,238.0 km,Road stage",
            "9,26 May,Naples to Campobasso,210.0 km,Road stage",
            "10,27 May,Campobasso to Guilianova,221.0 km,Road stage",
            "11,28 May,Guilianova to Cesenatico,229.0 km,Road stage",
            "12,29 May,Cesenatico to Reggio Emilia,206.0 km,Road stage",
            "13,30 May,Parma,46.0 km,Individual time trial",
            ",31 May,Rest day",
            "14,1 June,Parma to Arona,267.0 km,Road stage",
            "15,2 June,Arona to Brescia,196.0 km,Road stage",
            "16,3 June,Brescia to Bezzecca,143.0 km,Road stage",
            "17,4 June,Riva del Garda to Levico Terme,239.0 km,Road stage",
            "18,5 June,Levico Terme to Bolzano,137.0 km,Road stage",
            "19,6 June,Bolzano to Moena,100.0 km,Road stage",
            "20,7 June,Moena to Belluno,215.0 km,Road stage",
            "21,8 June,Belluno to Vittorio Veneto,181.0 km,Road stage",
            "22,9 June,Vittorio Veneto to Trieste,172.0 km,Road stage"
        ];
        let edition = tour_of_italy_1966();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }

    #[test]
    fn test_tour_of_italy_1967() {
        let route = [
            "1,20 May,Treviglio to Alessandria,135.0 km,Road stage",
            "2,21 May,Alessandria to La Spezia,223.0 km,Road stage",
            "3,22 May,La Spezia to Prato,205.0 km,Road stage",
            "4,23 May,Florence to Chianciano Terme,155.0 km,Road stage",
            "5,24 May,Rome to Naples,220.0 km,Road stage",
            "6,25 May,Palermo,63.0 km,Road stage",
            "7,26 May,Catania to Etna,169.0 km,Road stage",
            "8,27 May,Reggio Calabria to Cosenza,218.0 km,Road stage",
            "9,28 May,Cosenza to Taranto,202.0 km,Road stage",
            "10,29 May,Bari to Potenza,145.0 km,Road stage",
            "11,30 May,Potenza to Salerno,160.0 km,Road stage",
            "12,31 May,Caserta to Blockhaus,220.0 km,Road stage",
            "13,1 June,Chieti to Riccione,253.0 km,Road stage",
            "14,2 June,Riccione to Lido degli Estensi,94.0 km,Road stage",
            "15,3 June,Lido degli Estensi to Mantua,164.0 km,Road stage",
            "16,4 June,Mantua to Verona,45.0 km,Individual time trial",
            ",5 June,Rest day",
            "17,6 June,Verona to Vicenza,140.0 km,Road stage",
            "18,7 June,Vicenza to Udine,167.0 km,Road stage",
            "19,8 June,Udine to Tre Cime di Lavaredo,170.0 km,Road stage",
            "20,9 June,Cortina d'Ampezzo to Trento,235.0 km,Road stage",
            "21,10 June,Trento to Tirano,153.0 km,Road stage",
            "22a,11 June,Tirano to Madonna del Ghisallo,137.0 km,Road stage",
            "22b,11 June,Madonna del Ghisallo to Milano,68.0 km,Road stage"
        ];
        let edition = tour_of_italy_1967();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1968() {
        let route = [
            "P,20 May,Campione d'Italia,5.7 km,Individual time trial",
            "1,21 May,Campione d'Italia to Novara,128.0 km,Road stage",
            "2,22 May,Novara to Saint-Vincent,189.0 km,Road stage",
            "3,23 May,Saint-Vincent to Alba,168.0 km,Road stage",
            "4,24 May,Alba to San Remo,162.0 km,Road stage",
            "5,25 May,San Remo,149.0 km,Road stage",
            "6,26 May,San Remo to Alessandria,223.0 km,Road stage",
            "7,27 May,Alessandria to Piacenza,174.0 km,Road stage",
            "8,28 May,San Giorgio Piacentino to Brescia,225.0 km,Road stage",
            "9,29 May,Brescia to Lido di Caldonazzo,210.0 km,Road stage",
            "10,30 May,Trento to Monte Grappa,136.0 km,Road stage",
            "11,31 May,Bassano del Grappa to Trieste,197.0 km,Road stage",
            "12,1 June,Gorizia to Tre Cime di Lavaredo,213.0 km,Road stage",
            "13,2 June,Cortina d'Ampezzo to Vittorio Veneto,163.0 km,Road stage",
            "14,3 June,Vittorio Veneto to Marina Romea,199.0 km,Road stage",
            "15,4 June,Ravenna to Imola,141.0 km,Road stage",
            ",5 June,Rest day",
            "16,6 June,Cesenatico to San Marino (San Marino),49.3 km,Individual time trial",
            "17,7 June,San Marino (San Marino) to Foligno,196.0 km,Road stage",
            "18,8 June,Foligno to Abbadia San Salvatore,166.0 km,Road stage",
            "19,9 June,Abbadia San Salvatore to Rome,181.0 km,Road stage",
            "20,10 June,Rome to Rocca di Cambio,215.0 km,Road stage",
            "21,11 June,Rocca di Cambio to Blockhaus,198.0 km,Road stage",
            "22,12 June,Chieti to Naples,235.0 km,Road stage"
        ];
        let edition = tour_of_italy_1968();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_1969() {
        let route = [
            "1,16 May,Garda to Brescia,142.0 km,Road stage",
            "2,17 May,Brescia to Mirandola,180.0 km,Road stage",
            "3,18 May,Mirandola to Montecatini Terme,188.0 km,Road stage",
            "4,19 May,Montecatini Terme,21.0 km,Individual time trial",
            "5,20 May,Montecatini Terme to Follonica,194.0 km,Road stage",
            "6,21 May,Follonica to Viterbo,198.0 km,Road stage",
            "7,22 May,Viterbo to Terracina,206.0 km,Road stage",
            "8,23 May,Terracina to Naples,133.0 km,Road stage",
            "9,24 May,Naples to Potenza,173.0 km,Road stage",
            "10,25 May,Potenza to Campitello Matese,254.0 km,Road stage",
            "11,26 May,Campitello Matese to Scanno,165.0 km,Road stage",
            "12,27 May,Scanno to Silvi Marina,180.0 km,Road stage",
            "13,28 May,Silvi Marina to Senigallia,166.0 km,Road stage",
            "14,29 May,Senigallia to San Marino (San Marino),185.0 km,Road stage",
            "15,30 May,Cesenatico to San Marino (San Marino),49.3 km,Individual time trial",
            ",31 May,Rest day",
            "16,1 June,Parma to Savona,234.0 km,Road stage",
            "17,2 June,Celle Ligure to Pavia,182.0 km,Road stage",
            "18a,3 June,Pavia to Zingonia,115.0 km,Road stage",
            "18b,3 June,Zingonia to San Pellegrino Terme,100.0 km,Road stage",
            "19,4 June,San Pellegrino Terme to Folgaria,248.0 km,Road stage",
            "20,5 June,Trento to Marmolada,230.0 km,Road stage",
            "21,6 June,Rocca Pietore to Cavalese,131.0 km,Road stage",
            "22,7 June,Cavalese to Folgarida,150.0 km,Road stage",
            "23,8 June,Folgarida to Milano,257.0 km,Road stage"
        ];
        let edition = tour_of_italy_1969();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "23 stages including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1970() {
        let route = [
            "1,18 May,San Pellegrino Terme to Biandronno,115.0 km,Road stage",
            "2,19 May,Comerio to Saint-Vincent,164.0 km,Road stage",
            "3,20 May,Saint-Vincent to Aosta,162.0 km,Road stage",
            "4,21 May,Saint-Vincent to Lodi,205.0 km,Road stage",
            "5,22 May,Lodi to Zingonia,155.0 km,Road stage",
            "6,23 May,Zingonia to Malcesine,212.0 km,Road stage",
            "7,24 May,Malcesine to Brentonico,130.0 km,Road stage",
            "8,25 May,Rovereto to Bassano del Grappa,130.0 km,Road stage",
            "9,26 May,Bassano del Grappa to Treviso,56.0 km,Individual time trial",
            ",27 May,Rest day",
            "10,28 May,Terracina to Rivisondoli,172.0 km,Road stage",
            "11,29 May,Rivisondoli to Francavilla al Mare,180.0 km,Road stage",
            "12,30 May,Francavilla al Mare to Loreto,175.0 km,Road stage",
            "13,31 May,Loreto to Faenza,188.0 km,Road stage",
            "14,1 June,Faenza to Casciana Terme,218.0 km,Road stage",
            "15,2 June,Casciana Terme to Mirandola,215.0 km,Road stage",
            "16,3 June,Mirandola to Lido di Jesolo,195.0 km,Road stage",
            "17,4 June,Lido di Jesolo to Arta Terme,165.0 km,Road stage",
            "18,5 June,Arta Terme to Marmolada,180.0 km,Road stage",
            "19,6 June,Rocca Pietore to Dobbiaco,120.0 km,Road stage",
            "20,7 June,Dobbiaco to Bolzano,155.0 km,Road stage"
        ];
        let edition = tour_of_italy_1970();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1971() {
        let route = [
            "P,20 May,Lecce to Brindisi,62.2 km,Team time trial",
            "1,21 May,Brindisi to Bari,175.0 km,Road stage",
            "2,22 May,Bari to Potenza,260.0 km,Road stage",
            "3,23 May,Potenza to Benevento,177.0 km,Road stage",
            "4,24 May,Benevento to Pescasseroli,203.0 km,Road stage",
            "5,25 May,Pescasseroli to Gran Sasso d'Italia,198.0 km,Road stage",
            "6,26 May,L'Aquila to Orvieto,163.0 km,Road stage",
            "7,27 May,Orvieto to San Vincenzo,220.0 km,Road stage",
            "8,28 May,San Vincenzo to Casciana Terme,203.0 km,Road stage",
            "9,29 May,Casciana Terme to Forte dei Marmi,141.0 km,Road stage",
            "10,30 May,Forte dei Marmi,141.0 km,Road stage",
            "11,31 May,Sestola to Mantua,199.0 km,Road stage",
            ",1 June,Rest day",
            "12,2 June,Desenzano del Garda to Serniga di Salo,28.0 km,Individual time trial",
            "13,3 June,Salo to Sottomarina di Chioggia,218.0 km,Road stage",
            "14,4 June,Chioggia to Bibione,170.0 km,Road stage",
            "15,5 June,Bibione to Ljubljana (Yugoslavia),201.0 km,Road stage",
            "16,6 June,Ljubljana (Yugoslavia) to Tarvisio,100.0 km,Road stage",
            "17,7 June,Tarvisio to Großglockner (Austria),206.0 km,Road stage",
            "18,8 June,Lienz (Austria) to Falcade,195.0 km,Road stage",
            "19,9 June,Falcade to Ponte di Legno,182.0 km,Road stage",
            "20a,10 June,Ponte di Legno to Lainate,185.0 km,Road stage",
            "20b,10 June,Lainate to Milano,20.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_1971();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1972() {
        let route = [
            "1,21 May,Venice to Ravenna,196.0 km,Road stage",
            "2,22 May,Ravenna to Fermo,212.0 km,Road stage",
            "3,23 May,Porto San Giorgio to Francavilla al Mare,205.0 km,Road stage",
            "4a,24 May,Francavilla al Mare to Blockhaus,48.0 km,Road stage",
            "4b,24 May,Blockhaus to Foggia,210.0 km,Road stage",
            "5,25 May,Foggia to Montesano sulla Marcellana,238.0 km,Road stage",
            "6,26 May,Montesano sulla Marcellana to Cosenza,190.0 km,Road stage",
            "7,27 May,Cosenza to Catanzaro,151.0 km,Road stage",
            "8,28 May,Catanzaro to Reggio Calabria,160.0 km,Road stage",
            "9,29 May,Messina,110.0 km,Road stage",
            ",30 May,Rest day",
            "10,31 May,Rome to Monte Argentario,166.0 km,Road stage",
            "11,1 June,Monte Argentario to Forte dei Marmi,242.0 km,Road stage",
            "12a,2 June,Forte dei Marmi,20.0 km,Individual time trial",
            "12b,2 June,Forte dei Marmi,20.0 km,Individual time trial",
            "13,3 June,Forte dei Marmi to Savona,200.0 km,Road stage",
            "14,4 June,Savona to Monte Jafferau,256.0 km,Road stage",
            ",5 June,Rest day",
            "15,6 June,Parabiago,168.0 km,Road stage",
            "16,7 June,Parabiago to Livigno,256.0 km,Road stage",
            "17,8 June,Livigno to Passo dello Stelvio,88.0 km,Road stage",
            "18,9 June,Sulden to Asiago,223.0 km,Road stage",
            "19a,10 June,Asiago to Arco,163.0 km,Road stage",
            "19b,10 June,Arco,18.0 km,Individual time trial",
            "20,11 June,Arco to Milano,185.0 km,Road stage"
        ];
        let edition = tour_of_italy_1972();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages including 3 split stages");
    }
        
    #[test]
    fn test_tour_of_italy_1973() {
        let route = [
            "P,18 May,Verviers (Belgium),5.2 km,Two man team time trial",
            "1,19 May,Verviers (Belgium) to Cologne (West Germany),137.0 km,Road stage",
            "2,20 May,Cologne (West Germany) to Luxembourg City (Luxembourg),227.0 km,Road stage",
            "3,21 May,Luxembourg City (Luxembourg) to Strasbourg (France),239.0 km,Road stage",
            "4,22 May,Geneva (Switzerland) to Aosta,163.0 km,Road stage",
            ",23 May,Rest day",
            "5,24 May,Saint-Vincent to Milano,173.0 km,Road stage",
            "6,25 May,Milano to Iseo,144.0 km,Road stage",
            "7,26 May,Iseo to Lido delle Nazioni,248.0 km,Road stage",
            "8,27 May,Lido delle Nazioni to Monte Carpegna,156.0 km,Road stage",
            "9,28 May,Carpegna to Alba Adriatica,243.0 km,Road stage",
            "10,29 May,Alba Adriatica to Lanciano,174.0 km,Road stage",
            "11,30 May,Lanciano to Benevento,230.0 km,Road stage",
            "12,31 May,Benevento to Fiuggi,236.0 km,Road stage",
            "13,1 June,Fiuggi to Bolsena,215.0 km,Road stage",
            "14,2 June,Bolsena to Florence,202.0 km,Road stage",
            "15,3 June,Florence to Forte dei Marmi,150.0 km,Road stage",
            ",4 June,Rest day",
            "16,5 June,Forte dei Marmi,37.0 km,Individual time trial",
            "17,6 June,Forte dei Marmi to Verona,244.0 km,Road stage",
            "18,7 June,Verona to Andalo,173.0 km,Road stage",
            "19,8 June,Andalo to Auronzo di Cadore,208.0 km,Road stage",
            "20,9 June,Auronzo di Cadore to Trieste,197.0 km,Road stage"
        ];
        let edition = tour_of_italy_1973();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_1974() {
        let route = [
            "1,16 May,Vatican City (Vatican City) to Formia,164.0 km,Road stage",
            "2,17 May,Formia to Pompeii,121.0 km,Road stage",
            "3,18 May,Pompeii to Sorrento,137.0 km,Road stage",
            ",19 May,Rest day",
            "4,20 May,Sorrento to Sapri,208.0 km,Road stage",
            "5,21 May,Sapri to Taranto,215.0 km,Road stage",
            "6,22 May,Taranto to Foggia,206.0 km,Road stage",
            "7,23 May,Foggia to Chieti,257.0 km,Road stage",
            "8,24 May,Chieti to Macerata,150.0 km,Road stage",
            "9,25 May,Macerata to Carpegna,191.0 km,Road stage",
            "10,26 May,Macerata to Carpegna,191.0 km,Road stage",
            "11a,27 May,Modena to Il Ciocco,153.0 km,Road stage",
            "11b,27 May,Il Ciocco to Forte dei Marmi,62.0 km,Road stage",
            "12,28 May,Forte dei Marmi,40.0 km,Individual time trial",
            "13,29 May,Forte dei Marmi to Pietra Ligure,231.0 km,Road stage",
            "14,30 May,Pietra Ligure to San Remo,189.0 km,Road stage",
            "15,31 May,San Remo to Valenza,206.0 km,Road stage",
            ",1 June,Rest day",
            "16,2 June,Valenza to Monte Generoso (Switzerland),158.0 km,Road stage",
            "17,3 June,Como to Iseo,158.0 km,Road stage",
            "18,4 June,Iseo to Sella Valsugana,190.0 km,Road stage",
            "19,5 June,Borgo Valsugana to Pordenone,146.0 km,Road stage",
            "20,6 June,Pordenone to Tre Cime di Lavaredo,163.0 km,Road stage",
            "21,7 June,Misurina to Bassano del Grappa,194.0 km,Road stage",
            "22,8 June,Bassano del Grappa to Milano,257.0 km,Road stage"
        ];
        let edition = tour_of_italy_1974();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1975() {
        let route = [
            "1,17 May,Milano to Fiorano Modenese,177.0 km,Road stage",
            "2,18 May,Modena to Ancona,249.0 km,Road stage",
            "3,19 May,Ancona to Prati di Tivo,175.0 km,Road stage",
            "4,20 May,Teramo to Campobasso,258.0 km,Road stage",
            "5,21 May,Campobasso to Bari,224.0 km,Road stage",
            "6,22 May,Bari to Castrovillari,213.0 km,Road stage",
            "7a,23 May,Castrovillari to Padula,123.0 km,Road stage",
            "7b,23 May,Padula to Potenza,80.0 km,Road stage",
            "8,24 May,Potenza to Sorrento,220.0 km,Road stage",
            "9,25 May,Sorrento to Frosinone,222.0 km,Road stage",
            "10,26 May,Frosinone to Tivoli,176.0 km,Road stage",
            "11,27 May,Rome to Orvieto,158.0 km,Road stage",
            "12,28 May,Chianciano Terme to Forte dei Marmi,232.0 km,Road stage",
            "13,29 May,Forte dei Marmi,38.0 km,Individual time trial",
            ",30 May,Rest day",
            "14,31 May,Il Ciocco,13.0 km,Individual time trial",
            "15,1 June,Il Ciocco to Arenzano,203.0 km,Road stage",
            "16,2 June,Arenzano to Orta San Giulio,193.0 km,Road stage",
            "17a,3 June,Orta San Giulio to Pontoglio,167.0 km,Road stage",
            "17b,3 June,Pontoglio to Monte Maddalena,46.0 km,Road stage",
            "18,4 June,Brescia to Baselga di Pine,223.0 km,Road stage",
            "19,5 June,Baselga di Pine to Pordenone,175.0 km,Road stage",
            "20,6 June,Pordenone to Alleghe,197.0 km,Road stage",
            "21,7 June,Alleghe to Passo dello Stelvio,186.0 km,Road stage"
        ];
        let edition = tour_of_italy_1975();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 2 split stages");
    }
 
    #[test]
    fn test_tour_of_italy_1976() {
        let route = [
            "1a,21 May,Catania,64.0 km,Road stage",
            "1b,21 May,Catania to Siracusa,78.0 km,Road stage",
            "2,22 May,Siracusa to Caltanissetta,210.0 km,Road stage",
            "3,23 May,Caltanissetta to Palermo,163.0 km,Road stage",
            "4,24 May,Cefalu to Messina,192.0 km,Road stage",
            "5,25 May,Reggio Calabria to Cosenza,220.0 km,Road stage",
            "6,26 May,Cosenza to Matera,207.0 km,Road stage",
            "7,27 May,Ostuni,37.0 km,Individual time trial",
            "8,28 May,Selva di Fasano to Lago Laceno,256.0 km,Road stage",
            "9,29 May,Bagnoli Irpino to Roccaraso,204.0 km,Road stage",
            "10,30 May,Roccaraso to Terni,203.0 km,Road stage",
            "11,31 May,Terni to Gabicce Mare,222.0 km,Road stage",
            "12,1 June,Gabicce Mare to Porretta Terme,215.0 km,Road stage",
            "13,2 June,Porretta Terme to Il Ciocco,146.0 km,Road stage",
            "14,3 June,Il Ciocco to Varazze,227.0 km,Road stage",
            ",4 June,Rest day",
            "15,5 June,Varazze to Ozegna,216.0 km,Road stage",
            "16,6 June,Castellamonte to Arosio,258.0 km,Road stage",
            "17,7 June,Arosio to Verona,196.0 km,Road stage",
            "18,8 June,Verona to Longarone,174.0 km,Road stage",
            "19,9 June,Longarone to Vajolet Towers,132.0 km,Road stage",
            "20,10 June,Vigo di Fassa to Terme di Comano,170.0 km,Road stage",
            "21,11 June,Terme di Comano to Bergamo,238.0 km,Road stage",
            "22a,12 June,Arcore,28.0 km,Individual time trial",
            "22b,12 June,Milano,106.0 km,Road stage"
        ];
        let edition = tour_of_italy_1976();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_italy_1977() {
        let route = [
            "P,20 May,Bacoli to Monte di Procida,7.0 km,Individual time trial",
            "1,21 May,Lago Miseno to Avellino,159.0 km,Road stage",
            "2a,22 May,Avellino to Foggia,118.0 km,Road stage",
            "2b,22 May,Foggia,65.0 km,Road stage",
            "3,23 May,Foggia to Isernia,166.0 km,Road stage",
            "4,24 May,Isernia to Pescara,228.0 km,Road stage",
            "5,25 May,Pescara to Monteluco di Spoleto,215.0 km,Road stage",
            "6a,26 May,Spoleto to Gabicce Mare,185.0 km,Road stage",
            "6b,26 May,Gabicce Mare,70.0 km,Road stage",
            "7,27 May,Gabicce Mare to Forlì,163.0 km,Road stage",
            "8a,28 May,Forlì to Circuito del Mugello,103.0 km,Road stage",
            "8b,28 May,Circuito del Mugello,79.0 km,Road stage",
            "9,29 May,Lucca to Pisa,25.0 km,Individual time trial",
            "10,30 May,Pisa to Salsomaggiore Terme,205.0 km,Road stage",
            "11,31 May,Salsomaggiore Terme to Santa Margherita Ligure,198.0 km,Road stage",
            ",1 June,Rest day",
            "12,2 June,Santa Margherita Ligure to San Giacomo di Roburent,160.0 km,Road stage",
            "13,3 June,Mondovi to Varzi,192.0 km,Road stage",
            "14,4 June,Voghera to Vicenza,247.0 km,Road stage",
            "15,5 June,Vicenza to Trieste,223.0 km,Road stage",
            "16a,6 June,Trieste to Gemona del Friuli,107.0 km,Road stage",
            "16b,6 June,Gemona del Friuli to Conegliano,116.0 km,Road stage",
            "17,7 June,Conegliano to Cortina d'Ampezzo,220.0 km,Road stage",
            "18,8 June,Cortina d'Ampezzo to Pinzolo,223.0 km,Road stage",
            "19,9 June,Pinzolo to San Pellegrino Terme,205.0 km,Road stage",
            "20,10 June,San Pellegrino Terme to Varese,138.0 km,Road stage",
            "21,11 June,Binago,29.0 km,Individual time trial",
            "22,12 June,Milano,122.0 km,Road stage"
        ];
        let edition = tour_of_italy_1977();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 4 split stages");
    }

    #[test]
    fn test_tour_of_italy_1978() {
        let route = [
            "P,7 May,Saint-Vincent,2.0 km,Individual time trial",
            "1,8 May,Saint-Vincent to Novi Ligure,175.0 km,Road stage",
            "2,9 May,Novi Ligure to La Spezia,195.0 km,Road stage",
            "3,10 May,La Spezia to Cascina,183.0 km,Road stage",
            "4,11 May,Larciano to Pistoia,25.0 km,Individual time trial",
            "5,12 May,Prato to Cattolica,200.0 km,Road stage",
            "6,13 May,Cattolica to Silvi Marina,218.0 km,Road stage",
            "7,14 May,Silvi Marina to Benevento,242.0 km,Road stage",
            "8,15 May,Benevento to Ravello,175.0 km,Road stage",
            "9,16 May,Amalfi to Latina,248.0 km,Road stage",
            "10,17 May,Latina to Lago di Piediluco,220.0 km,Road stage",
            "11a,18 May,Terni to Assisi,74.0 km,Road stage",
            "11b,18 May,Assisi to Siena,145.0 km,Road stage",
            "12,19 May,Poggibonsi to Monte Trebbio,204.0 km,Road stage",
            "13,20 May,Modigiliana to Padua,183.0 km,Road stage",
            "14,21 May,Venice,12.0 km,Individual time trial",
            ",22 May,Rest day",
            "15,23 May,Treviso to Canazei,234.0 km,Road stage",
            "16,24 May,Mazzin to Cavalese,48.0 km,Individual time trial",
            "17,25 May,Cavalese to Monte Bondone,205.0 km,Road stage",
            "18,26 May,Mezzolombardo to Sarezzo,245.0 km,Road stage",
            "19,27 May,Brescia to Inverigo,175.0 km,Road stage",
            "20,28 May,Inverigo to Milano,220.0 km,Road stage"
        ];
        let edition = tour_of_italy_1978();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1979() {
        let route = [
            "P,17 May,Florence,2.0 km,Individual time trial",
            "1,18 May,Florence to Perugia,156.0 km,Road stage",
            "2,19 May,Perugia to Castel Gandolfo,204.0 km,Road stage",
            "3,20 May,Caserta to Naples,31.0 km,Individual time trial",
            "4,21 May,Caserta to Potenza,210.0 km,Road stage",
            "5,22 May,Potenza to Vieste,223.0 km,Road stage",
            "6,23 May,Vieste to Chieti,260.0 km,Road stage",
            "7,24 May,Chieti to Pesaro,252.0 km,Road stage",
            "8,25 May,Rimini to San Marino (San Marino),28.0 km,Individual time trial",
            "9,26 May,San Marino (San Marino) to Pistoia,248.0 km,Road stage",
            "10,27 May,Lerici to Portovenere,25.0 km,Individual time trial",
            "11,28 May,La Spezia to Voghera,212.0 km,Road stage",
            "12,29 May,Alessandria to Saint-Vincent,204.0 km,Road stage",
            "13,30 May,Aosta to Meda,229.0 km,Road stage",
            "14,31 May,Meda to Bosco Chiesanuova,212.0 km,Road stage",
            "15,1 June,Verona to Treviso,121.0 km,Road stage",
            "16,2 June,Treviso to Pieve di Cadore,195.0 km,Road stage",
            ",3 June,Rest day",
            "17,4 June,Pieve di Cadore to Trento,194.0 km,Road stage",
            "18,5 June,Trento to Barzio,245.0 km,Road stage",
            "19,6 June,Cesano Maderno to Milano,44.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_1979();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue");
    }

    #[test]
    fn test_tour_of_italy_1980() {
        let route = [
            "P,15 May,Genoa,7.0 km,Individual time trial",
            "1,16 May,Genoa to Imperia,123.0 km,Road stage",
            "2,17 May,Imperia to Turin,179.0 km,Road stage",
            "3,18 May,Turin to Parma,243.0 km,Road stage",
            "4,19 May,Parma to Marina di Pisa,193.0 km,Road stage",
            "5,20 May,Pontedera to Pisa,36.0 km,Individual time trial",
            ",21 May,Rest day",
            "6,22 May,Rio Marina to Portoferraio,126.0 km,Road stage",
            "7,23 May,Castiglione della Pescaia to Orvieto,200.0 km,Road stage",
            "8,24 May,Orvieto to Fiuggi,216.0 km,Road stage",
            "9,25 May,Fiuggi to Sorrento,247.0 km,Road stage",
            "10,26 May,Sorrento to Palinuro,177.0 km,Road stage",
            "11,27 May,Palinuro to Campotenese,145.0 km,Road stage",
            "12,28 May,Villapiana Lido to Campi Salentina,203.0 km,Road stage",
            "13,29 May,Campi Salentina to Barletta,220.0 km,Road stage",
            "14,30 May,Foggia to Roccaraso,186.0 km,Road stage",
            "15,31 May,Roccaraso to Termamo,194.0 km,Road stage",
            "16,1 June,Giulianova to Gatteo a Mare,229.0 km,Road stage",
            "17,2 June,Gatteo a Mare to Sirmione,237.0 km,Road stage",
            "18,3 June,Sirmione to Zoldo Alto,239.0 km,Road stage",
            "19,4 June,Longarone to Cles,241.0 km,Road stage",
            "20,5 June,Cles to Sondrio,221.0 km,Road stage",
            "21,6 June,Saronno to Turbigo,50.0 km,Individual time trial",
            "22,7 June,Milano,114.0 km,Road stage"
        ];
        let edition = tour_of_italy_1980();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_1981() {
        let route = [
            "P,13 May,Trieste,6.6 km,Individual time trial",
            "1a,14 May,Trieste to Bibione,100.0 km,Road stage",
            "1b,14 May,Lignano Sabbiadoro to Bibione,15.0 km,Team time trial",
            "2,15 May,Bibione to Ferrara,211.0 km,Road stage",
            "3,16 May,Bologna to Recanati,255.0 km,Road stage",
            ",17 May,Rest day",
            "4,18 May,Recanati to Lanciano,214.0 km,Road stage",
            "5,19 May,Marina di San Vito to Rodi Garganico,180.0 km,Road stage",
            "6,20 May,Rodi Garganico to Bari,225.0 km,Road stage",
            "7,21 May,Bari to Potenza,143.0 km,Road stage",
            "8,22 May,Sala Consilina to Cosenza,202.0 km,Road stage",
            "9,23 May,Cosenza to Reggio Calabria,231.0 km,Road stage",
            ",24 May,Rest day",
            "10,25 May,Rome to Cascia,166.0 km,Road stage",
            "11,26 May,Cascia to Arezzo,199.0 km,Road stage",
            "12,27 May,Arezzo to Livorno,224.0 km,Road stage",
            "13,28 May,Empoli to Montecatini Terme,35.0 km,Individual time trial",
            "14,29 May,Montecatini Terme to Salsomaggiore Terme,224.0 km,Road stage",
            "15,30 May,Salsomaggiore Terme to Pavia,198.0 km,Road stage",
            "16,31 May,Milano to Mantua,178.0 km,Road stage",
            "17,1 June,Mantua to Borno,215.0 km,Road stage",
            "18,2 June,Borno to Dimaro,127.0 km,Road stage",
            ",3 June,Rest day",
            "19,4 June,Dimaro to San Vigillo di Marebbe,208.0 km,Road stage",
            "20,5 June,San Vigillo di Marebbe to Tre Cime di Lavaredo,100.0 km,Road stage",
            "21,6 June,Auronzo di Cadore to Arzignano,197.0 km,Road stage",
            "22,7 June,Soave to Verona,42.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_1981();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1982() {
        let route = [
            "P,13 May,Milano,16.0 km,Individual time trial",
            "1,14 May,Parma to Viareggio,174.0 km,Road stage",
            "2,15 May,Viareggio to Cortona,233.0 km,Road stage",
            "3,16 May,Perugia to Assisi,37.0 km,Individual time trial",
            "4,17 May,Assisi to Rome,169.0 km,Road stage",
            "5,18 May,Rome to Caserta,213.0 km,Road stage",
            "6,19 May,Caserta to Castellammare di Stabia,130.0 km,Road stage",
            "7,20 May,Castellammare di Stabia to Diamante,226.0 km,Road stage",
            ",21 May,Rest day",
            "8,22 May,Taormina to Agrigento,248.0 km,Road stage",
            "9,23 May,Agrigento to Palermo,151.0 km,Road stage",
            "10,24 May,Cefalu to Messina,197.0 km,Road stage",
            "11,25 May,Palmi to Camigliatello Silano,229.0 km,Road stage",
            ",26 May,Rest day",
            "12,27 May,Cava de'Tirreni to Campitello Matese,171.0 km,Road stage",
            "13,28 May,Campitello Matese to Pescara,164.0 km,Road stage",
            "14,29 May,Pescara to Urbino,248.0 km,Road stage",
            "15,30 May,Urbino to Comacchio,190.0 km,Road stage",
            "16,31 May,Comacchio to San Martino di Castrozza,243.0 km,Road stage",
            "17,1 June,Fiera di Primiero to Boario Terme,235.0 km,Road stage",
            "18,2 June,Piancogno to Montecampione,85.0 km,Road stage",
            "19,3 June,Boario Terme to Vigevano,162.0 km,Road stage",
            "20,4 June,Vigevano to Cuneo,177.0 km,Road stage",
            "21,5 June,Cuneo to Pinerolo,254.0 km,Road stage",
            "22,6 June,Pinerolo to Turin,42.5 km,Individual time trial"
        ];
        let edition = tour_of_italy_1982();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue");
    }

    #[test]
    fn test_tour_of_italy_1983() {
        let route = [
            "P,12 May,Brescia,8.0 km,Individual time trial",
            "1,13 May,Brescia to Mantua,70.0 km,Team time trial",
            "2,14 May,Mantua to Comacchio,192.0 km,Road stage",
            "3,15 May,Comacchio to Fano,148.0 km,Road stage",
            "4,16 May,Pesaro to Todi,187.0 km,Road stage",
            "5,17 May,Terni to Vasto,269.0 km,Road stage",
            "6,18 May,Vasto to Campitello Matese,145.0 km,Road stage",
            "7,19 May,Campitello Matese to Salerno,216.0 km,Road stage",
            "8,20 May,Salerno to Terracina,212.0 km,Road stage",
            "9,21 May,Terracina to Montefiascone,225.0 km,Road stage",
            "10,22 May,Montefiascone to Bibbiena,232.0 km,Road stage",
            "11,23 May,Bibbiena to Pietrasanta,202.0 km,Road stage",
            ",24 May,Rest day",
            "12,25 May,Pietrasanta to Reggio Emilia,180.0 km,Road stage",
            "13,26 May,Reggio Emilia to Parma,38.0 km,Individual time trial",
            "14,27 May,Parma to Savona,243.0 km,Road stage",
            "15,28 May,Savona to Orta San Giulio,219.0 km,Road stage",
            "16a,29 May,Orta San Giulio to Milano,110.0 km,Road stage",
            "16b,29 May,Milano to Bergamo,100.0 km,Road stage",
            "17,30 May,Bergamo to Colli di San Fermo,91.0 km,Road stage",
            "18,31 May,Sarnico to Vicenza,178.0 km,Road stage",
            ",1 June,Rest day",
            "19,2 June,Vicenza to Selva di Val Gardena,224.0 km,Road stage",
            "20,3 June,Selva di Val Gardena to Arabba,169.0 km,Road stage",
            "21,4 June,Arabba to Gorizia,232.0 km,Road stage",
            "22,5 June,Gorizia to Udine,40.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_1983();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1984() {
        let route = [
            "P,17 May,Lucca,5.0 km,Individual time trial",
            "1,18 May,Lucca to Marina di Pietrasanta,55.0 km,Team time trial",
            "2,19 May,Marina di Pietrasanta to Firenze,127.0 km,Road stage",
            "3,20 May,Bologna to Madonna di San Luca,110.0 km,Road stage",
            "4,21 May,Bologna to Numana,238.0 km,Road stage",
            "5,22 May,Numana to Blockhaus,194.0 km,Road stage",
            "6,23 May,Chieti to Foggia,193.0 km,Road stage",
            "7,24 May,Foggia to Marconia di Pisticci,226.0 km,Road stage",
            "8,25 May,Policoro to Agropoli,228.0 km,Road stage",
            "9,26 May,Agropoli to Cava de'Tirreni,104.0 km,Road stage",
            ",27 May,Rest day",
            "10,28 May,Cava de'Tirreni to Isernia,209.0 km,Road stage",
            "11,29 May,Isernia to Rieti,243.0 km,Road stage",
            "12,30 May,Rieti to Citta di Castello,175.0 km,Road stage",
            "13,31 May,Citta di Castello to Lerici,269.0 km,Road stage",
            "14,1 June,Lerici to Alessandria,204.0 km,Road stage",
            "15,2 June,Certosa di Pavia to Milano,38.0 km,Individual time trial",
            ",3 June,Rest day",
            "16,4 June,Alessandria to Bardonecchia,198.0 km,Road stage",
            "17,5 June,Bardonecchia to Lecco,249.0 km,Road stage",
            "18,6 June,Lecco to Merano,252.0 km,Road stage",
            "19,7 June,Merano to Selva di Val Gardena,74.0 km,Road stage",
            "20,8 June,Selva di Val Gardena to Arabba,169.0 km,Road stage",
            "21,9 June,Arabba to Treviso,208.0 km,Road stage",
            "22,10 June,Soave to Verona,42.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_1984();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_1985() {
        let route = [
            "P,16 May,Verona,6.6 km,Individual time trial",
            "1,17 May,Verona to Busto Arsizio,218.0 km,Road stage",
            "2,18 May,Busto Arsizio to Milano,38.0 km,Team time trial",
            "3,19 May,Milano to Pinzolo,190.0 km,Road stage",
            "4,20 May,Pinzolo to Selva di Val Gardena,237.0 km,Road stage",
            "5,21 May,Selva di Val Gardena to Vittorio Veneto,225.0 km,Road stage",
            "6,22 May,Vittorio Veneto to Cervia,237.0 km,Road stage",
            "7,23 May,Cervia to Jesi,185.0 km,Road stage",
            ",24 May,Rest day",
            "8a,25 May,Foggia,45.0 km,Road stage",
            "8b,25 May,Foggia to Matera,167.0 km,Road stage",
            "9,26 May,Matera to Crotone,237.0 km,Road stage",
            "10,27 May,Crotone to Paola,203.0 km,Road stage",
            "11,28 May,Paola to Salerno,240.0 km,Road stage",
            "12,29 May,Capua to Maddaloni,38.0 km,Individual time trial",
            "13,30 May,Maddaloni to Frosinone,154.0 km,Road stage",
            "14,31 May,Frosinone to Gran Sasso d'Italia,195.0 km,Road stage",
            "15,1 June,L'Aquila to Perugia,208.0 km,Road stage",
            "16,2 June,Perugia to Cecina,217.0 km,Road stage",
            "17,3 June,Cecina to Modena,248.0 km,Road stage",
            ",4 June,Rest day",
            "18,5 June,Monza to Domodossola,128.0 km,Road stage",
            "19,6 June,Domodossola to Saint-Vincent,247.0 km,Road stage",
            "20,7 June,Saint-Vincent to Valnontey di Cogne,58.0 km,Road stage",
            "21,8 June,Saint-Vincent to Genoa,229.0 km,Road stage",
            "22,9 June,Lido di Camaiore to Lucca,48.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_1985();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1986() {
        let route = [
            "P,12 May,Palermo,1.0 km,Individual time trial",
            "1,12 May,Palermo to Sciacca,140.0 km,Road stage",
            "2,13 May,Sciacca to Catania,259.0 km,Road stage",
            "3,14 May,Catania to Taormina,50.0 km,Team time trial",
            "4,15 May,Villa San Giovanni to Nicotera,115.0 km,Road stage",
            "5,16 May,Nicotera to Cosenza,194.0 km,Road stage",
            "6,17 May,Cosenza to Potenza,251.0 km,Road stage",
            "7,18 May,Potenza to Baia Domizia,257.0 km,Road stage",
            "8,19 May,Cellole to Avezzano,160.0 km,Road stage",
            "9,20 May,Avezzano to Rieti,172.0 km,Road stage",
            "10,21 May,Rieti to Pesaro,238.0 km,Road stage",
            "11,22 May,Pesaro to Castiglione del Lago,207.0 km,Road stage",
            "12,23 May,Sinalunga to Siena,46.0 km,Individual time trial",
            "13,24 May,Siena to Sarzana,175.0 km,Road stage",
            "14,25 May,Savona to Sauze d'Oulx,236.0 km,Road stage",
            "15,26 May,Sauze d'Oulx to Erba,260.0 km,Road stage",
            "16,27 May,Erba to Foppolo,143.0 km,Road stage",
            "17,28 May,Foppolo to Piacenza,186.0 km,Road stage",
            "18,29 May,Piacenza to Cremona,36.0 km,Individual time trial",
            "19,30 May,Cremona to Peio,211.0 km,Road stage",
            "20,31 May,Peio to Bassano del Grappa,179.0 km,Road stage",
            "21,1 June,Bassano del Grappa to Bolzano,234.0 km,Road stage",
            "22,2 June,Merano,108.6 km,Road stage"
        ];
        let edition = tour_of_italy_1986();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_1987() {
        let route = [
            "P,21 May,San Remo,4.0 km,Individual time trial",
            "1a,22 May,San Remo to San Romolo,31.0 km,Road stage",
            "1b,22 May,Poggio di San Remo to San Remo,8.0 km,Individual time trial",
            "2,23 May,Imperia to Borgo Val di Taro,242.0 km,Road stage",
            "3,24 May,Lerici to Camaiore,43.0 km,Team time trial",
            "4,25 May,Camaiore to Montalcino,203.0 km,Road stage",
            "5,26 May,Montalcino to Terni,208.0 km,Road stage",
            "6,27 May,Terni to Monte Terminillo,134.0 km,Road stage",
            "7,28 May,Rieti to Roccaraso,205.0 km,Road stage",
            "8,29 May,Roccaraso to San Giorgio del Sannio,168.0 km,Road stage",
            "9,30 May,San Giorgio del Sannio to Bari,257.0 km,Road stage",
            "10,31 May,Bari to Termoli,210.0 km,Road stage",
            ",1 June,Rest day",
            "11,2 June,Guilianova to Osimo,245.0 km,Road stage",
            "12,3 June,Osimo to Bellaria,197.0 km,Road stage",
            "13,4 June,Rimini to San Marino (San Marino),46.0 km,Individual time trial",
            "14,5 June,San Marino (San Marino) to Lido di Jesolo,260.0 km,Road stage",
            "15,6 June,Lido di Jesolo to Sappada,224.0 km,Road stage",
            "16,7 June,Sappada to Canazei,211.0 km,Road stage",
            "17,8 June,Canazei to Riva del Garda,206.0 km,Road stage",
            "18,9 June,Riva del Garda to Trescore Balneario,213.0 km,Road stage",
            "19,10 June,Trescore Balneario to Madesimo,160.0 km,Road stage",
            "20,11 June,Madesimo to Como,156.0 km,Road stage",
            "21,12 June,Como to Pila,252.0 km,Road stage",
            "22,13 June,Aosta to Saint-Vincent,32.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_1987();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1988() {
        let route = [
            "1,23 May,Urbino,9.0 km,Individual time trial",
            "2,24 May,Urbino to Ascoli Piceno,230.0 km,Road stage",
            "3,25 May,Ascoli Piceno to Vasto,184.0 km,Road stage",
            "4a,26 May,Vasto to Rodi Garganico,123.0 km,Road stage",
            "4b,26 May,Rodi Garganico to Vieste,40.0 km,Team time trial",
            "5,27 May,Vieste to Santa Maria Capua Vetere,260.0 km,Road stage",
            "6,28 May,Santa Maria Capua Vetere to Campitello Matese,137.0 km,Road stage",
            "7,29 May,Campitello Matese to Avezzano,178.0 km,Road stage",
            "8,30 May,Avezzano to Chianciano Terme,251.0 km,Road stage",
            "9,31 May,Pienza to Marina di Massa,235.0 km,Road stage",
            "10,1 June,Carrara to Salsomaggiore Terme,190.0 km,Road stage",
            "11,2 June,Parma to Colle Don Bosco,229.0 km,Road stage",
            "12,3 June,Novara to Selvino,205.0 km,Road stage",
            "13,4 June,Bergamo to Chiesa in Valmalenco,129.0 km,Road stage",
            "14,5 June,Chiesa in Valmalenco to Bormio,120.0 km,Road stage",
            "15,6 June,Spondigna to Merano 2000,83.0 km,Road stage",
            "16,7 June,Merano to Innsbruck (Austria),176.0 km,Road stage",
            "17,8 June,Innsbruck (Austria) to Borgo Valsugana,221.0 km,Road stage",
            "18,9 June,Levico Terme to Valico del Vetriolo,18.0 km,Individual time trial",
            "19,10 June,Borgo Valsugana to Arta Terme,223.0 km,Road stage",
            "20,11 June,Arta Terme to Lido di Jesolo,212.0 km,Road stage",
            "21a,12 June,Lido di Jesolo to Vittorio Veneto,73.0 km,Road stage",
            "21b,12 June,Vittorio Veneto,43.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_1988();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 2 split stages");
    }
 
    #[test]
    fn test_tour_of_italy_1989() {
        let route = [
            "1,21 May,Taormina to Catania,123.0 km,Road stage",
            "2,22 May,Catania to Mount Etna,132.0 km,Road stage",
            "3,23 May,Villafranca to Messina,32.5 km,Team time trial",
            "4,24 May,Scilla to Cosenza,204.0 km,Road stage",
            "5,25 May,Cosenza to Potenza,275.0 km,Road stage",
            "6,26 May,Potenza to Campobasso,223.0 km,Road stage",
            "7,27 May,Isernia to Rome,208.0 km,Road stage",
            "8,28 May,Rome to Gran Sasso d'Italia,179.0 km,Road stage",
            "9,29 May,L'Aquila to Gubbio,221.0 km,Road stage",
            "10,30 May,Pesaro to Riccione,36.8 km,Individual time trial",
            "11,31 May,Riccione to Mantua,148.0 km,Road stage",
            "12,1 June,Mantua to Mira,148.0 km,Road stage",
            "13,2 June,Padua to Auronzo di Cadore,207.0 km,Road stage",
            "14,3 June,Auronzo di Cadore to Corvara,131.0 km,Road stage",
            "15a,4 June,Corvara to Trento,131.0 km,Road stage",
            "15b,4 June,Trento,83.2 km,Road stage",
            "16,5 June,Trento to Santa Caterina di Valfurva,208.0 km,Road stage",
            "17,6 June,Sondrio to Meda,137.0 km,Road stage",
            "18,7 June,Mendrisio (Switzerland) to Monte Generoso (Switzerland),10.7 km,Individual time trial",
            "19,8 June,Meda to Tortona,198.0 km,Road stage",
            "20,9 June,Voghera to La Spezia,220.0 km,Road stage",
            "21,10 June,La Spezia to Prato,216.0 km,Road stage",
            "22,11 June,Prato to Florence,53.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_1989();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1990() {
        let route = [
            "P,18 May,Bari,13.0 km,Individual time trial",
            "1,19 May,Bari to Sala Consilina,239.0 km,Road stage",
            "2,20 May,Sala Consilina to Mount Vesuvius,190.0 km,Road stage",
            "3a,21 May,Ercolano to Nola,31.0 km,Road stage",
            "3b,21 May,Nola to Sora,164.0 km,Road stage",
            "4,22 May,Sora to Teramo,233.0 km,Road stage",
            "5,23 May,Teramo to Fabriano,200.0 km,Road stage",
            "6,24 May,Fabriano to Vallombrosa,197.0 km,Road stage",
            "7,25 May,Reggello to Marina di Pietrasanta,188.0 km,Road stage",
            "8,26 May,La Spezia to Langhirano,176.0 km,Road stage",
            "9,27 May,Grinzane Cavour to Cuneo,68.0 km,Individual time trial",
            "10,28 May,Cuneo to Lodi,241.0 km,Road stage",
            "11,29 May,Brescia to Baselga di Pine,241.0 km,Road stage",
            "12,30 May,Baselga di Pine to Udine,224.0 km,Road stage",
            "13,31 May,Klagenfurt (Austria),164.0 km,Road stage",
            "14,1 June,Velden am Worther (Austria) to Dobbiaco,226.0 km,Road stage",
            "15,2 June,Dobbiaco to Passo Pordoi,171.0 km,Road stage",
            "16,3 June,Moena to Aprica,223.0 km,Road stage",
            "17,4 June,Aprica to Gallarate,180.0 km,Road stage",
            "18,5 June,Gallarate to Sacro Monte di Varese,39.0 km,Individual time trial",
            "19,6 June,Milano,90.0 km,Road stage"
        ];
        let edition = tour_of_italy_1990();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1991() {
        let route = [
            "1,26 May,Olbia,193.0 km,Road stage",
            "2a,27 May,Olbia to Sassari,127.0 km,Road stage",
            "2b,27 May,Sassari,7.0 km,Individual time trial",
            "3,28 May,Sassari to Cagliari,231.0 km,Road stage",
            ",29 May,Rest day",
            "4,30 May,Sorrento,170.0 km,Road stage",
            "5,31 May,Sorrento to Scanno,246.0 km,Road stage",
            "6,1 June,Scanno to Rieti,205.0 km,Road stage",
            "7,2 June,Rieti to Citta di Castello,174.0 km,Road stage",
            "8,3 June,Citta di Castello to Prato,169.0 km,Road stage",
            "9,4 June,Prato to Felino,229.0 km,Road stage",
            "10,5 June,Collecchio to Langhirano,43.0 km,Individual time trial",
            "11,6 June,Sala Baganza to Savona,223.0 km,Road stage",
            "12,7 June,Savona to Pian del Re,182.0 km,Road stage",
            "13,8 June,Savigliano to Sestrière,192.0 km,Road stage",
            "14,9 June,Turin to Morbegno,239.0 km,Road stage",
            "15,10 June,Morbegno to Aprica,132.0 km,Road stage",
            "16,11 June,Tirano to Selva di Val Gardena,220.0 km,Road stage",
            "17,12 June,Selva di Val Gardena to Passo Pordoi,169.0 km,Road stage",
            "18,13 June,Pozza di Fassa to Castelfranco Veneto,165.0 km,Road stage",
            "19,14 June,Castelfranco Veneto to Brescia,185.0 km,Road stage",
            "20,15 June,Brescia to Casteggio,66.0 km,Individual time trial",
            "21,16 June,Pavia to Milano,153.0 km,Road stage"
        ];
        let edition = tour_of_italy_1991();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1992() {
        let route = [
            "1,24 May,Genoa,8.0 km,Individual time trial",
            "2,25 May,Genoa to Uliveto Terme,194.0 km,Road stage",
            "3,26 May,Uliveto Terme to Arezzo,174.0 km,Road stage",
            "4,27 May,Arezzo to Sansepolcro,38.0 km,Individual time trial",
            "5,28 May,Sansepolcro to Porto Sant'Elpidio,198.0 km,Road stage",
            "6,29 May,Porto Sant'Elpidio to Sulmona,223.0 km,Road stage",
            "7,30 May,Roccaraso to Melfi,232.0 km,Road stage",
            "8,31 May,Melfi to Aversa,184.0 km,Road stage",
            "9,1 June,Aversa to Latina,165.0 km,Road stage",
            "10,2 June,Latina to Monte Terminillo,196.0 km,Road stage",
            "11,3 June,Montepulciano to Imola,233.0 km,Road stage",
            "12,4 June,Imola to Bassano del Grappa,233.0 km,Road stage",
            "13,5 June,Bassano del Grappa to Corvara,204.0 km,Road stage",
            "14,6 June,Corvara to Monte Bondone,205.0 km,Road stage",
            "15,7 June,Riva del Garda to Palazzolo sull'Oglio,171.0 km,Road stage",
            "16,8 June,Palazzolo sull'Oglio to Sondrio,166.0 km,Road stage",
            "17,9 June,Sondrio to Vercelli,203.0 km,Road stage",
            "18,10 June,Vercelli to Pian del Re,200.0 km,Road stage",
            "19,11 June,Saluzzo to Pila,260.0 km,Road stage",
            "20,12 June,Saint-Vincent to Verbania,201.0 km,Road stage",
            "21,13 June,Verbania to Vigevano,95.0 km,Road stage",
            "22,14 June,Vigevano to Milano,66.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_1992();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1993() {
        let route = [
            "1a,23 May,Porto Azzurro to Porteferraio,85.0 km,Road stage",
            "1b,23 May,Porteferraio,9.0 km,Individual time trial",
            "2,24 May,Grosseto to Rieti,224.0 km,Road stage",
            "3,25 May,Rieti to Scanno,153.0 km,Road stage",
            "4,26 May,Lago di Scanno to Marcianise,179.0 km,Road stage",
            "5,27 May,Paestum to Terme Luigiane,210.0 km,Road stage",
            "6,28 May,Villafranca Tirrena to Messina,130.0 km,Road stage",
            "7,29 May,Capo d'Orlando to Agrigento,240.0 km,Road stage",
            "8,30 May,Agrigento to Palermo,140.0 km,Road stage",
            ",31 May,Rest day",
            "9,1 June,Montelibretti to Fabriano,216.0 km,Road stage",
            "10,2 June,Senigallia,28.0 km,Individual time trial",
            "11,3 June,Senigallia to Dozza,184.0 km,Road stage",
            "12,4 June,Dozza to Asiago,239.0 km,Road stage",
            "13,5 June,Asiago to Corvara,220.0 km,Road stage",
            "14,6 June,Corvara,245.0 km,Road stage",
            "15,7 June,Corvara to Lumezzane,263.0 km,Road stage",
            "16,8 June,Lumezzane to Borgo Val di Taro,181.0 km,Road stage",
            "17,9 June,Varazze to Pontechianale,223.0 km,Road stage",
            "18,10 June,Sampeyre to Fossano,150.0 km,Road stage",
            "19,11 June,Pinerolo to Sestrière,55.0 km,Individual time trial",
            "20,12 June,Turin to Santuario di Oropa,162.0 km,Road stage",
            "21,13 June,Biella to Milano,166.0 km,Road stage"
        ];
        let edition = tour_of_italy_1993();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1994() {
        let route = [
            "1a,22 May,Bologna,86.0 km,Road stage",
            "1b,22 May,Bologna,7.0 km,Individual time trial",
            "2,23 May,Bologna to Osimo,232.0 km,Road stage",
            "3,24 May,Osimo to Loreto Aprutino,185.0 km,Road stage",
            "4,25 May,Montesilvano to Campitello Matese,204.0 km,Road stage",
            "5,26 May,Campobasso to Melfi,158.0 km,Road stage",
            "6,27 May,Potenza to Caserta,215.0 km,Road stage",
            "7,28 May,Fiuggi,119.0 km,Road stage",
            "8,29 May,Grosseto to Follonica,44.0 km,Individual time trial",
            "9,30 May,Castiglione della Pescaia to Pontedera,153.0 km,Road stage",
            "10,31 May,Marostica,115.0 km,Road stage",
            "11,1 June,Marostica to Bibione,165.0 km,Road stage",
            "12,2 June,Bibione to Kranj (Slovenia),204.0 km,Road stage",
            "13,3 June,Kranj (Slovenia) to Lienz (Austria),231.0 km,Road stage",
            "14,4 June,Lienz (Austria) to Merano,235.0 km,Road stage",
            "15,5 June,Merano to Aprica,195.0 km,Road stage",
            "16,6 June,Sondrio to Stradella,220.0 km,Road stage",
            "17,7 June,Santa Maria della Versa to Lavagna,190.0 km,Road stage",
            "18,8 June,Chiavari to Passo del Bocco,35.0 km,Individual time trial",
            "19,9 June,Lavagna to Bra,212.0 km,Road stage",
            "20,10 June,Cuneo to Les Deux Alpes (France),206.0 km,Road stage",
            "21,11 June,Les Deux Alpes (France) to Sestrière,121.0 km,Road stage",
            "22,12 June,Turin to Milano,198.0 km,Road stage"
        ];
        let edition = tour_of_italy_1994();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 1 split stage");
    }
 
    #[test]
    fn test_tour_of_italy_1995() {
        let route = [
            "1,13 May,Perugia to Terni,205.0 km,Road stage",
            "2,14 May,Foligno to Assisi,19.0 km,Individual time trial",
            "3,15 May,Spoleto to Marotta,161.0 km,Road stage",
            "4,16 May,Mondolfo to Loreto,192.0 km,Road stage",
            "5,17 May,Porto Recanati to Tortoreto,182.0 km,Road stage",
            "6,18 May,Trani to Taranto,165.0 km,Road stage",
            "7,19 May,Taranto to Terme Luigiane,216.0 km,Road stage",
            "8,20 May,Acquappesa to Massiccio del Sirino,209.0 km,Road stage",
            "9,21 May,Terme La Calda to Salerno,165.0 km,Road stage",
            "10,22 May,Telese Terme to Maddaloni,42.0 km,Individual time trial",
            ",23 May,Rest day",
            "11,24 May,Pietrasanta to Il Ciocco,175.0 km,Road stage",
            "12,25 May,Borgo a Mozzano to Cento,195.0 km,Road stage",
            "13,26 May,Pieve di Cento to Rovereto,218.0 km,Road stage",
            "14,27 May,Trento to Schnals,240.0 km,Road stage",
            "15,28 May,Schnals to Lenzerheide (Switzerland),185.0 km,Road stage",
            "16,29 May,Lenzerheide (Switzerland) to Treviglio,224.0 km,Road stage",
            "17,30 May,Cenate Sotto to Selvino,43.0 km,Individual time trial",
            "18,31 May,Stradella to Sanctuario di Vicoforte,221.0 km,Road stage",
            "19,1 June,Mondovi to Pontechianale,130.0 km,Road stage",
            "20,2 June,Briançon (France) to Gressoney-Saint-Jean,208.0 km,Road stage",
            "21,3 June,Pont-Saint-Martin to Luino,190.0 km,Road stage",
            "22,4 June,Luino to Milano,148.0 km,Road stage"
        ];
        let edition = tour_of_italy_1995();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1996() {
        let route = [
            "1,18 May,Athens (Greece),170.0 km,Road stage",
            "2,19 May,Eleusis (Greece) to Naupactus (Greece),235.0 km,Road stage",
            "3,20 May,Missolonghi (Greece) to Ioannina (Greece),199.0 km,Road stage",
            ",21 May,Rest day",
            "4,22 May,Ostuni,147.0 km,Road stage",
            "5,23 May,Metaponto to Crotone,196.0 km,Road stage",
            "6,24 May,Crotone to Catanzaro,176.0 km,Road stage",
            "7,25 May,Amantea to Massiccio del Sirino,164.0 km,Road stage",
            "8,26 May,Polla to Naples,135.0 km,Road stage",
            "9,27 May,Naples to Fiuggi,184.0 km,Road stage",
            "10,28 May,Arezzo to Prato,164.0 km,Road stage",
            "11,29 May,Prato to Marino di Massa,130.0 km,Road stage",
            "12,30 May,Aulla to Loano,195.0 km,Road stage",
            "13,31 May,Loano to Prato Nevoso,115.0 km,Road stage",
            "14,1 June,Sanctuario di Vicoforte to Briançon (France),202.0 km,Road stage",
            "15,2 June,Briançon (France) to Aosta,235.0 km,Road stage",
            "16,3 June,Aosta to Lausanne (Switzerland),180.0 km,Road stage",
            "17,4 June,Lausanne (Switzerland) to Biella,236.0 km,Road stage",
            "18,5 June,Meda to Vicenza,216.0 km,Road stage",
            "19,6 June,Vicenza to Marostica,62.0 km,Individual time trial",
            "20,7 June,Marostica to Passo Pordoi,220.0 km,Road stage",
            "21,8 June,Cavalese to Aprica,250.0 km,Road stage",
            "22,9 June,Sondrio to Milano,176.0 km,Road stage"
        ];
        let edition = tour_of_italy_1996();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1997() {
        let route = [
            "1,17 May,Venezia,128.0 km,Road stage",
            "2,18 May,Mestre to Cervia,211.0 km,Road stage",
            "3,19 May,Santarcangelo di Romagna to San Marino (San Marino),18.0 km,Individual time trial",
            "4,20 May,San Marino (San Marino) to Arezzo,156.0 km,Road stage",
            "5,21 May,Arezzo to Monte Terminillo,215.0 km,Road stage",
            "6,22 May,Rieti to Lanciano,210.0 km,Road stage",
            "7,23 May,Lanciano to Mondragone,210.0 km,Road stage",
            "8,24 May,Mondragone to Cava de'Tirreni,212.0 km,Road stage",
            "9,25 May,Cava de'Tirreni to Castrovillari,232.0 km,Road stage",
            "10,26 May,Castrovillari to Taranto,195.0 km,Road stage",
            ",27 May,Rest day",
            "11,28 May,Lido di Camaiore,155.0 km,Road stage",
            "12,29 May,La Spezia to Varazze,214.0 km,Road stage",
            "13,30 May,Varazze to Cuneo,150.0 km,Road stage",
            "14,31 May,Racconigi to Breuil-Cervinia,240.0 km,Road stage",
            "15,1 June,Verres to Borgomanero,173.0 km,Road stage",
            "16,2 June,Borgomanero to Dalmine,158.0 km,Road stage",
            "17,3 June,Dalmine to Verona,200.0 km,Road stage",
            "18,4 June,Baselga di Pine to Cavalese,40.0 km,Individual time trial",
            "19,5 June,Predazzo to Pfalzen,222.0 km,Road stage",
            "20,6 June,Bruneck to Passo del Tonale,176.0 km,Road stage",
            "21,7 June,Male to Edolo,238.0 km,Road stage",
            "22,8 June,Boario Terme to Milano,165.0 km,Road stage"
        ];
        let edition = tour_of_italy_1997();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }
 
    #[test]
    fn test_tour_of_italy_1998() {
        let route = [
            "P,16 May,Nice (France),8.0 km,Individual time trial",
            "1,17 May,Nice (France) to Cuneo,159.0 km,Road stage",
            "2,18 May,Alba to Imperia,160.0 km,Road stage",
            "3,19 May,Rapallo to Forte dei Marmi,196.0 km,Road stage",
            "4,20 May,Viareggio to Monte Argentario,239.0 km,Road stage",
            "5,21 May,Orbetello to Frascati,206.0 km,Road stage",
            "6,22 May,Maddaloni to Laga Laceno,158.0 km,Road stage",
            "7,23 May,Montella to Matera,238.0 km,Road stage",
            "8,24 May,Matera to Lecce,191.0 km,Road stage",
            "9,25 May,Foggia to Vasto,167.0 km,Road stage",
            "10,26 May,Vasto to Macerata,212.0 km,Road stage",
            "11,27 May,Macerata to San Marino (San Marino),220.0 km,Road stage",
            "12,28 May,San Marino (San Marino) to Carpi,202.0 km,Road stage",
            "13,29 May,Carpi to Schio,166.0 km,Road stage",
            "14,30 May,Schio to Piancavallo,165.0 km,Road stage",
            "15,31 May,Trieste,40.0 km,Individual time trial",
            "16,1 June,Udine to Asiago,227.0 km,Road stage",
            "17,2 June,Asiago to Selva,217.0 km,Road stage",
            "18,3 June,Selva to Passo di Pampeagno,115.0 km,Road stage",
            "19,4 June,Cavalese to Plan di Montecampione,239.0 km,Road stage",
            "20,5 June,Darfo Boario Terme to Mendrisio (Switzerland),137.0 km,Road stage",
            "21,6 June,Mendrisio (Switzerland) to Lugano (Switzerland),34.0 km,Individual time trial",
            "22,7 June,Lugano (Switzerland) to Milano,173.0 km,Road stage"
        ];
        let edition = tour_of_italy_1998();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_1999() {
        let route = [
            "1,15 May,Agrigento to Modica,175.0 km,Road stage",
            "2,16 May,Noto to Catania,133.0 km,Road stage",
            "3,17 May,Catania to Messina,176.0 km,Road stage",
            "4,18 May,Vibo Valentia to Terme Luigiane,186.0 km,Road stage",
            "5,19 May,Terme Luigiane to Massiccio del Sirino,144.0 km,Road stage",
            "6,20 May,Lauria to Foggia,257.0 km,Road stage",
            "7,21 May,Foggia to Lanciano,153.0 km,Road stage",
            "8,22 May,Pescara to Gran Sasso d'Italia,253.0 km,Road stage",
            "9,23 May,Ancona,32.0 km,Individual time trial",
            "10,24 May,Ancona to Sansepolcro,189.0 km,Road stage",
            "11,25 May,Sansepolcro to Cesenatico,125.0 km,Road stage",
            "12,26 May,Cesenatico to Sassuolo,168.0 km,Road stage",
            "13,27 May,Sassuolo to Rapallo,243.0 km,Road stage",
            ",28 May,Rest day",
            "14,29 May,Bra to Borgo San Dalmazzo,187.0 km,Road stage",
            "15,30 May,Racconigi to Santuario di Oropa,160.0 km,Road stage",
            "16,31 May,Biella to Lumezzane,232.0 km,Road stage",
            "17,1 June,Lumezzane to Castelfranco Veneto,215.0 km,Road stage",
            "18,2 June,Treviso,45.0 km,Individual time trial",
            "19,3 June,Castelfranco Veneto to Alpe di Pampeago,166.0 km,Road stage",
            "20,4 June,Predazzo to Madonna di Campiglio,175.0 km,Road stage",
            "21,5 June,Madonna di Campiglio to Aprica,190.0 km,Road stage",
            "22,6 June,Darfo Boario Terme to Milano,170.0 km,Road stage"
        ];
        let edition = tour_of_italy_1999();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2000() {
        let route = [
            "P,13 May,Rome,4.6 km,Individual time trial",
            "1,14 May,Rome to Terracina,125.0 km,Road stage",
            "2,15 May,Terracina to Maddaloni,225.0 km,Road stage",
            "3,16 May,Paestum to Scalea,177.0 km,Road stage",
            "4,17 May,Scalea to Matera,233.0 km,Road stage",
            "5,18 May,Matera to Peschici,232.0 km,Road stage",
            "6,19 May,Peschici to Vasto,160.0 km,Road stage",
            "7,20 May,Vasto to Teramo,182.0 km,Road stage",
            "8,21 May,Corinaldo to Prato,265.0 km,Road stage",
            "9,22 May,Prato to Abetone,138.0 km,Road stage",
            "10,23 May,San Marcello Pistoiese to Padua,253.0 km,Road stage",
            "11,24 May,Lignano Sabbiadoro to Bibione,45.0 km,Individual time trial",
            ",25 May,Rest day",
            "12,26 May,Bibione to Feltre,184.0 km,Road stage",
            "13,27 May,Feltre to Selva,186.0 km,Road stage",
            "14,28 May,Selva to Bormio,203.0 km,Road stage",
            "15,29 May,Bormio to Brescia,180.0 km,Road stage",
            "16,30 May,Brescia to Meda,102.0 km,Road stage",
            "17,31 May,Meda to Genoa,236.0 km,Road stage",
            "18,1 June,Genoa to Prato Nevoso,173.0 km,Road stage",
            "19,2 June,Saluzzo to Briançon (France),176.0 km,Road stage",
            "20,3 June,Briançon (France) to Sestrière,32.0 km,Individual time trial",
            "21,4 June,Turin to Milano,189.0 km,Road stage"
        ];
        let edition = tour_of_italy_2000();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_2001() {
        let route = [
            "P,19 May,Montesilvano to Pescara,7.0 km,Individual time trial",
            "1,20 May,Guilianova to Francavilla al Mare,205.0 km,Road stage",
            "2,21 May,Fossacesia to Lucera,163.0 km,Road stage",
            "3,22 May,Lucera to Potenza,149.0 km,Road stage",
            "4,23 May,Potenza to Mercogliano,169.0 km,Road stage",
            "5,24 May,Avellino to Nettuno,229.0 km,Road stage",
            "6,25 May,Nettuno to Rieti,152.0 km,Road stage",
            "7,26 May,Rieti to Montevarchi,239.0 km,Road stage",
            "8,27 May,Montecatini Terme to Reggio Emilia,185.0 km,Road stage",
            "9,28 May,Reggio Emilia to Rovigo,140.0 km,Road stage",
            "10,29 May,Lido di Jesolo to Ljubljana (Slovenia),212.0 km,Road stage",
            "11,30 May,Bled (Slovenia) to Gorizia,187.0 km,Road stage",
            "12,31 May,Gradisca d'Isonzo to Montebelluna,139.0 km,Road stage",
            "13,1 June,Montebelluna to Passo Pordoi,225.0 km,Road stage",
            "14,2 June,Cavalese to Arco,160.0 km,Road stage",
            "15,3 June,Sirmione to Salo,55.0 km,Individual time trial",
            "16,4 June,Erbusco to Parma,142.0 km,Road stage",
            ",5 June,Rest day",
            "17,6 June,San Remo,123.0 km,Road stage",
            "18,7 June,Imperia to Sant'Anna di Vinadio,230.0 km,Road stage",
            "19,8 June,Alba to Busto Arsizio,163.0 km,Road stage",
            "20,9 June,Busto Arsizio to Arona,181.0 km,Road stage",
            "21,10 June,Arona to Milano,125.0 km,Road stage"
        ];
        let edition = tour_of_italy_2001();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_2002() {
        let route = [
            "P,11 May,Groningen (Netherlands),6.5 km,Individual time trial",
            "1,12 May,Groningen (Netherlands) to Münster (Germany),215.0 km,Road stage",
            "2,13 May,Cologne (Germany) to Ans (Belgium),209.0 km,Road stage",
            "3,14 May,Verviers (Belgium) to Esch-sur-Alzette (Luxembourg),206.0 km,Road stage",
            "4,15 May,Esch-sur-Alzette (Luxembourg) to Strasbourg (France),232.0 km,Road stage",
            ",16 May,Rest day",
            "5,17 May,Fossano to Limone Piemonte,150.0 km,Road stage",
            "6,18 May,Cuneo to Varazze,190.0 km,Road stage",
            "7,19 May,Viareggio to Lido di Camaiore,159.0 km,Road stage",
            "8,20 May,Capannori to Orvieto,237.0 km,Road stage",
            "9,21 May,Tivoli to Caserta,41.0 km,Road stage",
            "10,22 May,Maddaloni to Benevento,118.0 km,Road stage",
            "11,23 May,Benevento to Campitello Matese,140.0 km,Road stage",
            "12,24 May,Campobasso to Chieti,200.0 km,Road stage",
            "13,25 May,Chieti to San Giacomo di Valle Castellana,190.0 km,Road stage",
            "14,26 May,Numana,30.0 km,Individual time trial",
            ",27 May,Rest day",
            "15,28 May,Terme Euganee to Conegliano,156.0 km,Road stage",
            "16,29 May,Conegliano to Corvara,163.0 km,Road stage",
            "17,30 May,Corvara to Folgaria,222.0 km,Road stage",
            "18,31 May,Rovereto to Brescia,143.0 km,Road stage",
            "19,1 June,Cambiago to Monticello Brianza,46.0 km,Individual time trial",
            "20,2 June,Cantu to Milano,141.0 km,Road stage"
        ];
        let edition = tour_of_italy_2002();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_2003() {
        let route = [
            "1,10 May,Lecce,201.0 km,Road stage",
            "2,11 May,Copertino to Matera,177.0 km,Road stage",
            "3,12 May,Policoro to Term Luigiane,145.0 km,Road stage",
            "4,13 May,Term Luigiane to Vibo Valentia,170.0 km,Road stage",
            "5,14 May,Messina to Catania,176.0 km,Road stage",
            ",15 May,Rest day",
            "6,16 May,Maddaloni to Avezzano,222.0 km,Road stage",
            "7,17 May,Avezzano to Monte Terminillo,146.0 km,Road stage",
            "8,18 May,Rieti to Arezzo,214.0 km,Road stage",
            "9,19 May,Arezzo to Montecatini Terme,160.0 km,Road stage",
            "10,20 May,Montecatini Terme to Faenza,202.0 km,Road stage",
            "11,21 May,Faenza to San Dona di Piave,222.0 km,Road stage",
            "12,22 May,San Dona di Piave to Monte Zoncolan,185.0 km,Road stage",
            "13,23 May,Pordenone to Marostica,149.0 km,Road stage",
            "14,24 May,Marostica to Alpe di Pampeago,162.0 km,Road stage",
            "15,25 May,Merano to Bolzano,42.5 km,Individual time trial",
            "16,26 May,Arco to Pavia,207.0 km,Road stage",
            ",27 May,Rest day",
            "17,28 May,Salice Terme to Asti,117.0 km,Road stage",
            "18,29 May,Sanuario di Vicoforte to Chianale,174.0 km,Road stage",
            "19,30 May,Canelli to Cascata del Toce,239.0 km,Road stage",
            "20,31 May,Cannobio to Cantu,133.0 km,Road stage",
            "21,1 June,Milano,33.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_2003();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2004() {
        let route = [
            "P,8 May,Genoa,6.9 km,Individual time trial",
            "1,9 May,Genoa to Alba,143.0 km,Road stage",
            "2,10 May,Novi Ligure to Pontremoli,184.0 km,Road stage",
            "3,11 May,Pontremoli to Corno alle Scale,191.0 km,Road stage",
            "4,12 May,Porretta Terme to Civitella di Val di Chiana,184.0 km,Road stage",
            "5,13 May,Civitella di Val di Chiana to Spoleto,177.0 km,Road stage",
            "6,14 May,Spoleto to Valmontone,164.0 km,Road stage",
            "7,15 May,Frosinone to Montevergine di Mercogliano,214.0 km,Road stage",
            "8,16 May,Giffoni Valle Piana to Policoro,214.0 km,Road stage",
            "9,17 May,Policoro to Carovigno,142.0 km,Road stage",
            ",18 May,Rest day",
            "10,19 May,Porto Sant'Elpidio to Ascoli Piceno,146.0 km,Road stage",
            "11,20 May,Porto Sant'Elpidio to Cesena,228.0 km,Road stage",
            "12,21 May,Cesena to Treviso,210.0 km,Road stage",
            "13,22 May,Trieste,52.0 km,Individual time trial",
            "14,23 May,Trieste to Pula (Croatia),175.0 km,Road stage",
            "15,24 May,Porec (Croatia) to San Vendemiano,234.0 km,Road stage",
            "16,25 May,San Vendemiano to Pfalzen,217.0 km,Road stage",
            ",26 May,Rest day",
            "17,27 May,Bruneck to Fondo/Sarnonico,153.0 km,Road stage",
            "18,28 May,Cles to Bormio 2000,118.0 km,Road stage",
            "19,29 May,Bormio to Presolana,122.0 km,Road stage",
            "20,30 May,Clusone to Milano,149.0 km,Road stage"
        ];
        let edition = tour_of_italy_2004();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_2005() {
        let route = [
            "P,7 May,Reggio Calabria,1.1 km,Individual time trial",
            "1,8 May,Reggio Calabria to Tropea,208.0 km,Road stage",
            "2,9 May,Catanzaro Lido to Santa Maria del Cedro,182.0 km,Road stage",
            "3,10 May,Diamante to Giffoni Valle Piana,205.0 km,Road stage",
            "4,11 May,Giffoni Valle Piana to Frosinone,211.0 km,Road stage",
            "5,12 May,Celano to L'Aquila,223.0 km,Road stage",
            "6,13 May,Viterbo to Marina di Grosseto,153.0 km,Road stage",
            "7,14 May,Grosseto to Pistoia,211.0 km,Road stage",
            "8,15 May,Lamporecchio to Florence,45.0 km,Individual time trial",
            "9,16 May,Florence to Ravenna,139.0 km,Road stage",
            ",17 May,Rest day",
            "10,18 May,Ravenna to Rossanto Veneto,212.0 km,Road stage",
            "11,19 May,Marostica to Zoldo Alto,150.0 km,Road stage",
            "12,20 May,Alleghe to Rovereto,178.0 km,Road stage",
            "13,21 May,Mezzocorona to Urtijei,218.0 km,Road stage",
            "14,22 May,Neumarkt to Livigno,210.0 km,Road stage",
            "15,23 May,Villa di Tirano to Lissone,154.0 km,Road stage",
            ",24 May,Rest day",
            "16,25 May,Lissone to Varazze,210.0 km,Road stage",
            "17,26 May,Varazze to Limone Piemonte,194.0 km,Road stage",
            "18,27 May,Chieri to Turin,34.0 km,Individual time trial",
            "19,28 May,Savigliano to Sestrière,190.0 km,Road stage",
            "20,29 May,Albese con Cassano to Milano,119.0 km,Road stage"
        ];
        let edition = tour_of_italy_2005();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }
 
    #[test]
    fn test_tour_of_italy_2006() {
        let route = [
            "1,6 May,Seraing (Belgium),6.2 km,Individual time trial",
            "2,7 May,Mons (Belgium) to Charleroi (Belgium),197.0 km,Road stage",
            "3,8 May,Perwez (Belgium) to Namur (Belgium),202.0 km,Road stage",
            "4,9 May,Wanze (Belgium) to Hotton (Belgium),193.0 km,Road stage",
            ",10 May,Rest day",
            "5,11 May,Piacenza to Cremona,38.0 km,Team time trial",
            "6,12 May,Busseto to Forlì,227.0 km,Road stage",
            "7,13 May,Cesena to Saltara,236.0 km,Road stage",
            "8,14 May,Civitanova Marche to Maielletta,171.0 km,Road stage",
            "9,15 May,Francavilla al Mare to Termoli,121.0 km,Road stage",
            "10,16 May,Termoli to Peschici,187.0 km,Road stage",
            ",17 May,Rest day",
            "11,18 May,Pontedera,50.0 km,Individual time trial",
            "12,19 May,Livorno to Sestri Levante,171.0 km,Road stage",
            "13,20 May,Alessandria to La Thuile,218.0 km,Road stage",
            "14,21 May,Aosta to Domodossola,223.0 km,Road stage",
            "15,22 May,Mergozzo to Brescia,189.0 km,Road stage",
            "16,23 May,Rovato to Trento,173.0 km,Road stage",
            "17,24 May,Tramin to Plan de Corones,133.0 km,Road stage",
            "18,25 May,Sillian to Gemona del Friuli,210.0 km,Road stage",
            "19,26 May,Pordenone to Passo di San Pellegrino,224.0 km,Road stage",
            "20,27 May,Trento to Aprica,211.0 km,Road stage",
            "21,28 May,Museo del Ghisallo to Milano,140.0 km,Road stage"
        ];
        let edition = tour_of_italy_2006();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_italy_2007() {
        let route = [
            "1,12 May,Caprera to La Maddalena,25.6 km,Team time trial",
            "2,13 May,Tempio Pausania to Bosa,205.0 km,Road stage",
            "3,14 May,Barumini to Cagliari,181.0 km,Road stage",
            ",15 May,Rest day",
            "4,16 May,Salerno to Montevergine di Mercogliano,153.0 km,Road stage",
            "5,17 May,Teano to Frascati,173.0 km,Road stage",
            "6,18 May,Tivoli to Spoleto,177.0 km,Road stage",
            "7,19 May,Spoleto to Scarperia,254.0 km,Road stage",
            "8,20 May,Barberino di Mugello to Fiorano Modenese,200.0 km,Road stage",
            "9,21 May,Reggio Emilia to Lido di Camaiore,177.0 km,Road stage",
            "10,22 May,Camaiore to Santuario Nostra Signora della Guardia,250.0 km,Road stage",
            "11,23 May,Serraville Scrivia to Pinerolo,198.0 km,Road stage",
            "12,24 May,Scalenghe to Briançon (France),163.0 km,Road stage",
            "13,25 May,Biella to Santuario di Oropa,12.6 km,Individual time trial",
            "14,26 May,Cantu to Bergamo,192.0 km,Road stage",
            "15,27 May,Trento to Tre Cime di Lavaredo,184.0 km,Road stage",
            ",28 May,Rest day",
            "16,29 May,Agordo to Lienz (Austria),189.0 km,Road stage",
            "17,30 May,Lienz (Austria) to Monte Zoncolan,142.0 km,Road stage",
            "18,31 May,Udine to Riese Pio X,203.0 km,Road stage",
            "19,1 June,Treviso to Terme di Comano,179.0 km,Road stage",
            "20,2 June,Bardolino to Verona,43.0 km,Individual time trial",
            "21,3 June,Vestone to Milano,185.0 km,Road stage"
        ];
        let edition = tour_of_italy_2007();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2008() {
        let route = [
            "1,10 May,Palermo,23.6 km,Team time trial",
            "2,11 May,Cefalu to Agrigento,207.0 km,Road stage",
            "3,12 May,Catania to Milazzo,221.0 km,Road stage",
            "4,13 May,Pizzo Calabro to Catanzaro-Lungomare,183.0 km,Road stage",
            "5,14 May,Belvedere Marittimo to Contursi Terme,203.0 km,Road stage",
            "6,15 May,Potenza to Peschici,231.6 km,Road stage",
            "7,16 May,Vasto to Pescocostanzo,180.0 km,Road stage",
            "8,17 May,Rivisondoli to Tivoli,208.0 km,Road stage",
            "9,18 May,Civitavecchia to San Vincenzo,218.0 km,Road stage",
            ",19 May,Rest day",
            "10,20 May,Pesaro to Urbino,39.4 km,Individual time trial",
            "11,21 May,Urbania to Cesena,199.0 km,Road stage",
            "12,22 May,Forlì to Carpi,172.0 km,Road stage",
            "13,23 May,Modena to Cittadella,177.0 km,Road stage",
            "14,24 May,Verona to Alpe di Pampeago,195.0 km,Road stage",
            "15,25 May,Arabba to Passo Fedaia,153.0 km,Road stage",
            "16,26 May,San Vigillo di Marebbe to Plan de Corones,12.8 km,Individual time trial",
            ",27 May,Rest day",
            "17,28 May,Sondrio to Locarno (Switzerland),146.0 km,Road stage",
            "18,29 May,Mendrisio (Switzerland) to Varese,147.0 km,Road stage",
            "19,30 May,Legnago to Presolana,238.0 km,Road stage",
            "20,31 May,Rovetta to Tirano,224.0 km,Road stage",
            "21,1 June,Milano,28.5 km,Individual time trial"
        ];
        let edition = tour_of_italy_2008();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2009() {
        let route = [
            "1,9 May,Lido (Venice),20.5 km,Team time trial",
            "2,10 May,Jesolo to Trieste,156.0 km,Road stage",
            "3,11 May,Grado to Valdobbiadene,198.0 km,Road stage",
            "4,12 May,Padua to San Martino di Castrozza,162.0 km,Road stage",
            "5,13 May,San Martino di Castrozza to Alpe di Siusi,125.0 km,Road stage",
            "6,14 May,Brixen to Mayrhofen (Austria),248.0 km,Road stage",
            "7,15 May,Innsbruck (Austria) to Chiavenna,244.0 km,Road stage",
            "8,16 May,Morbegno to Bergamo,209.0 km,Road stage",
            "9,17 May,Milano,165.0 km,Road stage",
            ",18 May,Rest day",
            "10,19 May,Cuneo to Pinerolo,262.0 km,Road stage",
            "11,20 May,Turin to Arenzano,214.0 km,Road stage",
            "12,21 May,Sestri Levante to Riomaggiore,60.6 km,Individual time trial",
            "13,22 May,Lido di Camaiore to Florence,176.0 km,Road stage",
            "14,23 May,Campi Bisenzio to Bologna,172.0 km,Road stage",
            "15,24 May,Forlì to Faenza,161.0 km,Road stage",
            "16,25 May,Pergola to Monte Petrano,237.0 km,Road stage",
            ",26 May,Rest day",
            "17,27 May,Chieti to Blockhaus,83.0 km,Road stage",
            "18,28 May,Sulmona to Benevento,182.0 km,Road stage",
            "19,29 May,Avellino to Mount Vesuvius,164.0 km,Road stage",
            "20,30 May,Naples to Anagi,203.0 km,Road stage",
            "21,31 May,Rome,14.4 km,Individual time trial"
        ];
        let edition = tour_of_italy_2009();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2010() {
        let route = [
            "1,8 May,Amsterdam (Netherlands),8.4 km,Individual time trial",
            "2,9 May,Amsterdam (Netherlands) to Utrecht (Netherlands),209.0 km,Road stage",
            "3,10 May,Amsterdam (Netherlands) to Middleburg (Netherlands),224.0 km,Road stage",
            ",11 May,Rest day",
            "4,12 May,Savigliano to Cuneo,32.5 km,Team time trial",
            "5,13 May,Novara to Novi Ligure,168.0 km,Road stage",
            "6,14 May,Fidenza to Marina di Carrara,166.0 km,Road stage",
            "7,15 May,Carrara to Montalcino,215.0 km,Road stage",
            "8,16 May,Chianciano to Monte Terminillo,189.0 km,Road stage",
            "9,17 May,Frosinone to Cava de'Tirreni,188.0 km,Road stage",
            "10,18 May,Avellino to Bitonto,220.0 km,Road stage",
            "11,19 May,Lucera to L'Aquila,256.0 km,Road stage",
            "12,20 May,Citta Sant'Angelo to Porto Recanati,191.0 km,Road stage",
            "13,21 May,Porto Recanati to Cesenatico,222.0 km,Road stage",
            "14,22 May,Ferrara to Asolo (Monte Grappa),201.0 km,Road stage",
            "15,23 May,Mestre to Monte Zoncolan,161.0 km,Road stage",
            ",24 May,Rest day",
            "16,25 May,Mareo to Plan de Corones,12.9 km,Individual time trial",
            "17,26 May,Bruneck to Peio Terme,173.0 km,Road stage",
            "18,27 May,Levico Terme to Brescia,151.0 km,Road stage",
            "19,28 May,Brescia to Aprica,195.0 km,Road stage",
            "20,29 May,Bormio to Passo del Tonale,178.0 km,Road stage",
            "21,30 May,Verona,15.3 km,Individual time trial"
        ];
        let edition = tour_of_italy_2010();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2011() {
        let route = [
            "1,7 May,Veneria Reale to Turin,19.3 km,Team time trial",
            "2,8 May,Alba to Parma,244.0 km,Road stage",
            "3,9 May,Reggio Emilia to Rapallo,173.0 km,Road stage",
            "4,10 May,Quarto dei Mille to Livorno,216.0 km,Road stage",
            "5,11 May,Piombino to Orvieto,191.0 km,Road stage",
            "6,12 May,Orvieto to Fiuggi,216.0 km,Road stage",
            "7,13 May,Maddaloni to Montevergine di Mercogliano,110.0 km,Road stage",
            "8,14 May,Sapri to Tropea,217.0 km,Road stage",
            "9,15 May,Messina to Etna,169.0 km,Road stage",
            ",16 May,Rest day",
            "10,17 May,Termoli to Teramo,159.0 km,Road stage",
            "11,18 May,Teramo to Castelfidardo,142.0 km,Road stage",
            "12,19 May,Castelfidardo to Ravenna,184.0 km,Road stage",
            "13,20 May,Spilimbergo to Großglockner (Austria),167.0 km,Road stage",
            "14,21 May,Lienz (Austria) to Monte Zoncolan,170.0 km,Road stage",
            "15,22 May,Conegliano to Gardeccia-Val di Fassa,229.0 km,Road stage",
            ",23 May,Rest day",
            "16,24 May,Belluno to Nevegal,12.7 km,Individual time trial",
            "17,25 May,Feltre to Tirano,230.0 km,Road stage",
            "18,26 May,Morbegno to San Pellegrino Terme,151.0 km,Road stage",
            "19,27 May,Bergamo to Macugnaga,209.0 km,Road stage",
            "20,28 May,Verbania to Sestrière,242.0 km,Road stage",
            "21,29 May,Milano,26.0 km,Individual time trial"
        ];
        let edition = tour_of_italy_2011();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2012() {
        let route = [
            "1,5 May,Herning (Denmark),8.7 km,Individual time trial",
            "2,6 May,Herning (Denmark),206.0 km,Road stage",
            "3,7 May,Horsens (Denmark),190.0 km,Road stage",
            ",8 May,Rest day",
            "4,9 May,Verona,33.2 km,Team time trial",
            "5,10 May,Modena to Fano,209.0 km,Road stage",
            "6,11 May,Urbino to Porto Sant'Elpidio,210.0 km,Road stage",
            "7,12 May,Recanati to Rocca di Cambio,205.0 km,Road stage",
            "8,13 May,Sulmona to Lago Laceno,229.0 km,Road stage",
            "9,14 May,San Giorgio del Sannio to Frosinone,166.0 km,Road stage",
            "10,15 May,Civitavecchia to Assisi,186.0 km,Road stage",
            "11,16 May,Assisi to Montecatini Terme,255.0 km,Road stage",
            "12,17 May,Seravezza to Sestri Levante,155.0 km,Road stage",
            "13,18 May,Savona to Cervere,121.0 km,Road stage",
            "14,19 May,Cherasco to Cervinia,206.0 km,Road stage",
            "15,20 May,Busto Arsizio to Lecco-Pian dei Resinelli,169.0 km,Road stage",
            ",21 May,Rest day",
            "16,22 May,Limone sul Garda to Pfalzen,173.0 km,Road stage",
            "17,23 May,Pfalzen to Cortina d'Ampezzo,186.0 km,Road stage",
            "18,24 May,San Vito di Cadore to Vedelago,149.0 km,Road stage",
            "19,25 May,Treviso to Alpe di Pampeago,198.0 km,Road stage",
            "20,26 May,Caldes-Val di Sole to Passo dello Stelvio,219.0 km,Road stage",
            "21,27 May,Milano,28.2 km,Individual time trial"
        ];
        let edition = tour_of_italy_2012();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2013() {
        let route = [
            "1,4 May,Naples,130.0 km,Road stage",
            "2,5 May,Ischia to Forio,17.4 km,Team time trial",
            "3,6 May,Sorrento to Marina di Ascea,222.0 km,Road stage",
            "4,7 May,Policastro Bussentino to Serra San Bruno,246.0 km,Road stage",
            "5,8 May,Cosenza to Matera,203.0 km,Road stage",
            "6,9 May,Mola di Bari to Margherita di Savoia,169.0 km,Road stage",
            "7,10 May,San Salvo to Pescara,177.0 km,Road stage",
            "8,11 May,Gabicce Mare to Saltara,54.8 km,Individual time trial",
            "9,12 May,Sansepolcro to Florence,170.0 km,Road stage",
            ",13 May,Rest day",
            "10,14 May,Cordenons to Altopiano del Montasio,167.0 km,Road stage",
            "11,15 May,Cave del Predil to Erto e Casso,182.0 km,Road stage",
            "12,16 May,Longarne to Treviso,134.0 km,Road stage",
            "13,17 May,Busseto to Cherasco,254.0 km,Road stage",
            "14,18 May,Cervere to Bardonecchia,180.0 km,Road stage",
            "15,19 May,Cesana Torinese to Col du Galibier Valloire,149.0 km,Road stage",
            ",20 May,Rest day",
            "16,21 May,Valloire (France) to Ivrea,238.0 km,Road stage",
            "17,22 May,Caravaggio to Vicenza,214.0 km,Road stage",
            "18,23 May,Mori to Polsa,20.6 km,Individual time trial",
            "19,24 May,Ponte di Legno to Martell,139.0 km,Road stage",
            "20,25 May,Schlanders to Tre Cime di Lavaredo,203.0 km,Road stage",
            "21,26 May,Riese Pio X to Brescia,197.0 km,Road stage"
        ];
        let edition = tour_of_italy_2013();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2014() {
        let route = [
            "1,9 May,Belfast (Northern Ireland),21.7 km,Team time trial",
            "2,10 May,Belfast (Northern Ireland),219.0 km,Road stage",
            "3,11 May,Armagh (Northern Ireland) to Dublin (Ireland),187.0 km,Road stage",
            ",12 May,Rest day",
            "4,13 May,Giovinazzo to Bari,112.0 km,Road stage",
            "5,14 May,Taranto to Viggiano,203.0 km,Road stage",
            "6,15 May,Sassono to Montecassino,257.0 km,Road stage",
            "7,16 May,Frosinone to Foligno,211.0 km,Road stage",
            "8,17 May,Foligno to Montecopiolo,179.0 km,Road stage",
            "9,18 May,Lugo to Sestola,172.0 km,Road stage",
            ",19 May,Rest day",
            "10,20 May,Modena to Salsomaggiore Terme,173.0 km,Road stage",
            "11,21 May,Collecchio to Savona,249.0 km,Road stage",
            "12,22 May,Barbaresco to Barolo,41.9 km,Individual time trial",
            "13,23 May,Fossano to Rivarolo Canavese,157.0 km,Road stage",
            "14,24 May,Agile to Oropa,164.0 km,Road stage",
            "15,25 May,Valdengo to Montecampione,225.0 km,Road stage",
            ",26 May,Rest day",
            "16,27 May,Ponte di Legno to Val Martello,139.0 km,Road stage",
            "17,28 May,Sarnonico to Vittorio Veneto,208.0 km,Road stage",
            "18,29 May,Belluno to Rifugio Panarotta,171.0 km,Road stage",
            "19,30 May,Bassano del Grappa to Cima Grappa,26.8 km,Individual time trial",
            "20,31 May,Maniago to Monte Zoncolan,167.0 km,Road stage",
            "21,1 June,Gemona del Friuli to Trieste,172.0 km,Road stage"
        ];
        let edition = tour_of_italy_2014();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2015() {
        let route = [
            "1,9 May,San Lorenzo al Mare to San Remo,17.6 km,Team time trial",
            "2,10 May,Albenga to Genoa,177.0 km,Road stage",
            "3,11 May,Rapallo to Sestri Levante,136.0 km,Road stage",
            "4,12 May,Chiavari to La Spezia,150.0 km,Road stage",
            "5,13 May,La Spezia to Abetone,152.0 km,Road stage",
            "6,14 May,Montecatini Terme to Castiglione della Pescaia,183.0 km,Road stage",
            "7,15 May,Grosseto to Fiuggi,264.0 km,Road stage",
            "8,16 May,Fiuggi to Campitello Matese,186.0 km,Road stage",
            "9,17 May,Benevento to San Giorgio del Sannio,224.0 km,Road stage",
            ",18 May,Rest day",
            "10,19 May,Civitanova Marche to Forlì,200.0 km,Road stage",
            "11,20 May,Forlì to Imola (Autodromo Enzo e Dino Ferrari),153.0 km,Road stage",
            "12,21 May,Imola to Vicenza (Monte Berico),190.0 km,Road stage",
            "13,22 May,Montecchio Maggiore to Jesolo,147.0 km,Road stage",
            "14,23 May,Treviso to Valdobbiadene,59.4 km,Individual time trial",
            "15,24 May,Marostica to Madonna di Campiglio,165.0 km,Road stage",
            ",25 May,Rest day",
            "16,26 May,Pinzolo to Aprica,174.0 km,Road stage",
            "17,27 May,Tirano to Lugano (Switzerland),134.0 km,Road stage",
            "18,28 May,Melide (Switzerland) to Verbania,170.0 km,Road stage",
            "19,29 May,Gravellona Toce to Cervinia,236.0 km,Road stage",
            "20,30 May,Saint-Vincent to Sestrière,196.0 km,Road stage",
            "21,31 May,Turin to Milano,185.0 km,Road stage"
        ];
        let edition = tour_of_italy_2015();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2016() {
        let route = [
            "1,6 May,Apeldoorn (Netherlands),9.8 km,Individual time trial",
            "2,7 May,Arnhem (Netherlands) to Nijmegen (Netherlands),190.0 km,Road stage",
            "3,8 May,Nijmegen (Netherlands) to Arnhem (Netherlands),190.0 km,Road stage",
            ",9 May,Rest day",
            "4,10 May,Catanzaro to Praia a Mare,200.0 km,Road stage",
            "5,11 May,Praia a Mare to Benevento,233.0 km,Road stage",
            "6,12 May,Ponte to Roccaraso (Aremogna),157.0 km,Road stage",
            "7,13 May,Sulmona to Foligno,211.0 km,Road stage",
            "8,14 May,Foligno to Arezzo,186.0 km,Road stage",
            "9,15 May,Chianti Classico Stage,40.5 km,Individual time trial",
            ",16 May,Rest day",
            "10,17 May,Campi Bisenzio to Sestola,219.0 km,Road stage",
            "11,18 May,Modena to Asolo,227.0 km,Road stage",
            "12,19 May,Noale to Bibione,182.0 km,Road stage",
            "13,20 May,Palmanova to Cividale del Friuli,170.0 km,Road stage",
            "14,21 May,Alpago (Farra) to Corvara,210.0 km,Road stage",
            "15,22 May,Castelrotto to Alpe di Siusi/Seiseralm,10.8 km,Individual time trial",
            ",23 May,Rest day",
            "16,24 May,Bressanone to Andalo,132.0 km,Road stage",
            "17,25 May,Molveno to Cassano d'Adda,196.0 km,Road stage",
            "18,26 May,Muggio to Pinerolo,244.0 km,Road stage",
            "19,27 May,Pinerolo to Risoul (France),162.0 km,Road stage",
            "20,28 May,Guillestre (France) to Sant'Anna di Vinadio,134.0 km,Road stage",
            "21,29 May,Cuneo to Torino,163.0 km,Road stage"
        ];
        let edition = tour_of_italy_2016();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2017() {
        let route = [
            "1,5 May,Alghero to Olbia,206.0 km,Road stage",
            "2,6 May,Olbia to Tortolì,221.0 km,Road stage",
            "3,7 May,Tortolì to Cagliari,148.0 km,Road stage",
            ",8 May,Rest day",
            "4,9 May,Cefalu to Etna,181.0 km,Road stage",
            "5,10 May,Pedara to Messina,159.0 km,Road stage",
            "6,11 May,Reggio Calabria to Terme Luigiane,217.0 km,Road stage",
            "7,12 May,Castrovillari to Alberobello (Valle D'Itria),224.0 km,Road stage",
            "8,13 May,Molfetta to Peschici,189.0 km,Road stage",
            "9,14 May,Montenero di Bisaccia to Blockhaus,149.0 km,Road stage",
            ",15 May,Rest day",
            "10,16 May,Foligno to Montefalco,39.8 km,Individual time trial",
            "11,17 May,Firenze (Ponte a Ema) to Bagno di Romagna,161.0 km,Road stage",
            "12,18 May,Forlì to Reggio Emilia,234.0 km,Road stage",
            "13,19 May,Reggio Emilia to Tortona,167.0 km,Road stage",
            "14,20 May,Castellania to Oropa (Biella),131.0 km,Road stage",
            "15,21 May,Valdengo to Bergamo,199.0 km,Road stage",
            ",22 May,Rest day",
            "16,23 May,Rovetta to Bormio,222.0 km,Road stage",
            "17,24 May,Tirano to Canazei (Val di Fassa),219.0 km,Road stage",
            "18,25 May,Moena (Val di Fassa) to Ortisei/St. Ulrich (Val Gardena),137.0 km,Road stage",
            "19,26 May,San Candido/Innichen to Piancavallo (Monte Jafferau),191.0 km,Road stage",
            "20,27 May,Pordenone to Asiago,190.0 km,Road stage",
            "21,28 May,Monza to Milano,29.3 km,Individual time trial"
        ];
        let edition = tour_of_italy_2017();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
 
    #[test]
    fn test_tour_of_italy_2018() {
        let route = [
            "1,4 May,Jerusalem (Israel),9.7 km,Individual time trial",
            "2,5 May,Haifa (Israel) to Tel Aviv (Israel),167.0 km,Road stage",
            "3,6 May,Beersheba (Israel) to Eilat (Israel),229.0 km,Road stage",
            ",7 May,Rest day",
            "4,8 May,Catania to Caltagirone,202.0 km,Road stage",
            "5,9 May,Agrigento to Santa Ninfa (Valle del Belice),153.0 km,Road stage",
            "6,10 May,Caltanissetta to Mount Etna,169.0 km,Road stage",
            "7,11 May,Pizzo to Praia a Mare,159.0 km,Road stage",
            "8,12 May,Praia a Mare to Montevergine di Mercogliano,209.0 km,Road stage",
            "9,13 May,Pesco Sannita to Gran Sasso d'Italia (Campo Imperatore),225.0 km,Road stage",
            ",14 May,Rest day",
            "10,15 May,Penne to Gualdo Tadino,244.0 km,Road stage",
            "11,16 May,Assisi to Osimo,156.0 km,Road stage",
            "12,17 May,Osimo to Imola,214.0 km,Road stage",
            "13,18 May,Ferrara to Nervesa della Battaglia,180.0 km,Road stage",
            "14,19 May,San Vito al Tagliamento to Monte Zoncolan,186.0 km,Road stage",
            "15,20 May,Tolmezzo to Sappada,176.0 km,Road stage",
            ",21 May,Rest day",
            "16,22 May,Trento to Rovereto,34.2 km,Individual time trial",
            "17,23 May,Riva del Garda to Iseo,149.5 km,Road stage",
            "18,24 May,Abbiategrasso to Prato Nevoso,196.0 km,Road stage",
            "19,25 May,Veneria Reale to Bardonecchia (Monte Jafferau),185.0 km,Road stage",
            "20,26 May,Susa to Cervinia,214.0 km,Road stage",
            "21,27 May,Rome,115.0 km,Road stage"
        ];

        let edition = tour_of_italy_2018();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
   
        let stage1 = edition.get_stage(&String::from("1"));
        assert_eq!(stage1.is_flat_stage(), true);

        let stage2_cols = [
            "91.0 km,Zikhrow Ya'Aqov,C.4,162m"
        ];

        let stage2 = edition.get_stage(&String::from("2"));
        assert_eq!(stage2.profile(), stage2_cols);
        assert_eq!(stage2.is_flat_stage(), false);

        let stage3_cols = [
            "127.8 km,Faran River,C.4,322m"
        ];

        let stage3 = edition.get_stage(&String::from("3"));
        assert_eq!(stage3.profile(), stage3_cols);
        assert_eq!(stage3.is_flat_stage(), false);

        let stage4_cols = [
            "86.4 km,Pietre Calde,C.4,779m",
            "154.5 km,Vizzini,C.4,595m"
        ];

        let stage4 = edition.get_stage(&String::from("4"));
        assert_eq!(stage4.profile(), stage4_cols);
        assert_eq!(stage4.is_flat_stage(), false);

        let stage5_cols = [
            "90.7 km,Santa Margherita di Belice,C.4,430m",
            "111.8 km,Partanna,C.4,405m"
        ];

        let stage5 = edition.get_stage(&String::from("5"));
        assert_eq!(stage5.profile(), stage5_cols);
        assert_eq!(stage5.is_flat_stage(), false);

        let stage6_cols = [
            "169.0 km,Mount Etna,C.1,1736m",
        ];

        let stage6 = edition.get_stage(&String::from("6"));
        assert_eq!(stage6.profile(), stage6_cols);
        assert_eq!(stage6.is_flat_stage(), false);

        let stage7 = edition.get_stage(&String::from("7"));
        assert_eq!(stage7.is_flat_stage(), true);

        let stage8_cols = [
            "209.0 km,Montevergine di Mercogliano,C.2,1260m",
        ];

        let stage8 = edition.get_stage(&String::from("8"));
        assert_eq!(stage8.profile(), stage8_cols);
        assert_eq!(stage8.is_flat_stage(), false);

        let stage9_cols = [
            "108.1 km,Roccaraso,C.2,1252m",
            "192.9 km,Calascio,C.2,1190m"
        ];

        let stage9 = edition.get_stage(&String::from("9"));
        assert_eq!(stage9.profile(), stage9_cols);
        assert_eq!(stage9.is_flat_stage(), false);

        let stage10_cols = [
            "21.8 km,Forte Delia Creta,C.2,1254m",
            "61.2 km,Bruzzolana,C.3,523m",
            "213.5 km,Annifo,C.4,895m"
        ];

        let stage10 = edition.get_stage(&String::from("10"));
        assert_eq!(stage10.profile(), stage10_cols);
        assert_eq!(stage10.is_flat_stage(), false);

        let stage11_cols = [
            "41.7 km,Passo Cornello,C.3,814m",
            "97.5 km,Valico di Pietra Rossa,C.3,674m",
            "156.0 km,Osimo,C.4,265m"
        ];

        let stage11 = edition.get_stage(&String::from("11"));
        assert_eq!(stage11.profile(), stage11_cols);
        assert_eq!(stage11.is_flat_stage(), false);

        let stage12_cols = [
            "206.6 km,Tre Monti,C.4,252m"
        ];

        let stage12 = edition.get_stage(&String::from("12"));
        assert_eq!(stage12.profile(), stage12_cols);
        assert_eq!(stage12.is_flat_stage(), false);

        let stage13_cols = [
            "160.7 km,Montello,C.4,242m"
        ];

        let stage13 = edition.get_stage(&String::from("13"));
        assert_eq!(stage13.profile(), stage13_cols);
        assert_eq!(stage13.is_flat_stage(), false);

        let stage14_cols = [
            "43.3 km,Monte di Ragogna,C.3,494m",
            "106.0 km,Avaglio,C.3,738m",
            "142.5 km,Passo Duron,C.2,1609m",
            "165.8 km,Sella Valcalda Ravascletto,C.3,958m",
            "186.0 km,Monte Zoncolan,C.1,1730m"
        ];

        let stage14 = edition.get_stage(&String::from("14"));
        assert_eq!(stage14.profile(), stage14_cols);
        assert_eq!(stage14.is_flat_stage(), false);

        let stage15_cols = [
            "48.4 km,Passo della Mauria,C.3,1301m",
            "110.6 km,Passo Tre Croci,C.2,1805m",
            "146.9 km,Passo di Sant'Antonio,C.2,1470m",
            "160.6 km,Costalissoio (Bosco dei Giavi),C.2,1300m",
        ];

        let stage15 = edition.get_stage(&String::from("15"));
        assert_eq!(stage15.profile(), stage15_cols);
        assert_eq!(stage15.is_flat_stage(), false);

        let stage16 = edition.get_stage(&String::from("16"));
        assert_eq!(stage16.is_flat_stage(), true);

        let stage17_cols = [
            "71.5 km,Lodrino,C.3,736m"
        ];

        let stage17 = edition.get_stage(&String::from("17"));
        assert_eq!(stage17.profile(), stage17_cols);
        assert_eq!(stage17.is_flat_stage(), false);

        let stage18_cols = [
            "196.0 km,Prato Nevoso,C.1,1607m"
        ];

        let stage18 = edition.get_stage(&String::from("18"));
        assert_eq!(stage18.profile(), stage18_cols);
        assert_eq!(stage18.is_flat_stage(), false);

        let stage19_cols = [
            "48.9 km,Colle del Lys,C.2,1311m",
            "110.7 km,Colle delle Finestre,C.1,2178m",
            "138.4 km,Sestrière,C.3,2035m",
            "185.0 km,Bardonecchia (Monte Jafferau),C.1,1908m"
        ];

        let stage19 = edition.get_stage(&String::from("19"));
        assert_eq!(stage19.profile(), stage19_cols);
        assert_eq!(stage19.is_flat_stage(), false);

        let stage20_cols = [
            "146.5 km,Col Tsecore (Col du Mont-Tseuc),C.1,1623m",
            "185.8 km,Col Saint Pantaléon,C.1,1664m",
            "214.0 km,Cervinia,C.1,2001m"
        ];

        let stage20 = edition.get_stage(&String::from("20"));
        assert_eq!(stage20.profile(), stage20_cols);
        assert_eq!(stage20.is_flat_stage(), false);

        let stage21 = edition.get_stage(&String::from("21"));
        assert_eq!(stage21.is_flat_stage(), true);
    }

    #[test]
    fn test_tour_of_italy_2019() {
        let route = [
            "1,11 May,Bologna to San Luca,8.0 km,Individual time trial",
            "2,12 May,Bologna to Fucecchio,205.0 km,Road stage",
            "3,13 May,Vinci to Orbetello,220.0 km,Road stage",
            "4,14 May,Orbetello to Frascati,235.0 km,Road stage",
            "5,15 May,Frascati to Terracina,140.0 km,Road stage",
            "6,16 May,Cassino to San Giovanni Rotondo,238.0 km,Road stage",
            "7,17 May,Vasto to L'Aquila,185.0 km,Road stage",
            "8,18 May,Tortoreto Lido to Pesaro,239.0 km,Road stage",
            "9,19 May,Riccione to San Marino (San Marino),34.8 km,Individual time trial",
            ",20 May,Rest day",
            "10,21 May,Ravenna to Modena,145.0 km,Road stage",
            "11,22 May,Carpi to Novi Ligure,221.0 km,Road stage",
            "12,23 May,Cuneo to Pinerolo,158.0 km,Road stage",
            "13,24 May,Pinerolo to Ceresole Reale (Lago Serrù),196.0 km,Road stage",
            "14,25 May,Saint-Vincent to Courmayeur (Skyway Monte Bianco),131.0 km,Road stage",
            "15,26 May,Ivrea to Como,232.0 km,Road stage",
            ",27 May,Rest day",
            "16,28 May,Lovere to Ponte di Legno,194.0 km,Road stage",
            "17,29 May,Commezzadura (Val di Sole) to Anterselva/Antholz,181.0 km,Road stage",
            "18,30 May,Valdaora/Olang to Santa Maria di Sala,222.0 km,Road stage",
            "19,31 May,Treviso to San Martino di Castrozza,151.0 km,Road stage",
            "20,1 June,Feltre to Monte Avena,194.0 km,Road stage",
            "21,2 June,Verona,17.0 km,Individual time trial"
        ];

        let edition = tour_of_italy_2019();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");

        let stage1 = edition.get_stage(&String::from("1"));
        assert_eq!(stage1.is_flat_stage(), true);

        let stage2_cols = [
            "157.6 km,Montalbano,C.3,424m",
            "178.7 km,San Baranto,C.4,340m"
        ];

        let stage2 = edition.get_stage(&String::from("2"));
        assert_eq!(stage2.profile(), stage2_cols);
        assert_eq!(stage2.is_flat_stage(), false);

        let stage3_cols = [
            "182.0 km,Poggio L'Appartita,C.4,202m"
        ];

        let stage3 = edition.get_stage(&String::from("3"));
        assert_eq!(stage3.profile(), stage3_cols);
        assert_eq!(stage3.is_flat_stage(), false);

        let stage4_cols = [
            "32.8 km,Manciano,C.4,389m"
        ];

        let stage4 = edition.get_stage(&String::from("4"));
        assert_eq!(stage4.profile(), stage4_cols);
        assert_eq!(stage4.is_flat_stage(), false);

        let stage5_cols = [
            "87.2 km,Sezze,C.4,248m",
        ];

        let stage5 = edition.get_stage(&String::from("5"));
        assert_eq!(stage5.profile(), stage5_cols);
        assert_eq!(stage5.is_flat_stage(), false);

        let stage6_cols = [
            "220.1 km,Coppa Casarinelle,C.2,678m"
        ];

        let stage6 = edition.get_stage(&String::from("6"));
        assert_eq!(stage6.profile(), stage6_cols);
        assert_eq!(stage6.is_flat_stage(), false);

        let stage7_cols = [
            "138.8 km,Le Svolte di Popoli,C.2,746m",
        ];

        let stage7 = edition.get_stage(&String::from("7"));
        assert_eq!(stage7.profile(), stage7_cols);
        assert_eq!(stage7.is_flat_stage(), false);

        let stage8_cols = [
            "168.5 km,Monte della Mattera,C.3,418m",
            "203.7 km,Monteluro,C.4,222m",
            "214.9 km,Gabicce Monte,C.4,120m"
        ];

        let stage8 = edition.get_stage(&String::from("8"));
        assert_eq!(stage8.profile(), stage8_cols);
        assert_eq!(stage8.is_flat_stage(), false);

        assert_eq!(edition.get_stage(&String::from("9")).is_flat_stage(), true);
        assert_eq!(edition.get_stage(&String::from("10")).is_flat_stage(), true);
        assert_eq!(edition.get_stage(&String::from("11")).is_flat_stage(), true);

        let stage12_cols = [
            "125.9 km,Montoso,C.1,1248m"
        ];

        let stage12 = edition.get_stage(&String::from("12"));
        assert_eq!(stage12.profile(), stage12_cols);
        assert_eq!(stage12.is_flat_stage(), false);

        let stage13_cols = [
            "54.3 km,Colle del Lys,C.1,1311m",
            "134.3 km,Pian del Lupo,C.2,1405m",
            "196.0 km,Ceresole Reale (Lago Serrù),C.1,2247m"
        ];

        let stage13 = edition.get_stage(&String::from("13"));
        assert_eq!(stage13.profile(), stage13_cols);
        assert_eq!(stage13.is_flat_stage(), false);

        let stage14_cols = [
            "13.8 km,Verrayes,C.2,1017m",
            "51.5 km,Verrogne,C.1,1582m",
            "75.9 km,Truc d'Arbe,C.2,1256m",
            "106.1 km,Colle San Carlo,C.1,1951m"
        ];

        let stage14 = edition.get_stage(&String::from("14"));
        assert_eq!(stage14.profile(), stage14_cols);
        assert_eq!(stage14.is_flat_stage(), false);

        let stage15_cols = [
            "173.7 km,Madonna del Ghisallo,C.2,754m",
            "189.6 km,Colman di Sormano,C.2,1124m"
        ];

        let stage15 = edition.get_stage(&String::from("15"));
        assert_eq!(stage15.profile(), stage15_cols);
        assert_eq!(stage15.is_flat_stage(), false);

        let stage16_cols = [
            "89.6 km,Cevo,C.3,1054m",
            "128.2 km,Aprica,C.3,1173m",
            "166.3 km,Passo del Mortirolo,C.1,1854m"
        ];

        let stage16 = edition.get_stage(&String::from("16"));
        assert_eq!(stage16.profile(), stage16_cols);
        assert_eq!(stage16.is_flat_stage(), false);

        let stage17_cols = [
            "114.0 km,Elva,C.4,824m",
            "135.3 km,Terento,C.3,1244m",
            "181.0 km,Anterselva/Antholz,C.3,1635m"
        ];

        let stage17 = edition.get_stage(&String::from("17"));
        assert_eq!(stage17.profile(), stage17_cols);
        assert_eq!(stage17.is_flat_stage(), false);

        let stage18_cols = [
            "118.1 km,Pieve di Alpago,C.4,691m"
        ];

        let stage18 = edition.get_stage(&String::from("18"));
        assert_eq!(stage18.profile(), stage18_cols);
        assert_eq!(stage18.is_flat_stage(), false);

        let stage19_cols = [
            "66.6 km,Passo di San Boldo,C.3,701m",
            "116.5 km,Lamon,C.4,594m",
            "151.0 km,San Martino di Castrozza,C.2,1487m"
        ];

        let stage19 = edition.get_stage(&String::from("19"));
        assert_eq!(stage19.profile(), stage19_cols);
        assert_eq!(stage19.is_flat_stage(), false);

        let stage20_cols = [
            "27.1 km,Cima Campo,C.2,1425m",
            "78.0 km,Passo Manghen,C.1,2047m",
            "133.1 km,Passo Rolle,C.2,1980m",
            "183.1 km,Croce d'Aune,C.2,1015m",
            "194.0 km,Monte Avena,C.1,1225m"
        ];

        let stage20 = edition.get_stage(&String::from("20"));
        assert_eq!(stage20.profile(), stage20_cols);
        assert_eq!(stage20.is_flat_stage(), false);

        let stage21_cols = [
            "9.5 km,Torricelle,C.4,277m",
        ];

        let stage21 = edition.get_stage(&String::from("21"));
        assert_eq!(stage21.profile(), stage21_cols);
        assert_eq!(stage21.is_flat_stage(), false);
    }
}
