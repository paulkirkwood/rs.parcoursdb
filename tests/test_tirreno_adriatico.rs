extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::tirreno_adriatico::repository::*;

    #[test]
    fn test_tirreno_adriatico_2013() {
        let route = [
            "P,6 March,San Vincenzo to Donoratico,16.9 km,Individual time trial",
            "1,7 March,San Vincenzo to Indicatore,232.0 km,Road stage",
            "2,8 March,Indicatore to Narni Scalo,190.0 km,Road stage",
            "3,9 March,Narni Scalo to Prati di Tivo,173.0 km,Road stage",
            "4,10 March,Ortona to Chieti,230.0 km,Road stage",
            "5,11 March,Porto Sant'Elpidio,209.0 km,Road stage",
            "6,12 March,San Benedetto del Tronto,9.2 km,Individual time trial"
        ];

        let edition = tirreno_adriatico_2013();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "6 stages + Prologue");
    }

    #[test]
    fn test_tirreno_adriatico_2014() {
        let route = [
            "P,12 March,Donoratico to San Vincenzo,18.5 km,Individual time trial",
            "1,13 March,San Vincenzo to Cascina,166.0 km,Road stage",
            "2,14 March,Cascina to Arezzo,212.0 km,Road stage",
            "3,15 March,Indicatore to Cittareale,244.0 km,Road stage",
            "4,16 March,Amatrice to Guardiagrele,192.0 km,Road stage",
            "5,17 March,Bucchianico to Porto Sant'Elpidio,193.0 km,Road stage",
            "6,18 March,San Benedetto del Tronto,9.1 km,Individual time trial"
        ];

        let edition = tirreno_adriatico_2014();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "6 stages + Prologue");
    }

    #[test]
    fn test_tirreno_adriatico_2015() {
        let route = [
            "1,11 March,Lido di Camaiore,5.4 km,Individual time trial",
            "2,12 March,Camaiore to Cascina,153.0 km,Road stage",
            "3,13 March,Cascina to Arezzo,203.0 km,Road stage",
            "4,14 March,Indicatore to Castelraimondo,226.0 km,Road stage",
            "5,15 March,Esanatoglia to Monte Terminillo,196.9 km,Road stage",
            "6,16 March,Rieti to Porto Sant'Elpidio,210.0 km,Road stage",
            "7,17 March,San Benedetto del Tronto,10.1 km,Individual time trial"
        ];

        let edition = tirreno_adriatico_2015();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "7 stages");
    }

    #[test]
    fn test_tirreno_adriatico_2016() {
        let route = [
            "1,9 March,Lido di Camaiore,22.7 km,Team time trial",
            "2,10 March,Camaiore to Pomarance,207.0 km,Road stage",
            "3,11 March,Castelnuouvo di Val di Cecina to Montalto di Castro,176.0 km,Road stage",
            "4,12 March,Montalto di Castro to Foligno,216.0 km,Road stage",
            "5,13 March,Foligno to Monte San Vicino,178.0 km,Road stage",
            "6,14 March,Castelraimondo to Cepagatti,210.0 km,Road stage",
            "7,15 March,San Benedetto del Tronto,10.1 km,Individual time trial",
        ];

        let edition = tirreno_adriatico_2016();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "7 stages");
    }
}
