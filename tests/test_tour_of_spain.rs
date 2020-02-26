extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::tour_of_spain::repository::*;

    #[test]
    fn test_tour_of_spain_1935() {
        let route = [
            "1,29 April,Madrid to Valladolid,185.0 km,Road stage",
            "2,30 April,Valladolid to Santander,251.0 km,Road stage",
            ",1 May,Rest day",
            "3,2 May,Santander to Bilbao,199.0 km,Road stage",
            "4,3 May,Bilbao to San Sebastián,235.0 km,Road stage",
            "5,4 May,San Sebastián to Zaragoza,264.0 km,Road stage",
            "6,5 May,Zaragoza to Barcelona,310.0 km,Road stage",
            ",6 May,Rest day",
            "7,7 May,Barcelona to Tortosa,188.0 km,Road stage",
            "8,8 May,Tortosa to Valencia,188.0 km,Road stage",
            "9,9 May,Valencia to Murcia,265.0 km,Road stage",
            "10,10 May,Murcia to Granada,285.0 km,Road stage",
            "11,11 May,Granada to Seville,260.0 km,Road stage",
            ",12 May,Rest day",
            "12,13 May,Seville to Cáceres,270.0 km,Road stage",
            "13,14 May,Cáceres to Zamora,275.0 km,Road stage",
            "14,15 May,Zamora to Madrid,250.0 km,Road stage"
        ];

        let edition = tour_of_spain_1935();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "14 stages");
    }

    #[test]
    fn test_tour_of_spain_1936() {
        let route = [
            "1,5 May,Madrid to Salamanca,210.0 km,Road stage",
            "2,6 May,Salamanca to Cáceres,214.0 km,Road stage",
            "3,7 May,Cáceres to Seville,270.0 km,Road stage",
            ",8 May,Rest day",
            "4,9 May,Seville to Málaga,212.0 km,Road stage",
            "5,10 May,Málaga to Granada,132.0 km,Road stage",
            "6,11 May,Granada to Almería,185.0 km,Road stage",
            ",12 May,Rest day",
            "7,13 May,Almería to Alicante,306.0 km,Road stage",
            "8,14 May,Alicante to Valencia,184.0 km,Road stage",
            "9,15 May,Valencia to Tarragona,279.0 km,Road stage",
            ",16 May,Rest day",
            "10,17 May,Tarragona to Barcelona,129.0 km,Road stage",
            "11,18 May,Barcelona to Zaragoza,293.0 km,Road stage",
            "12,19 May,Zaragoza to San Sebastián,265.0 km,Road stage",
            ",20 May,Rest day",
            "13,21 May,San Sebastián to Bilbao,160.0 km,Road stage",
            "14,22 May,Bilbao to Santander,199.0 km,Road stage",
            ",23 May,Rest day",
            "15,24 May,Santander to Gijón,194.0 km,Road stage",
            "16,25 May,Gijón to Ribadeo,155.0 km,Road stage",
            "17,26 May,Ribadeo to A Coruña,157.0 km,Road stage",
            "18,27 May,A Coruña to Vigo,175.0 km,Road stage",
            ",28 May,Rest day",
            "19,29 May,Vigo to Verín,178.0 km,Road stage",
            "20,30 May,Verín to Zamora,207.0 km,Road stage",
            "21,31 May,Zamora to Madrid,250.0 km,Road stage",
        ];

        let edition = tour_of_spain_1936();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_1941() {
        let route = [
            "1,12 June,Madrid to Salamanca,210.0 km,Road stage",
            "2,13 June,Salamanca to Cáceres,214.0 km,Road stage",
            "3,14 June,Cáceres to Seville,270.0 km,Road stage",
            ",15 June,Rest day",
            "4,16 June,Seville to Málaga,212.0 km,Road stage",
            "5,17 June,Málaga to Almería,220.0 km,Road stage",
            "6,18 June,Almería to Murcia,223.0 km,Road stage",
            "7,19 June,Murcia to Valencia,248.0 km,Road stage",
            ",20 June,Rest day",
            "8,21 June,Valencia to Tarragona,279.0 km,Road stage",
            "9,22 June,Tarragona to Barcelona,112.0 km,Road stage",
            "10,23 June,Barcelona to Zaragoza,294.0 km,Road stage",
            "11,24 June,Zaragoza to Logroño,172.0 km,Road stage",
            "12,25 June,Logroño to San Sebastián,213.0 km,Road stage",
            "13,26 June,San Sebastián to Bilbao,160.0 km,Road stage",
            ",27 June,Rest day",
            "14,28 June,Bilbao to Santander,165.0 km,Road stage",
            "15,29 June,Santander to Gijón,192.0 km,Road stage",
            "16a,30 June,Gijón to Oviedo,53.0 km,Individual time trial",
            "16b,30 June,Oviedo to Luarca,101.0 km,Road stage",
            "17,1 July,Luarca to A Coruña,219.0 km,Road stage",
            "18,2 July,A Coruña to Vigo,175.0 km,Road stage",
            ",3 July,Rest day",
            "19,4 July,Vigo to Verín,178.0 km,Road stage",
            "20,5 July,Verín to Valladolid,301.0 km,Road stage",
            "21,6 July,Valladolid to Madrid,198.0 km,Road stage"
         ];

        let edition = tour_of_spain_1941();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1942() {
        let route = [
            "1,30 June,Madrid to Albacete,245.0 km,Road stage",
            "2,1 July,Albacete to Murcia,160.0 km,Road stage",
            "3,2 July,Murcia to Valencia,248.0 km,Road stage",
            ",3 July,Rest day",
            "4,4 July,Valencia to Tarragona,278.0 km,Road stage",
            "5,5 July,Tarragona to Barcelona,120.0 km,Road stage",
            "6,6 July,Barcelona to Huesca,279.0 km,Road stage",
            "7,7 July,Huesca to San Sebastián,305.0 km,Road stage",
            "8,8 July,San Sebastián to Bilbao,160.0 km,Road stage",
            "9,9 July,Bilbao to Castro Urdiales,53.0 km,Individual time trial",
            "10,10 July,Castro Urdiales to Santander,151.0 km,Road stage",
            "11,11 July,Santander to Reinosa,120.0 km,Road stage",
            "12,12 July,Reinosa to Gijón,199.0 km,Road stage",
            "13,13 July,Gijón to Oviedo,75.0 km,Road stage",
            "14,14 July,Oviedo to Luarca,129.0 km,Road stage",
            "15,15 July,Luarca to A Coruña,219.0 km,Road stage",
            "16a,16 July,A Coruña to Santiago de Compostela,63.0 km,Individual time trial",
            "16b,16 July,Santiago de Compostela to Vigo,110.0 km,Road stage",
            "17,17 July,Vigo to Ponferrada,270.0 km,Road stage",
            "18,18 July,Ponferrada to Salamanca,251.0 km,Road stage",
            "19,19 July,Salamanca to Madrid,248.0 km,Road stage"
        ];

        let edition = tour_of_spain_1942();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1945() {
        let route = [
            "1,10 May,Madrid to Salamanca,212.0 km,Road stage",
            "2,11 May,Salamanca to Cáceres,214.0 km,Road stage",
            "3,12 May,Cáceres to Badajoz,132.0 km,Road stage",
            "4a,13 May,Badajoz to Almendralejo,57.0 km,Individual time trial",
            "4b,13 May,Almendralejo to Seville,171.0 km,Road stage",
            ",14 May,Rest day",
            "5,15 May,Seville to Granada,251.0 km,Road stage",
            "6,16 May,Granada to Murcia,285.0 km,Road stage",
            "7,17 May,Murcia to Valencia,244.0 km,Road stage",
            ",18 May,Rest day",
            "8,19 May,Valencia to Tortosa,188.0 km,Road stage",
            "9,20 May,Tortosa to Barcelona,288.0 km,Road stage",
            "10,21 May,Barcelona to Zaragoza,306.0 km,Road stage",
            "11,22 May,Zaragoza to San Sebastián,276.0 km,Road stage",
            ",23 May,Rest day",
            "12,24 May,San Sebastián to Bilbao,207.0 km,Road stage",
            "13,25 May,Bilbao to Santander,188.0 km,Road stage",
            "14,26 May,Santander to Reinosa,110.0 km,Road stage",
            "15,27 May,Reinosa to Gijón,200.0 km,Road stage",
            ",28 May,Rest day",
            "16,29 May,Gijón to León,172.0 km,Road stage",
            "17,30 May,León to Valladolid,132.0 km,Road stage",
            "18,31 May,Valladolid to Madrid,185.0 km,Road stage"
        ];

        let edition = tour_of_spain_1945();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "18 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1946() {
        let route = [
            "1,7 May,Madrid to Salamanca,212.0 km,Road stage",
            "2a,8 May,Salamanca to Béjar,73.0 km,Individual time trial",
            "2b,8 May,Béjar to Cáceres,141.0 km,Road stage",
            "3,9 May,Cáceres to Badajoz,132.0 km,Road stage",
            "4,10 May,Badajoz to Seville,218.0 km,Road stage",
            ",11 May,Rest day",
            "5,12 May,Seville to Granada,251.0 km,Road stage",
            "6,13 May,Granada to Baza,107.0 km,Road stage",
            "7,14 May,Baza to Murcia,178.0 km,Road stage",
            "8,15 May,Murcia to Valencia,264.0 km,Road stage",
            ",16 May,Rest day",
            "9a,17 May,Valencia to Castellón,67.0 km,Team time trial",
            "9b,17 May,Castellón to Tortosa,123.0 km,Road stage",
            "10,18 May,Tortosa to Barcelona,215.0 km,Road stage",
            "11,19 May,Barcelona to Lleida,162.0 km,Road stage",
            "12,20 May,Lleida to Zaragoza,144.0 km,Road stage",
            "13,21 May,Zaragoza to San Sebastián,276.0 km,Road stage",
            ",22 May,Rest day",
            "14,23 May,San Sebastián to Bilbao,207.0 km,Road stage",
            "15,24 May,Bilbao to Santander,226.0 km,Road stage",
            "16,25 May,Santander to Reinosa,110.0 km,Road stage",
            "17,26 May,Reinosa to Gijón,204.0 km,Road stage",
            ",27 May,Rest day",
            "18a,28 May,Gijón to Oviedo,53.0 km,Individual time trial",
            "18b,28 May,Oviedo to León,119.0 km,Road stage",
            "19,29 May,León to Valladolid,134.0 km,Road stage",
            "20,30 May,Valladolid to Madrid,200.0 km,Road stage"
        ];

        let edition = tour_of_spain_1946();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages including 3 split stages");
    }

    #[test]
    fn test_tour_of_spain_1947() {
        let route = [
            "1,12 May,Madrid to Albacete,243.0 km,Road stage",
            "2,13 May,Albacete to Murcia,146.0 km,Road stage",
            "3,14 May,Murcia to Alcoy,135.0 km,Road stage",
            "4,15 May,Alcoy to Castellón,175.0 km,Road stage",
            "5,16 May,Castellón to Tarragona,222.0 km,Road stage",
            ",17 May,Rest day",
            "6,18 May,Tarragona to Barcelona,119.0 km,Road stage",
            "7,19 May,Barcelona to Lleida,162.0 km,Road stage",
            "8,20 May,Lleida to Zaragoza,144.0 km,Road stage",
            "9,21 May,Zaragoza to Pamplona,176.0 km,Road stage",
            "10,22 May,Pamplona to San Sebastián,107.0 km,Road stage",
            "11,23 May,San Sebastián to Bilbao,229.0 km,Road stage",
            "12,24 May,Bilbao to Santander,212.0 km,Road stage",
            "13,25 May,Santander to Reinosa,201.0 km,Road stage",
            "14,26 May,Reinosa to Gijón,204.0 km,Road stage",
            "15,27 May,Gijón to Oviedo,105.0 km,Road stage",
            "16a,28 May,Oviedo to Luarca,101.0 km,Road stage",
            "16b,28 May,Luarca to Ribadeo,70.0 km,Individual time trial",
            "17,29 May,Ribadeo to Ferrol,159.0 km,Road stage",
            "18,30 May,Ferrol to A Coruña,70.0 km,Road stage",
            "19,31 May,A Coruña to Vigo,180.0 km,Road stage",
            "20,1 June,Vigo to Ourense,105.0 km,Road stage",
            "21,2 June,Ourense to Astorga,228.0 km,Road stage",
            "22,3 June,Astorga to León,47.0 km,Individual time trial",
            "23,4 June,León to Valladolid,133.0 km,Road stage",
            "24,5 June,Valladolid to Madrid,220.0 km,Road stage"
        ];

        let edition = tour_of_spain_1947();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "24 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1948() {
        let route = [
            "1a,13 June,Madrid,14.0 km,Individual time trial",
            "1b,13 June,Madrid to Valdepeñas,198.0 km,Road stage",
            "2,14 June,Valdepeñas to Granada,232.0 km,Road stage",
            "3,15 June,Granada to Murcia,285.0 km,Road stage",
            "4,16 June,Murcia to Alicante,230.0 km,Road stage",
            "5,17 June,Alicante to Valencia,163.0 km,Road stage",
            ",18 June,Rest day",
            "6,19 June,Valencia to Tortosa,201.0 km,Road stage",
            "7,20 June,Tortosa to Barcelona,209.0 km,Road stage",
            "8,21 June,Barcelona to Lleida,203.0 km,Road stage",
            "9,22 June,Lleida to Zaragoza,144.0 km,Road stage",
            "10,23 June,Zaragoza to San Sebastián,276.0 km,Road stage",
            ",24 June,Rest day",
            "11,25 June,San Sebastián to Bilbao,259.0 km,Road stage",
            "12,26 June,Bilbao to Santander,212.0 km,Road stage",
            "13,27 June,Santander to Gijón,225.0 km,Road stage",
            "14,28 June,Gijón to Ribadeo,200.0 km,Road stage",
            "15,29 June,Ribadeo to A Coruña,156.0 km,Road stage",
            ",30 June,Rest day",
            "16,1 July,A Coruña to Ourense,156.0 km,Road stage",
            "17,2 July,Ourense to León,276.0 km,Road stage",
            "18,3 July,León to Segovia,269.0 km,Road stage",
            "19,4 July,Segovia to Madrid,100.0 km,Road stage"
        ];

        let edition = tour_of_spain_1948();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1950() {
        let route = [
            "1,17 August,Madrid to Valladolid,190.0 km,Road stage",
            "2,18 August,Valladolid to León,133.0 km,Road stage",
            "3,19 August,León to Gijón,148.0 km,Road stage",
            "4a,20 August,Gijón to Torrelavega,167.0 km,Road stage",
            "4b,20 August,Torrelavega to Santander,78.0 km,Road stage",
            "5,21 August,Santander to Bilbao,177.0 km,Road stage",
            "6,22 August,Bilbao to Irun,240.0 km,Road stage",
            ",23 August,Rest day",
            "7,24 August,Irun to Pamplona,109.0 km,Road stage",
            "8a,25 August,Pamplona to Tudela,90.0 km,Individual time trial",
            "8b,25 August,Tudela to Zaragoza,176.0 km,Road stage",
            "9,26 August,Zaragoza to Lleida,144.0 km,Road stage",
            "10,27 August,Lleida to Barcelona,167.0 km,Road stage",
            ",28 August,Rest day",
            "11,29 August,Barcelona to Tarragona,150.0 km,Road stage",
            "12,30 August,Tarragona to Castellón,194.0 km,Road stage",
            "13,31 August,Castellón to Valencia,65.0 km,Individual time trial",
            "14,1 September,Valencia to Murcia,265.0 km,Road stage",
            "15,2 September,Murica to Lorca,117.0 km,Road stage",
            "16,3 September,Lorca to Granada,22.0 km,Road stage",
            "17,4 September,Granada to Málaga,183.0 km,Road stage",
            ",5 September,Rest day",
            "18,6 September,Málaga to Cádiz,268.0 km,Road stage",
            "19a,7 September,Cádiz to Jerez de la Frontera,56.0 km,Individual time trial",
            "19b,7 September,Jerez de la Frontera to Seville,100.0 km,Road stage",
            "20,8 September,Seville to Mérida,200.0 km,Road stage",
            "21,9 September,Mérida to Talavera de la Reina,228.0 km,Road stage",
            "22,10 September,Talavera de la Reina to Madrid,117.0 km,Road stage"
        ];

        let edition = tour_of_spain_1950();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 3 split stages");
    }

    #[test]
    fn test_tour_of_spain_1955() {
        let route = [
            "1,23 April,Bilbao to San Sebastián,240.0 km,Road stage",
            "2,24 April,San Sebastián to Bayonne (France),211.0 km,Road stage",
            "3,25 April,Bayonne (France) to Pamplona,157.0 km,Road stage",
            "4,26 April,Pamplona to Zaragoza,229.0 km,Road stage",
            "5,27 April,Zaragoza to Lleida,195.0 km,Road stage",
            "6,28 April,Lleida to Barcelona,230.0 km,Road stage",
            ",29 April,Rest day",
            "7,30 April,Barcelona,29.0 km,Individual time trial",
            "8,1 May,Barcelona to Tortosa,213.0 km,Road stage",
            "9,2 May,Tortosa to Valencia,190.0 km,Road stage",
            "10,3 May,Valencia to Cuenca,222.0 km,Road stage",
            "11,4 May,Cuenca to Madrid,168.0 km,Road stage",
            "12,5 May,Madrid,15.0 km,Team time trial",
            "13,6 May,Madrid to Valladolid,222.0 km,Road stage",
            "14,7 May,Valladolid to Bilbao,308.0 km,Road stage",
            "15,8 May,Bilbao,147.0 km,Road stage"
        ];

        let edition = tour_of_spain_1955();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_of_spain_1956() {
        let route = [
            "1,26 April,Bilbao to Santander,203.0 km,Road stage",
            "2,27 April,Santander to Oviedo,248.0 km,Road stage",
            "3,28 April,Oviedo to Valladolid,175.0 km,Road stage",
            "4,29 April,Valladolid to Madrid,212.0 km,Road stage",
            "5,30 April,Madrid to Albacete,241.0 km,Road stage",
            "6,1 May,Albacete to Alicante,227.0 km,Road stage",
            "7,2 May,Alicante to Valencia,182.0 km,Road stage",
            "8,3 May,Valencia to Tarragona,249.0 km,Road stage",
            ",4 May,Rest day",
            "9,5 May,Tarragona to Barcelona,163.0 km,Road stage",
            "10a,6 May,Barcelona,21.0 km,Team time trial",
            "10b,6 May,Barcelona to Tàrrega,112.0 km,Road stage",
            "11,7 May,Tàrrega to Zaragoza,238.0 km,Road stage",
            "12,8 May,Zaragoza to Bayonne (France),274.0 km,Road stage",
            "13a,9 May,Bayonne (France) to Irun,43.0 km,Individual time trial",
            "13b,9 May,Irun to Pamplona,111.0 km,Road stage",
            "14,10 May,Pamplona to San Sebastián,195.0 km,Road stage",
            "15,11 May,San Sebastián to Bilbao,225.0 km,Road stage",
            "16,12 May,Bilbao to Vitoria,207.0 km,Road stage",
            "17,13 May,Vitoria to Bilbao,190.0 km,Road stage"
        ];

        let edition = tour_of_spain_1956();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1957() {
        let route = [
            "1,26 April,Bilbao to Vitoria,158.0 km,Road stage",
            "2,27 April,Vitoria to Santander,220.0 km,Road stage",
            "3,28 April,Santander to Mieres,259.0 km,Road stage",
            "4,29 April,Mieres to León,0.0 km,Road stage",
            "5,30 April,León to Valladolid,172.0 km,Road stage",
            "6,1 May,Valladolid to Madrid,212.0 km,Road stage",
            "7,2 May,Madrid,200.0 km,Road stage",
            "8,3 May,Madrid to Cuenca,159.0 km,Road stage",
            ",4 May,Rest day",
            "9,5 May,Cuenca to Valencia,249.0 km,Road stage",
            "10,6 May,Valencia to Tortosa,192.0 km,Road stage",
            "11,7 May,Tortosa to Barcelona,199.0 km,Road stage",
            "12,8 May,Igualada to Zaragoza,229.0 km,Road stage",
            "13,9 May,Zaragoza to Huesca,85.0 km,Individual time trial",
            "14,10 May,Huesca to Bayonne (France),249.0 km,Road stage",
            "15,11 May,Bayonne (France) to San Sebastián,199.0 km,Road stage",
            "16,12 May,San Sebastián to Bilbao,193.0 km,Road stage"
        ];

        let edition = tour_of_spain_1957();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "16 stages");
    }

    #[test]
    fn test_tour_of_spain_1958() {
        let route = [
            "1,30 April,Bilbao to San Sebastián,164.0 km,Road stage",
            "2,1 May,San Sebastián to Pamplona,150.0 km,Road stage",
            "3,2 May,Pamplona to Zaragoza,245.0 km,Road stage",
            "4,3 May,Zaragoza to Barcelona,229.0 km,Road stage",
            "5a,4 May,Barcelona,4.0 km,Team time trial",
            "5b,4 May,Barcelona to Tarragona,119.0 km,Road stage",
            "6,5 May,Tarragona to Valencia,263.0 km,Road stage",
            "7,6 May,Valencia to Cuenca,216.0 km,Road stage",
            "8,7 May,Cuenca to Toledo,206.0 km,Road stage",
            "9,8 May,Toledo to Madrid,241.0 km,Road stage",
            "10,9 May,Madrid to Soria,225.0 km,Road stage",
            "11,10 May,Soria to Vitoria,167.0 km,Road stage",
            "12,11 May,Vitoria to Bilbao,169.0 km,Road stage",
            "13a,12 May,Bilbao to Castro Urdiales,35.0 km,Individual time trial",
            "13b,12 May,Castro Urdiales to Santander,105.0 km,Road stage",
            "14,13 May,Santander to Gijón,221.0 km,Road stage",
            "15,14 May,Oviedo to Palencia,246.0 km,Road stage",
            "16,15 May,Palencia to Madrid,241.0 km,Road stage"
        ];

        let edition = tour_of_spain_1958();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "16 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1959() {
        let route = [
            "1a,29 April,Madrid,9.0 km,Team time trial",
            "1b,29 April,Madrid to Toledo,114.0 km,Road stage",
            "2,30 April,Manzanares to Córdoba,228.0 km,Road stage",
            "3,1 May,Córdoba to Seville,140.0 km,Road stage",
            "4,2 May,Seville to Granada,240.0 km,Road stage",
            "5,3 May,Guadix to Murcia,225.0 km,Road stage",
            "6,4 May,Murcia to Alicante,173.0 km,Road stage",
            "7,5 May,Alicante to Castellón,233.0 km,Road stage",
            "8,6 May,Castellón to Tortosa,130.0 km,Road stage",
            "9,7 May,Tortosa to Barcelona,196.0 km,Road stage",
            "10,8 May,Granollers to Lleida,183.0 km,Road stage",
            "11,9 May,Lleida to Pamplona,242.0 km,Road stage",
            "12,10 May,Pamplona to San Sebastián,210.0 km,Road stage",
            "13,11 May,San Sebastián,9.0 km,Team time trial",
            "14,12 May,Eibar to Vitoria,62.0 km,Individual time trial",
            "15,13 May,Vitoria to Santander,230.0 km,Road stage",
            "16,14 May,Santander to Bilbao,187.0 km,Road stage",
            "17,15 May,Bilbao,222.0 km,Road stage"
        ];

        let edition = tour_of_spain_1959();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1960() {
        let route = [
            "1,29 April,Gijón,8.0 km,Team time trial",
            "2,30 April,Gijón to A Coruña,235.0 km,Road stage",
            "3,1 May,A Coruña to Vigo,187.0 km,Road stage",
            "4,2 May,Vigo to Ourense,105.0 km,Road stage",
            "5,3 May,Ourense to Zamora,287.0 km,Road stage",
            "6,4 May,Zamora to Madrid,250.0 km,Road stage",
            "7,5 May,Madrid,209.0 km,Road stage",
            "8,6 May,Guadalajara to Zaragoza,264.0 km,Road stage",
            "9,7 May,Zaragoza to Barcelona,269.0 km,Road stage",
            "10,8 May,Barcelona to Barbastro,240.0 km,Road stage",
            "11,9 May,Barbastro to Pamplona,267.0 km,Road stage",
            "12,10 May,Pamplona to Logroño,179.0 km,Road stage",
            "13,11 May,Logroño to San Sebastián,211.0 km,Road stage",
            "14,12 May,San Sebastián to Vitoria,263.0 km,Road stage",
            "15,13 May,Vitoria to Santander,232.0 km,Road stage",
            "16,14 May,Santander to Bilbao,192.0 km,Road stage",
            "17a,15 May,Bilbao to Guernica,116.0 km,Road stage",
            "17b,15 May,Guernica to Bilbao,53.0 km,Individual time trial"
        ];
        let edition = tour_of_spain_1960();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1961() {
        let route = [
            "1a,26 April,San Sebastián,10.5 km,Team time trial",
            "1b,26 April,San Sebastián to Pamplona,91.0 km,Road stage",
            "2,27 April,Pamplona,174.0 km,Road stage",
            "3,28 April,Pamplona to Huesca,174.0 km,Road stage",
            "4,29 April,Binéfar to Barcelona,199.0 km,Road stage",
            "5,30 April,Barcelona to Tortosa,185.0 km,Road stage",
            "6,1 May,Tortosa to Valencia,188.0 km,Road stage",
            "7,2 May,Valencia to Benidorm,141.0 km,Road stage",
            "8,3 May,Benidorm to Albacete,211.0 km,Road stage",
            "9,4 May,Albacete to Madrid,198.0 km,Road stage",
            "10,5 May,Madrid,195.0 km,Road stage",
            "11,6 May,Madrid to Valladolid,189.0 km,Road stage",
            "12,7 May,Valladolid to Palencia,48.0 km,Individual time trial",
            "13,8 May,Palencia to Santander,220.0 km,Road stage",
            "14,9 May,Santander to Vitoria,235.0 km,Road stage",
            "15,10 May,Vitoria to Bilbao,179.0 km,Road stage",
            "16,11 May,Bilbao,159.0 km,Road stage"
        ];
        let edition = tour_of_spain_1961();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "16 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1962() {
        let route = [
            "1,27 April,Barcelona,90.0 km,Road stage",
            "2,28 April,Barcelona to Tortosa,185.0 km,Road stage",
            "3,29 April,Tortosa to Valencia,188.0 km,Road stage",
            "4,30 April,Valencia to Benidorm,141.0 km,Road stage",
            "5,1 May,Benidorm,21.0 km,Team time trial",
            "6,2 May,Benidorm to Cartagena,152.0 km,Road stage",
            "7,3 May,Murcia to Almería,223.0 km,Road stage",
            "8,4 May,Almería to Málaga,220.0 km,Road stage",
            "9,5 May,Málaga to Córdoba,193.0 km,Road stage",
            "10,6 May,Valapenas to Madrid,210.0 km,Road stage",
            "11,7 May,Madrid to Valladolid,189.0 km,Road stage",
            "12,8 May,Valladolid to Logroño,232.0 km,Road stage",
            "13,9 May,Logroño to Pamplona,191.0 km,Road stage",
            "14,10 May,Pamplona to Bayonne (France),149.0 km,Road stage",
            "15,11 May,Bayonne (France) to San Sebastián,82.0 km,Individual time trial",
            "16,12 May,San Sebastián to Vitoria,177.0 km,Road stage",
            "17,13 May,Vitoria to Bilbao,171.0 km,Road stage"
        ];
        let edition = tour_of_spain_1962();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages");
    }

    #[test]
    fn test_tour_of_spain_1963() {
        let route = [
            "1a,1 May,Gijón to Mieres,45.0 km,Road stage",
            "1b,1 May,Mieres to Gijón,52.0 km,Individual time trial",
            "2,2 May,Gijón to Torrelavega,175.0 km,Road stage",
            "3,3 May,Torrelavega to Vitoria,249.0 km,Road stage",
            "4,4 May,Vitoria to Bilbao,104.0 km,Road stage",
            "5,5 May,Bilbao,191.0 km,Road stage",
            "6,6 May,Bilbao to Eibar,165.0 km,Road stage",
            "7,7 May,Eibar to Tolosa,138.0 km,Road stage",
            "8,8 May,Tolosa to Pamplona,169.0 km,Road stage",
            "9,9 May,Pamplona to Zaragoza,180.0 km,Road stage",
            "10,10 May,Zaragoza to Lleida,144.0 km,Road stage",
            "11,11 May,Lleida to Barcelona,182.0 km,Road stage",
            "12a,12 May,Barcelona,80.0 km,Road stage",
            "12b,12 May,Sitges to Tarragona,52.0 km,Individual time trial",
            "13,13 May,Tarragona to Valencia,252.0 km,Road stage",
            "14,14 May,Cuenca to Madrid,177.0 km,Road stage",
            "15,15 May,Madrid,87.0 km,Road stage"
        ];
        let edition = tour_of_spain_1963();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1964() {
        let route = [
            "1a,30 April,Benidorm to Barcelona,42.0 km,Road stage",
            "1b,30 April,Benidorm,11.0 km,Individual time trial",
            "2,1 May,Benidorm to Nules,199.0 km,Road stage",
            "3,2 May,Nules to Salou,212.0 km,Road stage",
            "4a,3 May,Salou to Barcelona,115.0 km,Road stage",
            "4b,3 May,Barcelona,49.0 km,Road stage",
            "5,4 May,Barcelona to Puigcerdà,174.0 km,Road stage",
            "6,5 May,Puigcerdà to Lleida,187.0 km,Road stage",
            "7,6 May,Lleida to Jaca,201.0 km,Road stage",
            "8,7 May,Jaca to Pamplona,205.0 km,Road stage",
            "9,8 May,Pamplona to San Sebastián,205.0 km,Road stage",
            "10,9 May,San Sebastián to Bilbao,197.0 km,Road stage",
            "11,10 May,Bilbao to Vitoria,107.0 km,Road stage",
            "12,11 May,Vitoria to Santander,211.0 km,Road stage",
            "13,12 May,Santander to Avilés,230.0 km,Road stage",
            "14,13 May,Avilés to León,163.0 km,Road stage",
            "15,14 May,Becilla to Valladolid,65.0 km,Individual time trial",
            "16,15 May,Valladolid to Madrid,209.0 km,Road stage",
            "17,16 May,Madrid,87.0 km,Road stage"
        ];
        let edition = tour_of_spain_1964();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1965() {
        let route = [
            "1,29 April,Vigo,168.0 km,Road stage",
            "2,30 April,Pontevedra to Lugo,150.0 km,Road stage",
            "3,1 May,Lugo to Gijón,247.0 km,Road stage",
            "4a,2 May,Mieres to Pajares,41.0 km,Individual time trial",
            "4b,2 May,Pajares to Palencia,189.0 km,Road stage",
            "5,3 May,Palencia to Madrid,238.0 km,Road stage",
            "6,4 May,Madrid to Cuenca,161.0 km,Road stage",
            "7,5 May,Albacete to Benidorm,212.0 km,Road stage",
            "8,6 May,Benidorm to Sagunto,174.0 km,Road stage",
            "9,7 May,Sagunto to Salou,237.0 km,Road stage",
            "10a,8 May,Salou to Barcelona,115.0 km,Road stage",
            "10b,8 May,Barcelona,50.0 km,Road stage",
            "11,9 May,Barcelona to Andorra la Vella (Andorra),241.0 km,Road stage",
            "12,10 May,Andorra la Vella (Andorra) to Lleida,158.0 km,Road stage",
            "13,11 May,Lleida to Zaragoza,190.0 km,Road stage",
            "14,12 May,Zaragoza to Pamplona,193.0 km,Road stage",
            "15,13 May,Pamplona to Bayonne (France),149.0 km,Road stage",
            "16,14 May,Saint-Pée-sur-Nivelle (France) to San Sebastián,61.0 km,Individual time trial",
            "17,15 May,San Sebastián to Vitoria,214.0 km,Road stage",
            "18,16 May,Vitoria to Bilbao,222.0 km,Road stage"
        ];
        let edition = tour_of_spain_1965();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "18 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1966() {
        let route = [
            "1a,28 April,Murcia,111.0 km,Road stage",
            "1b,28 April,Murcia,3.5 km,Individual time trial",
            "2a,29 April,Murcia to La Manga,81.0 km,Road stage",
            "2b,29 April,La Manga to Benidorm,153.0 km,Road stage",
            "3,30 April,Benidorm to Valencia,148.0 km,Road stage",
            "4,1 May,Cuenca to Madrid,177.0 km,Road stage",
            "5,2 May,Madrid,181.0 km,Road stage",
            "6,3 May,Madrid to Calatayud,225.0 km,Road stage",
            "7,4 May,Calatayud to Zaragoza,105.0 km,Road stage",
            "8,5 May,Zaragoza to Lleida,144.0 km,Road stage",
            "9,6 May,Lleida to Las Colinas,128.0 km,Road stage",
            "10a,7 May,Sitges to Barcelona,40.0 km,Road stage",
            "10b,7 May,Barcelona,49.0 km,Road stage",
            "11,8 May,Barcelona to Huesca,266.0 km,Road stage",
            "12,9 May,Huesca to Pamplona,221.0 km,Road stage",
            "13,10 May,Pamplona to San Sebastián,178.0 km,Road stage",
            "14,11 May,San Sebastián to Vitoria,178.0 km,Road stage",
            "15a,12 May,Vitoria to Haro,61.0 km,Individual time trial",
            "15b,12 May,Haro to Logroño,52.0 km,Road stage",
            "16,13 May,Logroño to Burgos,116.0 km,Road stage",
            "17,14 May,Burgos to Santander,226.0 km,Road stage",
            "18,15 May,Santander to Bilbao,154.0 km,Road stage"
        ];
        let edition = tour_of_spain_1966();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "18 stages including 4 split stages");
    }

    #[test]
    fn test_tour_of_spain_1967() {
        let route = [
            "1a,27 April,Vigo to O Baixo Miño,110.0 km,Road stage",
            "1b,27 April,Vigo,4.1 km,Individual time trial",
            "2,28 April,Pontevedra to Ourense,186.0 km,Road stage",
            "3,29 April,Ourense to Astorga,230.0 km,Road stage",
            "4,30 April,Astorga to Salamanca,201.0 km,Road stage",
            "5,1 May,Salamanca to Madrid,201.0 km,Road stage",
            "6,2 May,Albacete to Benidorm,212.0 km,Road stage",
            "7,3 May,Benidorm to Valencia,148.0 km,Road stage",
            "8,4 May,Valencia to Vinaròs,145.0 km,Road stage",
            "9,5 May,Vinaròs to Sitges,172.0 km,Road stage",
            "10a,6 May,Sitges to Barcelona,39.0 km,Road stage",
            "10b,6 May,Barcelona,45.4 km,Road stage",
            "11,7 May,Barcelona to Andorra la Vella (Andorra),241.0 km,Road stage",
            "12,8 May,Andorra la Vella (Andorra) to Lleida,158.0 km,Road stage",
            "13,9 May,Lleida to Zaragoza,182.0 km,Road stage",
            "14,10 May,Zaragoza to Pamplona,193.0 km,Road stage",
            "15a,11 May,Pamplona to Logroño,92.0 km,Road stage",
            "15b,11 May,Laguardia to Vitoria,44.0 km,Individual time trial",
            "16,12 May,Vitoria to San Sebastián,139.0 km,Road stage",
            "17,13 May,Villabona to Zarautz,28.0 km,Individual time trial",
            "18,14 May,Zarautz to Bilbao,175.0 km,Road stage"
        ];
        let edition = tour_of_spain_1967();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "18 stages including 3 split stages");
    }

    #[test]
    fn test_tour_of_spain_1968() {
        let route = [
            "1a,25 April,Zaragoza,130.0 km,Road stage",
            "1b,25 April,Zaragoza,4.0 km,Individual time trial",
            "2,26 April,Zaragoza to Lleida,195.0 km,Road stage",
            "3a,27 April,Lleida to Barcelona,165.0 km,Road stage",
            "3b,27 April,Barcelona,38.0 km,Road stage",
            "4,28 April,Barcelona to Salou,108.0 km,Road stage",
            "5,29 April,Salou to Vinaròs,106.0 km,Road stage",
            "6,30 April,Vinaròs to Valencia,148.0 km,Road stage",
            "7,1 May,Valencia to Benidorm,144.0 km,Road stage",
            "8,2 May,Benidorm to Almansa,167.0 km,Road stage",
            "9,3 May,Almansa to Alcázar de San Juan,230.0 km,Road stage",
            "10,4 May,Alcázar de San Juan to Madrid,173.0 km,Road stage",
            "11,5 May,Madrid to Palencia,242.0 km,Road stage",
            "12,6 May,Villalón de Campos to Gijón,236.0 km,Road stage",
            "13,7 May,Gijón to Santander,203.0 km,Road stage",
            "14,8 May,Santander to Vitoria,244.0 km,Road stage",
            "15,9 May,Vitoria to Pamplona,0.0 km,Road stage",
            "16,10 May,Pamplona to San Sebastián,204.0 km,Road stage",
            "17,11 May,San Sebastián to Tolosa,67.0 km,Individual time trial",
            "18,12 May,Tolosa to Bilbao,206.0 km,Road stage"
        ];
        let edition = tour_of_spain_1968();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "18 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1969() {
        let route = [
            "1A,23 April,Badajoz,6.5 km,Individual time trial",
            "1B,24 April,Badajoz,246.0 km,Road stage",
            "2,25 April,Badajoz to Cáceres,135.0 km,Road stage",
            "3,26 April,Cáceres to Talavera de la Reina,190.0 km,Road stage",
            "4,27 April,Talavera de la Reina to Madrid,124.0 km,Road stage",
            "5,28 April,Madrid to Alcázar de San Juan,162.0 km,Road stage",
            "6,29 April,Alcázar de San Juan to Almansa,231.0 km,Road stage",
            "7,30 April,Almansa to Nules,233.0 km,Road stage",
            "8,1 May,Nules to Benicàssim,199.0 km,Road stage",
            "9,2 May,Benicàssim to Reus,169.0 km,Road stage",
            "10,3 May,Reus to Barcelona,146.0 km,Road stage",
            "11,4 May,Barcelona to Sant Feliu de Guixols,118.0 km,Road stage",
            "12,5 May,Sant Feliu de Guixols to Moià,151.0 km,Road stage",
            "13,6 May,Moià to Barbastro,229.0 km,Road stage",
            "14a,7 May,Barbastro to Zaragoza,125.0 km,Road stage",
            "14b,7 May,Zaragoza,4.0 km,Individual time trial",
            "15,8 May,Zaragoza to Pamplona,176.0 km,Road stage",
            "16,9 May,Irun to San Sebastián,25.0 km,Individual time trial",
            "17,10 May,San Sebastián to Vitoria,129.0 km,Road stage",
            "18a,11 May,Vitoria to Llodio,76.0 km,Road stage",
            "18b,11 May,Llodio to Bilbao,29.0 km,Individual time trial"
        ];
        let edition = tour_of_spain_1969();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1970() {
        let route = [
            "P,23 April,Cádiz,6.0 km,Individual time trial",
            "1,24 April,Cádiz to Jerez de la Frontera,170.0 km,Road stage",
            "2,25 April,Jerez de la Frontera to Fuengirola,217.0 km,Road stage",
            "3,26 April,Fuengirola to Almería,249.0 km,Road stage",
            "4,27 April,Almería to Lorca,161.0 km,Road stage",
            "5,28 April,Lorca to Calp,209.0 km,Road stage",
            "6,29 April,Calp to Borriana,198.0 km,Road stage",
            "7,30 April,Borriana to Tarragona,201.0 km,Road stage",
            "8a,1 May,Tarragona to Barcelona,100.0 km,Road stage",
            "8b,1 May,Barcelona,48.0 km,Road stage",
            "9,2 May,Barcelona to Igualada,189.0 km,Road stage",
            "10,3 May,Igualada to Zaragoza,237.0 km,Road stage",
            "11,4 May,Zaragoza to Calatayud,118.0 km,Road stage",
            "12,5 May,Calatayud to Madrid,204.0 km,Road stage",
            "13,6 May,Madrid to Soria,221.0 km,Road stage",
            "14,7 May,Soria to Valladolid,238.0 km,Road stage",
            "15,8 May,Valladolid to Burgos,134.0 km,Road stage",
            "16,9 May,Burgos to Santander,179.0 km,Road stage",
            "17,10 May,Santander to Vitoria,191.0 km,Road stage",
            "18,11 May,Vitoria to San Sebastián,157.0 km,Road stage",
            "19a,12 May,San Sebastián to Llodio,104.0 km,Road stage",
            "19b,12 May,Llodio to Bilbao,29.0 km,Individual time trial"
        ];

        let edition = tour_of_spain_1970();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1971() {
        let route = [
            "P,29 April,Almería,4.2 km,Individual time trial",
            "1,30 April,Almería,126.0 km,Road stage",
            "2,1 May,Águilas to Calp,245.0 km,Road stage",
            "3,2 May,Calp to La Pobla de Farnals,164.0 km,Road stage",
            "4,3 May,La Pobla de Farnals to Benicàssim,175.0 km,Road stage",
            "5,4 May,Benicàssim to Salou,172.0 km,Road stage",
            "6,5 May,Salou to Barcelona,149.0 km,Road stage",
            "7,6 May,Barcelona to Manresa,179.0 km,Road stage",
            "8,7 May,Balaguer to Jaca,211.0 km,Road stage",
            "9,8 May,Jaca to Pamplona,175.0 km,Road stage",
            "10,9 May,Pamplona to San Sebastián,120.0 km,Road stage",
            "11a,10 May,San Sebastián to Bilbao,140.0 km,Road stage",
            "11b,10 May,Bilbao,2.6 km,Individual time trial",
            "12,11 May,Bilbao to Vitoria,185.0 km,Road stage",
            "13,12 May,Vitoria to Torrelavega,208.0 km,Road stage",
            "14,13 May,Torrelavega to Burgos,192.0 km,Road stage",
            "15,14 May,Burgos to Segovia,188.0 km,Road stage",
            "16,15 May,Segovia to Ávila,114.0 km,Road stage",
            "17a,16 May,Ávila to Madrid,138.0 km,Road stage",
            "17b,16 May,Madrid,5.3 km,Individual time trial"
        ];
        let edition = tour_of_spain_1971();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages + Prologue including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1972() {
        let route = [
            "P,27 April,Fuengirola,6.0 km,Individual time trial",
            "1,28 April,Fuengirola to Cabra,167.0 km,Road stage",
            "2,29 April,Cabra to Granada,206.0 km,Road stage",
            "3,30 April,Granada to Almería,181.0 km,Road stage",
            "4,1 May,Almería to Dehesa de Campoamor,251.0 km,Road stage",
            "5,2 May,Dehesa de Campoamor to Gandia,183.0 km,Road stage",
            "6a,3 May,Gandia to El Saler,120.0 km,Road stage",
            "6b,3 May,El Saler,6.5 km,Team time trial",
            "7,4 May,Valencia to Vinaròs,181.0 km,Road stage",
            "8,5 May,Vinaròs to Tarragona,189.0 km,Road stage",
            "9a,6 May,Tarragona to Barcelona,118.0 km,Road stage",
            "9b,6 May,Barcelona,10.0 km,Individual time trial",
            "10,7 May,Barcelona to Banyoles,192.0 km,Road stage",
            "11,8 May,Manresa to Zaragoza,259.0 km,Road stage",
            "12,9 May,Zaragoza to Formigal,169.0 km,Road stage",
            "13,10 May,Sangüesa to Arrate,201.0 km,Road stage",
            "14,11 May,Eibar to Bilbao,145.0 km,Road stage",
            "15,12 May,Bilbao to Torrelavega,148.0 km,Road stage",
            "16,13 May,Torrelavega to Vitoria,219.0 km,Road stage",
            "17a,14 May,Vitoria to San Sebastián,138.0 km,Road stage",
            "17b,14 May,San Sebastián,20.0 km,Individual time trial"
        ];
        let edition = tour_of_spain_1972();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages + Prologue including 3 split stages");
    }

    #[test]
    fn test_tour_of_spain_1973() {
        let route = [
            "P,26 April,Calp,5.0 km,Individual time trial",
            "1,27 April,Calp to Murcia,187.0 km,Road stage",
            "2,28 April,Murcia to Albacete,156.0 km,Road stage",
            "3,29 April,Albacete to Alcázar de San Juan,146.0 km,Road stage",
            "4,30 April,Alcázar de San Juan to Cuenca,169.0 km,Road stage",
            "5,1 May,Cuenca to Teruel,191.0 km,Road stage",
            "6a,2 May,Teruel to La Pobla de Farnals,150.0 km,Road stage",
            "6b,2 May,La Pobla de Farnals,5.0 km,Team time trial",
            "7,3 May,La Pobla de Farnals to Castellón de la Plana,165.0 km,Road stage",
            "8,4 May,Castellón de la Plana to Calafell,245.0 km,Road stage",
            "9a,5 May,Calafell to Barcelona,80.0 km,Road stage",
            "9b,5 May,Barcelona,37.9 km,Individual time trial",
            "10,6 May,Barcelona to Empuriabrava,171.0 km,Road stage",
            "11,7 May,Empuriabrava to Manresa,225.0 km,Road stage",
            "12,8 May,Manresa to Zaragoza,259.0 km,Road stage",
            "13,9 May,Mallen to Irache,175.0 km,Road stage",
            "14,10 May,Irache to Bilbao,182.0 km,Road stage",
            "15a,11 May,Bilbao to Torrelavega,154.0 km,Road stage",
            "15b,11 May,Torrelavega,17.4 km,Individual time trial",
            "16,12 May,Torrelavega to Miranda de Ebro,203.0 km,Road stage",
            "17a,13 May,Miranda de Ebro to Tortosa,127.0 km,Road stage",
            "17b,13 May,Hernani to San Sebastián,10.5 km,Individual time trial"
        ];
        let edition = tour_of_spain_1973();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages + Prologue including 4 split stages");
    }

    #[test]
    fn test_tour_of_spain_1974() {
        let route = [
            "P,23 April,Almería,5.0 km,Individual time trial",
            "1,24 April,Almería,98.0 km,Road stage",
            "2,25 April,Almería to Granada,187.0 km,Road stage",
            "3,26 April,Granada to Fuengirola,161.0 km,Road stage",
            "4,27 April,Marbella to Seville,206.0 km,Road stage",
            "5,28 April,Seville to Córdoba,139.0 km,Road stage",
            "6,29 April,Córdoba to Ciudad Real,211.0 km,Road stage",
            "7,30 April,Ciudad Real to Toledo,126.0 km,Road stage",
            "8a,1 May,Toledo to Madrid,167.0 km,Road stage",
            "8b,1 May,Circuito del Jarama,4.0 km,Team time trial",
            "9,2 May,Madrid to Los Ángeles de San Rafael,158.0 km,Road stage",
            "10a,3 May,Los Ángeles de San Rafael,5.0 km,Individual time trial",
            "10b,3 May,Los Ángeles de San Rafael to Ávila,125.0 km,Road stage",
            "11,4 May,Ávila to Valladolid,168.0 km,Road stage",
            "12,5 May,Valladolid to León,203.0 km,Road stage",
            "13,6 May,León to Monte Naranco,128.0 km,Road stage",
            "14,7 May,Oviedo to Cangas de Onís,134.0 km,Road stage",
            "15,8 May,Cangas de Onís to Laredo,210.0 km,Road stage",
            "16,9 May,Laredo to Bilbao,133.0 km,Road stage",
            "17,10 May,Bilbao to Miranda de Ebro,157.0 km,Road stage",
            "18,11 May,Miranda de Ebro to Eibar,152.0 km,Road stage",
            "19a,12 May,Eibar to San Sebastián,79.0 km,Road stage",
            "19b,12 May,San Sebastián,35.9 km,Individual time trial"
        ];
        let edition = tour_of_spain_1974();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 3 split stages");
    }

    #[test]
    fn test_tour_of_spain_1975() {
        let route = [
            "P,22 April,Fuengirola,4.4 km,Individual time trial",
            "1,23 April,Marbella,78.0 km,Road stage",
            "2,24 April,Málaga to Granada,143.0 km,Road stage",
            "3,25 April,Granada,179.0 km,Road stage",
            "4,26 April,Almería to Águilas,178.0 km,Road stage",
            "5,27 April,Águilas to Murcia,176.0 km,Road stage",
            "6,28 April,Murcia to Benidorm,217.0 km,Road stage",
            "7,29 April,Benidorm,8.3 km,Individual time trial",
            "8,30 April,Benidorm to La Pobla de Farnals,217.0 km,Road stage",
            "9,1 May,La Pobla de Farnals to Vinaròs,157.0 km,Road stage",
            "10,2 May,Vinaròs to Cambrils,173.0 km,Road stage",
            "11,3 May,Cambrils to Barcelona,151.0 km,Road stage",
            "12,4 May,Palma de Mallorca,181.0 km,Road stage",
            "13,5 May,Barcelona to Tremp,189.0 km,Road stage",
            "14,6 May,Tremp to Formigal,233.0 km,Road stage",
            "15,7 May,Jaca to Irache,160.0 km,Road stage",
            "16,8 May,Irache to Urkiola,150.0 km,Road stage",
            "17,9 May,Durango to Bilbao,123.0 km,Road stage",
            "18,10 May,Bilbao to Miranda de Ebro,186.0 km,Road stage",
            "19a,11 May,Miranda de Ebro to Beasain,110.0 km,Road stage",
            "19b,11 May,San Sebastián,31.7 km,Individual time trial"
        ];
        let edition = tour_of_spain_1975();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1976() {
        let route = [
            "P,27 April,Estepona,3.2 km,Individual time trial",
            "1,28 April,Estepona,135.0 km,Road stage",
            "2,29 April,Estepona to Priego de Córdoba,224.0 km,Road stage",
            "3,30 April,Priego de Córdoba to Jaén,177.0 km,Road stage",
            "4,1 May,Jaén to Baza,166.0 km,Road stage",
            "5,2 May,Baza to Cartagena,201.0 km,Road stage",
            "6,3 May,Cartagena,14.0 km,Individual time trial",
            "7,4 May,Cartagena to Murcia,136.0 km,Road stage",
            "8,5 May,Murcia to Almansa,219.0 km,Road stage",
            "9,6 May,Almansa to Nules,208.0 km,Road stage",
            "10,7 May,Castellón to Cambrils,226.0 km,Road stage",
            "11,8 May,Cambrils to Barcelona,151.0 km,Road stage",
            "12,9 May,Pamplona to Logroño,168.0 km,Road stage",
            "13,10 May,Logroño to Palencia,209.0 km,Road stage",
            "14,11 May,Parades de Nava to Gijón,249.0 km,Road stage",
            "15,12 May,Gijón to Cangas de Onís,141.0 km,Road stage",
            "16,13 May,Cangas de Onís to Reinosa,156.0 km,Road stage",
            "17,14 May,Reinosa to Bilbao,183.0 km,Road stage",
            "18,15 May,Galdácano to Sanctuario de Oro,204.0 km,Road stage",
            "19,16 May,Murgia to San Sebastián,139.0 km,Road stage",
            "20a,17 May,Murgia to San Sebastián,139.0 km,Road stage",
            "20b,17 May,San Sebastián,31.7 km,Individual time trial"
        ];
        let edition = tour_of_spain_1976();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1977() {
        let route = [
            "P,26 April,Dehesa de Campoamor,8.0 km,Individual time trial",
            "1,27 April,Dehesa de Campoamor to La Manga,115.0 km,Road stage",
            "2,28 April,La Managa to Murcia,161.0 km,Road stage",
            "3,29 April,Murcia to Benidorm,200.0 km,Road stage",
            "4,30 April,Benidorm,8.3 km,Individual time trial",
            "5,1 May,Benidorm to El Saler,159.0 km,Road stage",
            "6,2 May,Valencia to Teruel,170.0 km,Road stage",
            "7,3 May,Teruel to Alcalá de Xivert,204.0 km,Road stage",
            "8,4 May,Alcalá de Xivert to Tortosa,141.0 km,Road stage",
            "9,5 May,Tortosa to Salou,144.0 km,Road stage",
            "10,6 May,Salou to Barcelona,144.0 km,Road stage",
            "11a,7 May,Barcelona,3.8 km,Individual time trial",
            "11b,7 May,Barcelona,45.0 km,Road stage",
            "12,8 May,Barcelona to La Tossa de Montbui,198.0 km,Road stage",
            "13,9 May,Igualada to La Seu d'Urgell,135.0 km,Road stage",
            "14,10 May,La Seu d'Urgell to Monzón,200.0 km,Road stage",
            "15,11 May,Monzón to Formigal,166.0 km,Road stage",
            "16,12 May,Formigal to Cordovilla,170.0 km,Road stage",
            "17,13 May,Cordovilla to Bilbao,183.0 km,Road stage",
            "18,14 May,Bilbao to Urkiola,126.0 km,Road stage",
            "19,15 May,Durango to Miranda de Ebro,104.0 km,Road stage"
        ];
        let edition = tour_of_spain_1977();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1978() {
        let route = [
            "P,25 April,Gijón,8.6 km,Individual time trial",
            "1,26 April,Gijón,144.0 km,Road stage",
            "2,27 April,Gijón to Cangas de Onís,94.0 km,Road stage",
            "3,28 April,Cangas de Onís to León,187.0 km,Road stage",
            "4,29 April,León to Valladolid,171.0 km,Road stage",
            "5,30 April,Valladolid to Ávila,136.0 km,Road stage",
            "6,1 May,Torrelaguna to Torrejón de Ardoz,46.0 km,Road stage",
            "7,2 May,Torrejón de Ardoz to Cuenca,160.0 km,Road stage",
            "8,3 May,Cuenca to Benicàssim,249.0 km,Road stage",
            "9,4 May,Benicàssim to Tortosa,156.0 km,Road stage",
            "10,5 May,Tortosa to Calafell,201.0 km,Road stage",
            "11a,6 May,Calafell to Barcelona,67.0 km,Road stage",
            "11b,6 May,Barcelona,3.8 km,Individual time trial",
            "12,7 May,Bellaterra (Cerdanyola del Vallès) to La Tossa de Montbui (Santa Margarida de Montbui),205.0 km,Road stage",
            "13,8 May,Igualada to Jaca,243.0 km,Road stage",
            "14,9 May,Jaca to Logroño,219.0 km,Road stage",
            "15,10 May,Logroño to Miranda de Ebro,131.0 km,Road stage",
            "16,11 May,Miranda de Ebro to Bien Aparecida Sanctuary,208.0 km,Road stage",
            "17,12 May,Ampuero to Bilbao,123.0 km,Road stage",
            "18,13 May,Bilbao to Amurrio,154.0 km,Road stage",
            "19a,14 May,Amurrio to San Sebastián,84.0 km,Road stage",
            "19b,14 May,San Sebastián,0.0 km,Individual time trial"
        ];
        let edition = tour_of_spain_1978();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1979() {
        let route = [
            "P,24 April,Jerez de la Frontera,6.3 km,Individual time trial",
            "1,25 April,Jerez de la Frontera to Seville,156.0 km,Road stage",
            "2,26 April,Seville to Córdoba,188.0 km,Road stage",
            "3,27 April,Córdoba to Sierra Nevada,190.0 km,Road stage",
            "4,28 April,Granada to Puerto Lumbreras,222.0 km,Road stage",
            "5,29 April,Puerto Lumbreras to Murcia,139.0 km,Road stage",
            "6,30 April,Murcia to Alcoy,171.0 km,Road stage",
            "7,1 May,Alcoy to Sedavi,173.0 km,Road stage",
            "8a,2 May,Sedavi to Benicàssim,145.0 km,Road stage",
            "8b,2 May,Benicàssim,11.3 km,Individual time trial",
            "9,3 May,Benicàssim to Reus,193.0 km,Road stage",
            "10,4 May,Reus to Zaragoza,230.0 km,Road stage",
            "11,5 May,Zaragoza to Pamplona,183.0 km,Road stage",
            "12,6 May,Pamplona to Logroño,149.0 km,Road stage",
            "13,7 May,Haro to Peña Cabarga,180.0 km,Road stage",
            "14,8 May,Torrelavega to Gijón,178.0 km,Road stage",
            "15,9 May,Gijón to León,156.0 km,Road stage",
            "16a,10 May,León to Valladolid,134.0 km,Road stage",
            "16b,10 May,Valladolid,22.0 km,Individual time trial",
            "17,11 May,Valladolid to Ávila,204.0 km,Road stage",
            "18a,12 May,Ávila to Colmenar Viejo,155.0 km,Road stage",
            "18b,12 May,Colmenar Viejo to Azuqueca de Henares,104.0 km,Road stage",
            "19,13 May,Madrid,84.0 km,Road stage"
        ];
        let edition = tour_of_spain_1979();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 3 split stages");
    }

    #[test]
    fn test_tour_of_spain_1980() {
        let route = [
            "P,22 April,La Manga,10.0 km,Individual time trial",
            "1,23 April,La Manga to Benidorm,155.0 km,Road stage",
            "2,24 April,Benidorm to Cuenca,170.0 km,Road stage",
            "3,25 April,Cullera to Vinaròs,207.0 km,Road stage",
            "4,26 April,Vinaròs to Sant Quirze del Vallès,214.0 km,Road stage",
            "5,27 April,Sant Quirze del Vallès to La Seu d'Urgell,200.0 km,Road stage",
            "6,28 April,La Seu d'Urgell to Viella,131.0 km,Road stage",
            "7,29 April,Viella to Jaca,216.0 km,Road stage",
            "8,30 April,Monastery of Leyre to Logroño,160.0 km,Road stage",
            "9,1 May,Logroño to Burgos,138.0 km,Road stage",
            "10,2 May,Burgos to Santander,178.0 km,Road stage",
            "11,3 May,Santander to Gijón,219.0 km,Road stage",
            "12,4 May,Santiago de Compostela to Pontevedra,133.0 km,Road stage",
            "13,5 May,Pontevedra to Vigo,195.0 km,Road stage",
            "14,6 May,Vigo to Ourense,156.0 km,Road stage",
            "15,7 May,Ourense to Ponferrada,164.0 km,Road stage",
            "16a,8 May,Ponferrada to León,131.0 km,Road stage",
            "16b,8 May,León,22.8 km,Individual time trial",
            "17,9 May,León to Valladolid,138.0 km,Road stage",
            "18,10 May,Valladolid to Los Ángeles de San Rafael,197.0 km,Road stage",
            "19,11 May,Madrid,84.0 km,Road stage"
        ];
        let edition = tour_of_spain_1980();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1981() {
        let route = [
            "P,21 April,Santander,6.3 km,Individual time trial",
            "1,22 April,Santander to Avilés,221.0 km,Road stage",
            "2,23 April,Avilés to León,159.0 km,Road stage",
            "3,24 April,León to Salamanca,195.0 km,Road stage",
            "4,25 April,Salamanca to Cáceres,206.0 km,Road stage",
            "5,26 April,Cáceres to Mérida,152.0 km,Road stage",
            "6,27 April,Mérida to Seville,199.0 km,Road stage",
            "7,28 April,Écija to Jaén,181.0 km,Road stage",
            "8a,29 April,Jaén to Granada,100.0 km,Road stage",
            "8b,29 April,Granada to Sierra Nevada,30.5 km,Individual time trial",
            "9,30 April,Baza to Murcia,204.0 km,Road stage",
            "10,1 May,Murcia to Almussafes,223.0 km,Road stage",
            "11,2 May,Almussafes to Peniscola,193.0 km,Road stage",
            "12,3 May,Peniscola to Esparreguera,217.0 km,Road stage",
            "13,4 May,Esparreguera to Rasos de Peguera,187.0 km,Road stage",
            "14,5 May,Gironella to Balaguer,197.0 km,Road stage",
            "15a,6 May,Balaguer to Alfajarín,146.0 km,Road stage",
            "15b,6 May,Zaragoza,11.3 km,Individual time trial",
            "16,7 May,Calatayud to Torrejón de Ardoz,209.0 km,Road stage",
            "17,8 May,Torrejón de Ardoz to Segovia,150.0 km,Road stage",
            "18,9 May,Segovia to Los Ángeles de San Rafael,175.0 km,Road stage",
            "19,10 May,Madrid,84.0 km,Road stage"
        ];
        let edition = tour_of_spain_1981();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1982() {
        let route = [
            "P,20 April,Santiago de Compostela,6.7 km,Individual time trial",
            "1a,21 April,Santiago de Compostela to A Coruña,97.0 km,Road stage",
            "1b,21 April,A Coruña to Lugo,97.0 km,Road stage",
            "2,22 April,Lugo to Gijón,240.0 km,Road stage",
            "3,23 April,Gijón to Santander,208.0 km,Road stage",
            "4,24 April,Santander to Reinosa,196.0 km,Road stage",
            "5,25 April,Reinosa to Logroño,230.0 km,Road stage",
            "6,26 April,Logroño to Zaragoza,190.0 km,Road stage",
            "7,27 April,Zaragoza to Sabiñánigo,146.0 km,Road stage",
            "8,28 April,Zaragoza to Lleida,216.0 km,Road stage",
            "9,29 April,Artesa de Segre to Puigcerdà,182.0 km,Road stage",
            "10,30 April,Puigcerdà to Sant Quirze del Vallès,181.0 km,Road stage",
            "11,1 May,Sant Quirze del Vallès to Barcelona,143.0 km,Road stage",
            "12,2 May,Salou to Nules,200.0 km,Road stage",
            "13,3 May,Nules to Antella,195.0 km,Road stage",
            "14,4 May,Antella to Albacete,153.0 km,Road stage",
            "15a,5 May,Albacete to Tomelloso,119.0 km,Road stage",
            "15b,5 May,Tomelloso to Campo de Criptana,35.0 km,Individual time trial",
            "16,6 May,Campo de Criptana to San Fernando de Henares,176.0 km,Road stage",
            "17,7 May,San Fernando de Henares to Puerto de Navacerrada,178.0 km,Road stage",
            "18,8 May,Palazuelos de Eresma (Destilerias DYC),184.0 km,Road stage",
            "19,9 May,Madrid,84.0 km,Road stage"
        ];

        let edition = tour_of_spain_1982();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 2 split stages");
    }

    #[test]
    fn test_tour_of_spain_1983() {
        let route = [
            "P,19 April,Almussafes,6.8 km,Individual time trial",
            "1,20 April,Almussafes to Cuenca,235.0 km,Road stage",
            "2,21 April,Cuenca to Teruel,152.0 km,Road stage",
            "3,22 April,Teruel to Sant Carles de la Ràpita,241.0 km,Road stage",
            "4,23 April,Sant Carles de la Ràpita to Sant Quirze del Vallès,192.0 km,Road stage",
            "5,24 April,Sant Quirze del Vallès to Castellar de n'Hug,195.0 km,Road stage",
            "6,25 April,La Pobla de Lillet to Viella,235.0 km,Road stage",
            "7,26 April,Les to Sabiñánigo,137.0 km,Road stage",
            "8,27 April,Sabiñánigo to Balneario de Panticosa,38.0 km,Individual time trial",
            "9,28 April,Panticosa to Alfajarín,183.0 km,Road stage",
            "10,29 April,Zaragoza to Soria,174.0 km,Road stage",
            "11,30 April,Soria to Logroño,185.0 km,Road stage",
            "12,1 May,Logroño to Burgos,147.0 km,Road stage",
            "13,2 May,Aguilar de Campoo to Lagos de Covadonga,188.0 km,Road stage",
            "14,3 May,Cangas de Onís to León,195.0 km,Road stage",
            "15a,4 May,León to Valladolid,134.0 km,Road stage",
            "15b,4 May,Valladolid,22.0 km,Individual time trial",
            "16,5 May,Valladolid to Salamanca,162.0 km,Road stage",
            "17,6 May,Salamanca to Ávila,216.0 km,Road stage",
            "18,7 May,Ávila to Palazuelos de Eresma (Destilerias DYC),204.0 km,Road stage",
            "19,8 May,Palazuelos de Eresma (Destilerias DYC) to Madrid,135.0 km,Road stage"
        ];
        let edition = tour_of_spain_1983();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1984() {
        let route = [
            "P,17 April,Jerez de la Frontera,6.6 km,Individual time trial",
            "1,18 April,Jerez de la Frontera to Málaga,272.0 km,Road stage",
            "2,19 April,Málaga to Almería,202.0 km,Road stage",
            "3,20 April,Mojácar to Elche,204.0 km,Road stage",
            "4,21 April,Elche to Valencia,197.0 km,Road stage",
            "5,22 April,Valencia to Salou,245.0 km,Road stage",
            "6,23 April,Salou to Sant Quirze del Vallès,113.0 km,Road stage",
            "7,24 April,Sant Quirze del Vallès to Rasos de Peguera,184.0 km,Road stage",
            "8,25 April,Cardona to Zaragoza,269.0 km,Road stage",
            "9,26 April,Zaragoza to Soria,159.0 km,Road stage",
            "10,27 April,Soria to Burgos,148.0 km,Road stage",
            "11,28 April,Burgos to Santander,182.0 km,Road stage",
            "12,29 April,Santander to Lagos de Covadonga,199.0 km,Road stage",
            "13,30 April,Cangas de Onís to Oviedo,170.0 km,Road stage",
            "14,1 May,Lugones to Monte Naranco,12.0 km,Individual time trial",
            "15,2 May,Oviedo to León,121.0 km,Road stage",
            "16,3 May,León to Valladolid,138.0 km,Road stage",
            "17,4 May,Valladolid to Segovia,258.0 km,Road stage",
            "18a,5 May,Segovia to Torrejón de Ardoz,145.0 km,Road stage",
            "18b,5 May,Torrejón de Ardoz,33.0 km,Individual time trial",
            "19,6 May,Torrejón de Ardoz to Madrid,139.0 km,Road stage"
        ];
        let edition = tour_of_spain_1984();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1985() {
        let route = [
            "P,23 April,Valladolid,5.6 km,Individual time trial",
            "1,24 April,Valladolid to Zamora,177.0 km,Road stage",
            "2,25 April,Zamora to Ourense,262.0 km,Road stage",
            "3,26 April,Ourense to Santiago de Compostela,197.0 km,Road stage",
            "4,27 April,Santiago de Compostela to Lugo,162.0 km,Road stage",
            "5,28 April,Lugo to Oviedo,238.0 km,Road stage",
            "6,29 April,Oviedo to Lagos de Covadonga,145.0 km,Road stage",
            "7,30 April,Cangas de Onís to Alto Campoo,190.0 km,Road stage",
            "8,1 May,Aguilar de Campoo to Logroño,224.0 km,Road stage",
            "9,2 May,Logroño to Balneario de Panticosa,253.0 km,Road stage",
            "10,3 May,Sabiñánigo to Tremp,209.0 km,Road stage",
            "11,4 May,Tremp to Andorra la Vella (Andorra),124.0 km,Road stage",
            "12,5 May,Andorra la Vella (Andorra) to Pal (Andorra),16.0 km,Individual time trial",
            "13,6 May,Andorra la Vella (Andorra) to Sant Quirze del Vallès,193.0 km,Road stage",
            "14,7 May,Valencia to Benidorm,201.0 km,Road stage",
            "15,8 May,Benidorm to Albacete,208.0 km,Road stage",
            "16,9 May,Albacete to Alcalá de Henares,252.0 km,Road stage",
            "17,10 May,Alcalá de Henares,43.0 km,Individual time trial",
            "18,11 May,Alcalá de Henares to Palazuelos de Eresma (Destilerias DYC),200.0 km,Road stage",
            "19,12 May,Palazuelos de Eresma (Destilerias DYC) to Salamanca,175.0 km,Road stage"
        ];
        let edition = tour_of_spain_1985();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "19 stages + Prologue");
    }

    #[test]
    fn test_tour_of_spain_1986() {
        let route = [
            "P,22 April,Palma de Mallorca,5.7 km,Individual time trial",
            "1,23 April,Palma de Mallorca,190.0 km,Road stage",
            "2,24 April,Barcelona,182.0 km,Road stage",
            "3,25 April,Lleida to Zaragoza,212.0 km,Road stage",
            "4,26 April,Zaragoza to Logroño,192.0 km,Road stage",
            "5,27 April,Haro to Santander,202.0 km,Road stage",
            "6,28 April,Santander to Lagos de Covadonga,191.0 km,Road stage",
            "7,29 April,Cangas de Onís to Oviedo,180.0 km,Road stage",
            "8,30 April,Oviedo to Alto del Naranco,9.7 km,Individual time trial",
            "9,1 May,Oviedo to San Isidro,180.0 km,Road stage",
            "10,2 May,San Isidro to Palencia,193.0 km,Road stage",
            "11,3 May,Valladolid,29.1 km,Individual time trial",
            "12,4 May,Valladolid to Segovia,258.0 km,Road stage",
            "13,5 May,Segovia to Villalba,148.0 km,Road stage",
            "14,6 May,Casino Gran Madrid to Leganés,165.0 km,Road stage",
            "15,7 May,Aranjuez to Albacete,207.0 km,Road stage",
            "16,8 May,Albacete to Jaén,264.0 km,Road stage",
            "17,9 May,Jaén to Sierra Nevada,172.0 km,Road stage",
            "18,10 May,Granada to Benalmádena,191.0 km,Road stage",
            "19,11 May,Benalmádena to Puerto Real,234.0 km,Road stage",
            "20,12 May,Puerto Real to Jerez de la Frontera,239.0 km,Road stage",
            "21,13 May,Jerez de la Frontera,22.0 km,Individual time trial"
        ];
        let edition = tour_of_spain_1986();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }

    #[test]
    fn test_tour_of_spain_1987() {
        let route = [
            "P,23 April,Benidorm,6.6 km,Individual time trial",
            "1,24 April,Benidorm to Albacete,219.0 km,Road stage",
            "2,25 April,Albacete to Valencia,217.0 km,Road stage",
            "3,26 April,Valencia,34.8 km,Individual time trial",
            "4,27 April,Valencia to Villareal,169.0 km,Road stage",
            "5,28 April,Salou to Barcelona,165.0 km,Road stage",
            "6,29 April,Barcelona to Grau Roig (Andorra),220.0 km,Road stage",
            "7,30 April,La Seu d'Urgell to Cerler,186.0 km,Road stage",
            "8,1 May,Benasque to Zaragoza,219.0 km,Road stage",
            "9,2 May,Zaragoza to Pamplona,180.0 km,Road stage",
            "10,3 May,Miranda de Ebro to Alto Campoo,213.0 km,Road stage",
            "11,4 May,Santander to Lagos de Covadonga,179.0 km,Road stage",
            "12,5 May,Cangas de Onís to Oviedo,142.0 km,Road stage",
            "13,6 May,Luarca to Ferrol,223.0 km,Road stage",
            "14,7 May,Ferrol to A Coruña,220.0 km,Road stage",
            "15,8 May,A Coruña to Vigo,185.0 km,Road stage",
            "16,9 May,Ponteareas to Ponferrada,237.0 km,Road stage",
            "17,10 May,Ponferrada to Valladolid,221.0 km,Road stage",
            "18,11 May,Valladolid,24.0 km,Individual time trial",
            "19,12 May,El Barco de Ávila to Ávila,213.0 km,Road stage",
            "20,13 May,Ávila to Palazuelos de Eresma (Destilerias DYC),183.0 km,Road stage",
            "21,14 May,Palazuelos de Eresma (Destilerias DYC) to Collado Villalba,160.0 km,Road stage",
            "22,15 May,Alcalá de Henares to Madrid,173.0 km,Road stage"
        ];
        let edition = tour_of_spain_1987();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue");
    }

    #[test]
    fn test_tour_of_spain_1988() {
        let route = [
            "1,25 April,Santa Cruz de Tenerife,17.4 km,Individual time trial",
            "2,26 April,San Cristóbal de la Laguna to Santa Cruz de Tenerife,210.0 km,Road stage",
            "3,27 April,Las Palmas,34.0 km,Team time trial",
            "4,28 April,Alcalá del Rio to Badajoz,210.0 km,Road stage",
            "5,29 April,Badajoz to Béjar,234.0 km,Road stage",
            "6,30 April,Béjar to Valladolid,202.0 km,Road stage",
            "7,1 May,Valladolid to León,160.0 km,Road stage",
            "8,2 May,León to Brañillín,176.7 km,Road stage",
            "9,3 May,Oviedo to Monte Naranco,6.8 km,Individual time trial",
            "10,4 May,Oviedo to Santander,197.3 km,Road stage",
            "11,5 May,Santander to Valdezcaray,217.2 km,Road stage",
            "12,6 May,Logroño to Jaca,197.5 km,Road stage",
            "13,7 May,Jaca to Cerler,178.2 km,Road stage",
            "14,8 May,Benasque to Andorra la Vella (Andorra),190.3 km,Road stage",
            "15,9 May,La Seu d'Urgell to Sant Quirze del Vallès,166.0 km,Road stage",
            "16,10 May,Valencia to Albacete,192.0 km,Road stage",
            "17,11 May,Albacete to Toledo,244.4 km,Road stage",
            "18,12 May,Toledo to Ávila,212.5 km,Road stage",
            "19,13 May,Ávila to Segovia,150.0 km,Road stage",
            "20,14 May,Las Rozas to Villalba,30.0 km,Individual time trial",
            "21,15 May,Villalba to Madrid,202.0 km,Road stage"
        ];
        let edition = tour_of_spain_1988();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_1989() {
        let route = [
            "1,24 April,A Coruña,21.0 km,Road stage",
            "2,25 April,A Coruña to Santiago de Compostela,222.0 km,Road stage",
            "3a,26 April,Vigo,35.0 km,Team time trial",
            "3b,26 April,Vigo to Ourense,105.0 km,Road stage",
            "4,27 April,Ourense to Pontevedra,163.0 km,Road stage",
            "5,28 April,La Baneza to Béjar,260.0 km,Road stage",
            "6,29 April,Béjar to Ávila,195.0 km,Road stage",
            "7,30 April,Ávila to Toledo,165.0 km,Road stage",
            "8,1 May,Toledo to Albacete,226.0 km,Road stage",
            "9,2 May,Albacete to Gandia,194.0 km,Road stage",
            "10,3 May,Gandia to Benicàssim,219.0 km,Road stage",
            "11,4 May,Vinaròs to Lleida,182.0 km,Road stage",
            "12,5 May,Lleida to Cerler,190.0 km,Road stage",
            "13,6 May,Benasque to Jaca,190.0 km,Road stage",
            "14,7 May,Jaca to Zaragoza,166.0 km,Road stage",
            "15,8 May,Ezcaray to Valdezcaray,23.0 km,Individual time trial",
            "16,9 May,Haro to Santoña,193.0 km,Road stage",
            "17,10 May,Santoña to Lagos de Enol,225.0 km,Road stage",
            "18,11 May,Cangas de Onís to Brañillín,152.0 km,Road stage",
            "19,12 May,León to Valladolid,157.0 km,Road stage",
            "20,13 May,Valladolid to Medina del Campo,42.0 km,Individual time trial",
            "21,14 May,Collado Villalba to Palazuelos de Eresma (Destilerias DYC),187.0 km,Road stage",
            "22,15 May,Palazuelos de Eresma (Destilerias DYC) to Madrid,179.0 km,Road stage"
        ];

        let edition = tour_of_spain_1989();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1990() {
        let route = [
            "1,24 April,Benicàssim,10.3 km,Individual time trial",
            "2a,25 April,Oropesa to Castellón,108.0 km,Road stage",
            "2b,25 April,Benicàssim to Borriana,36.3 km,Team time trial",
            "3,26 April,Dénia to Murcia,196.3 km,Road stage",
            "4,27 April,Murcia to Almería,226.2 km,Road stage",
            "5,28 April,Almería to Sierra Nevada,198.0 km,Road stage",
            "6,29 April,Loja to Ubrique,195.2 km,Road stage",
            "7,30 April,Jerez to Seville,190.3 km,Road stage",
            "8,1 May,Seville to Mérida,187.6 km,Road stage",
            "9,2 May,Cáceres to Guijuelo,192.7 km,Road stage",
            "10,3 May,Peñaranda de Bracamonte to León,226.8 km,Road stage",
            "11,4 May,León to San Isidro,203.0 km,Road stage",
            "12,5 May,San Isidro to Naranco,156.0 km,Road stage",
            "13,6 May,Oviedo to Santander,193.3 km,Road stage",
            "14,7 May,Santander to Nájera,207.5 km,Road stage",
            "15,8 May,Ezcaray to Valdezcaray,24.1 km,Individual time trial",
            "16,9 May,Logroño to Pamplona,165.5 km,Road stage",
            "17,10 May,Pamplona to Jaca,155.3 km,Road stage",
            "18,11 May,Jaca to Cerler,178.5 km,Road stage",
            "19,12 May,Benasque to Zaragoza,223.6 km,Road stage",
            "20,13 May,Zaragoza,40.0 km,Individual time trial",
            "21,14 May,Collado Villalba to Palazuelos de Eresma,188.6 km,Road stage",
            "22,15 May,Segovia to Madrid,177.0 km,Road stage"
        ];
        let edition = tour_of_spain_1990();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1991() {
        let route = [
            "1,29 April,Mérida,8.8 km,Three man team time trial",
            "2a,30 April,Mérida to Cáceres,134.5 km,Road stage",
            "2b,30 April,Montigo to Badajoz,40.4 km,Team time trial",
            "3,1 May,Badajoz to Seville,233.2 km,Road stage",
            "4,2 May,Seville to Jaén,292.0 km,Road stage",
            "5,3 May,Linares to Albacete,227.8 km,Road stage",
            "6,4 May,Albacete to Valencia,236.5 km,Road stage",
            "7,5 May,Palma de Mallorca,188.0 km,Road stage",
            "8,6 May,Cala d'Or,47.0 km,Individual time trial",
            "9,7 May,Sant Cugat del Vallès to Lloret de Mar,140.0 km,Road stage",
            "10,8 May,Lloret de Mar to Andorra la Vella (Andorra),229.0 km,Road stage",
            "11,9 May,Andorra la Vella (Andorra) to Pla de Beret,0.0 km,Road stage",
            "12,10 May,Bossost to Cerler,111.0 km,Road stage",
            "13,11 May,Benasque to Zaragoza,219.0 km,Road stage",
            "14,12 May,Ezcaray to Valdezcaray,24.1 km,Individual time trial",
            "15,13 May,Santo Domingo de la Calzada to Santander,219.5 km,Road stage",
            "16,14 May,Santander to Lagos de Covadonga,186.6 km,Road stage",
            "17,15 May,Cangas de Onís to Alto del Naranco,152.0 km,Road stage",
            "18,16 May,León to Valladolid,137.5 km,Road stage",
            "19,17 May,Valladolid,53.2 km,Individual time trial",
            "20,18 May,Palazuelos de Eresma (Destilerias DYC),212.7 km,Road stage",
            "21,19 May,Collado Villalba to Madrid,169.6 km,Road stage"
        ];
        let edition = tour_of_spain_1991();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1992() {
        let route = [
            "1,27 April,Jerez de la Frontera,9.2 km,Individual time trial",
            "2a,28 April,San Fernando to Jerez de la Frontera,135.5 km,Road stage",
            "2b,28 April,Arcos de la Frontera to Jerez de la Frontera,32.6 km,Team time trial",
            "3,29 April,Jerez de la Frontera to Córdoba,205.0 km,Road stage",
            "4,30 April,Linares to Albacete,229.0 km,Road stage",
            "5,1 May,Albacete to Gandia,213.5 km,Road stage",
            "6,2 May,Gandia to Benicàssim,202.8 km,Road stage",
            "7,3 May,Alquerias del Nino Perdido to Oropesa,49.5 km,Individual time trial",
            "8,4 May,Lleida to Pla de Beret,240.5 km,Road stage",
            "9,5 May,Vielha to Luz Ardiden (France),144.0 km,Road stage",
            "10,6 May,Luz-Saint-Sauveur (France) to Sabiñánigo,196.0 km,Road stage",
            "11,7 May,Sabiñánigo to Pamplona,162.9 km,Road stage",
            "12,8 May,Pamplona to Burgos,200.1 km,Road stage",
            "13,9 May,Burgos to Santander,178.3 km,Road stage",
            "14,10 May,Santander to Lagos de Covadonga,213.4 km,Road stage",
            "15,11 May,Cangas de Onís to Alto del Naranco,163.0 km,Road stage",
            "16,12 May,Oviedo to León,162.0 km,Road stage",
            "17,13 May,León to Salamanca,200.6 km,Road stage",
            "18,14 May,Salamanca to Ávila,218.9 km,Road stage",
            "19,15 May,Fuenlabrada,37.9 km,Individual time trial",
            "20,16 May,Collado Villalba to Palazuelos de Eresma (Destilerias DYC),188.3 km,Road stage",
            "21,17 May,Palazuelos de Eresma (Destilerias DYC) to Madrid,175.0 km,Road stage"
        ];
        let edition = tour_of_spain_1992();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 1 split stage");
    }

    #[test]
    fn test_tour_of_spain_1993() {
        let route = [
            "1,26 April,A Coruña,10.0 km,Individual time trial",
            "2,27 April,A Coruña to Vigo,251.1 km,Road stage",
            "3,28 April,Vigo to Ourense,171.4 km,Road stage",
            "4,29 April,A Gudina to Salamanca,233.4 km,Road stage",
            "5,30 April,Salamanca to Ávila,219.8 km,Road stage",
            "6,1 May,Palazuelos de Eresma (Destilerias DYC) to Puerto de Navacerrada,24.1 km,Individual time trial",
            "7,2 May,Palazuelos de Eresma (Destilerias DYC) to Madrid,184.0 km,Road stage",
            "8,3 May,Aranjuez to Albacete,225.1 km,Road stage",
            "9,4 May,Albacete to Valencia,224.0 km,Road stage",
            "10,5 May,Valencia to La Senia,206.0 km,Road stage",
            "11,6 May,Lleida to Cerler,221.0 km,Road stage",
            "12,7 May,Benasque to Zaragoza,220.7 km,Road stage",
            "13,8 May,Zaragoza,37.1 km,Individual time trial",
            "14,9 May,Tudela to Alto de la Cruz de la Demanda (Ezcaray),197.2 km,Road stage",
            "15,10 May,Santo Domingo de la Calzada to Santander,226.2 km,Road stage",
            "16,11 May,Santander to Alto Campoo,160.0 km,Road stage",
            "17,12 May,Santander to Lagos de Covadonga,179.5 km,Road stage",
            "18,13 May,Cangas de Onís to Gijón,170.0 km,Road stage",
            "19,14 May,Gijón to Alto del Naranco,153.0 km,Road stage",
            "20,15 May,Salas to Ferol,247.0 km,Road stage",
            "21,16 May,Padron to Santiago de Compostela,44.6 km,Individual time trial"
        ];
        let edition = tour_of_spain_1993();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_1994() {
        let route = [
            "1,25 April,Valladolid,9.0 km,Individual time trial",
            "2,26 April,Valladolid to Salamanca,178.4 km,Road stage",
            "3,27 April,Salamanca to Cáceres,239.0 km,Road stage",
            "4,28 April,Almendralejo to Córdoba,235.6 km,Road stage",
            "5,29 April,Córdoba to Granada,166.9 km,Road stage",
            "6,30 April,Granada to Sierra Nevada,151.7 km,Road stage",
            "7,1 May,Baza to Alicante,256.5 km,Road stage",
            "8,2 May,Benidorm,39.5 km,Individual time trial",
            "9,3 May,Benidorm to Valencia,166.0 km,Road stage",
            "10,4 May,Igualada to Andorra-Arcalis (Andorra),205.0 km,Road stage",
            "11,5 May,Andorra la Vella (Andorra) to Cerler,195.3 km,Road stage",
            "12,6 May,Benasque to Zaragoza,226.7 km,Road stage",
            "13,7 May,Zaragoza to Pamplona,201.6 km,Road stage",
            "14,8 May,Pamplona to Sierra de la Demanda,174.0 km,Road stage",
            "15,9 May,Santo Domingo de la Calzada to Santander,209.3 km,Road stage",
            "16,10 May,Santander to Lagos de Covadonga,147.7 km,Road stage",
            "17,11 May,Cangas de Onís to Monte Naranco,150.4 km,Road stage",
            "18,12 May,Ávila,189.0 km,Road stage",
            "19,13 May,Ávila to Palazuelos de Eresma,171.0 km,Road stage",
            "20,14 May,Segovia to Palazuelos de Eresma,53.0 km,Individual time trial",
            "21,15 May,Palazuelos de Eresma to Madrid,165.7 km,Road stage"
        ];
        let edition = tour_of_spain_1994();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_1995() {
        let route = [
            "P,2 September,Zaragoza,7.0 km,Individual time trial",
            "1,3 September,Zaragoza to Logroño,186.6 km,Road stage",
            "2,4 September,San Asensio to Santander,223.5 km,Road stage",
            "3,5 September,Santander to Alto del Naranco,206.0 km,Road stage",
            "4,6 September,Tapia de Casariego to A Coruña,82.6 km,Road stage",
            "5,7 September,A Coruña to Ourense,179.8 km,Road stage",
            "6,8 September,Ourense to Zamora,264.0 km,Road stage",
            "7,9 September,Salamanca,41.0 km,Individual time trial",
            "8,10 September,Salamanca to Ávila,219.8 km,Road stage",
            "9,11 September,Ávila to Palazuelos de Eresma,122.5 km,Road stage",
            "10,12 September,Córdoba to Seville,208.5 km,Road stage",
            "11,13 September,Seville to Marbella,162.5 km,Road stage",
            "12,14 September,Marbella to Sierra Nevada,238.5 km,Road stage",
            "13,15 September,Olula del Rio to Murcia,181.0 km,Road stage",
            "14,16 September,Elche to Valencia,207.0 km,Road stage",
            "15,17 September,Barcelona to Estadi Olimpic Lluis Companys,154.0 km,Road stage",
            ",18 September,Rest day",
            "16,19 September,Tàrrega to Pla de Beret,197.3 km,Road stage",
            "17,20 September,Salardu (Naut Aran) to Luz Ardiden (France),179.2 km,Road stage",
            "18,21 September,Luz-Saint-Sauveur (France) to Sabiñánigo,157.8 km,Road stage",
            "19,22 September,Sabiñánigo to Calatayud,227.7 km,Road stage",
            "20,23 September,Alcalá de Henares,41.6 km,Individual time trial",
            "21,24 September,Alcalá de Henares to Madrid,147.5 km,Road stage"
        ];
        let edition = tour_of_spain_1995();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }

    #[test]
    fn test_tour_of_spain_1996() {
        let route = [
            "1,7 September,Valencia,162.0 km,Road stage",
            "2,8 September,Valencia to Cuenca,210.0 km,Road stage",
            "3,9 September,Cuenca to Albacete,167.2 km,Road stage",
            "4,10 September,Albacete to Murcia,166.5 km,Road stage",
            "5,11 September,Murcia to Almería,208.4 km,Road stage",
            "6,12 September,Almería to Málaga,196.5 km,Road stage",
            "7,13 September,Málaga to Marbella,171.1 km,Road stage",
            "8,14 September,Marbella to Jerez de la Frontera,220.7 km,Road stage",
            "9,15 September,Jerez de la Frontera to Córdoba,203.5 km,Road stage",
            ",16 September,Rest day",
            "10,17 September,El Tiemblo to Ávila,46.5 km,Individual time trial",
            "11,18 September,Ávila to Salamanca,188.0 km,Road stage",
            "12,19 September,Benavente to Alto del Naranco,188.0 km,Road stage",
            "13,20 September,Oviedo to Lagos de Covadonga,159.0 km,Road stage",
            "14,21 September,Cangas de Onís to Cabarceno Natural Park,202.6 km,Road stage",
            "15,22 September,Cabarceno to Alto de la Cruz de la Demanda (Ezcaray),220.0 km,Road stage",
            "16,23 September,Logroño to Sabiñánigo,220.9 km,Road stage",
            "17,24 September,Sabiñánigo to Cerler,165.7 km,Road stage",
            "18,25 September,Benasque to Zaragoza,219.5 km,Road stage",
            "19,26 September,Getafe to Ávila,217.1 km,Road stage",
            "20,27 September,Ávila to Palazuelos de Eresma (Destilerias DYC),209.5 km,Road stage",
            "21,28 September,Segovia to Palazuelos de Eresma (Destilerias DYC),43.0 km,Individual time trial",
            "22,29 September,Madrid,157.6 km,Road stage"
        ];
        let edition = tour_of_spain_1996();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }

    #[test]
    fn test_tour_of_spain_1997() {
        let route = [
            "1,6 September,Lisbon (Portugal) to Estoril (Portugal),155.0 km,Road stage",
            "2,7 September,Evora (Portugal) to Vilamoura (Portugal),225.0 km,Road stage",
            "3,8 September,Loule (Portugal) to Huelva,173.0 km,Road stage",
            "4,9 September,Huelva to Jerez de la Frontera,192.0 km,Road stage",
            "5,10 September,Jerez de la Frontera to Málaga,230.0 km,Road stage",
            "6,11 September,Málaga to Granada,147.0 km,Road stage",
            "7,12 September,Guadix to Sierra Nevada,219.0 km,Road stage",
            "8,13 September,Granada to Córdoba,176.0 km,Road stage",
            "9,14 September,Córdoba,35.0 km,Individual time trial",
            "10,15 September,Córdoba to Almendralejo,224.0 km,Road stage",
            "11,16 September,Almendralejo to Plasencia,194.0 km,Road stage",
            "12,17 September,León to Alto del Morredero,147.0 km,Road stage",
            "13,18 September,Ponferrada to Estacion Valgrande-Pajares,196.0 km,Road stage",
            "14,19 September,Oviedo to Alto del Naranco,169.0 km,Road stage",
            "15,20 September,Oviedo to Lagos de Covadonga,160.0 km,Road stage",
            "16,21 September,Cangas de Onís to Santander,170.0 km,Road stage",
            "17,22 September,Santander to Burgos,183.0 km,Road stage",
            "18,23 September,Burgos to Valladolid,184.0 km,Road stage",
            "19,24 September,Valladolid to Los Ángeles de San Rafael,193.0 km,Road stage",
            "20,25 September,Los Ángeles de San Rafael to Ávila,199.0 km,Road stage",
            "21,26 September,Alcobendas,43.0 km,Individual time trial",
            "22,27 September,Madrid,154.0 km,Road stage"
        ];
        let edition = tour_of_spain_1997();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }

    #[test]
    fn test_tour_of_spain_1998() {
        let route = [
            "1,5 September,Córdoba,161.7 km,Road stage",
            "2,6 September,Córdoba to Cádiz,234.6 km,Road stage",
            "3,7 September,Cádiz to Estepona,192.6 km,Road stage",
            "4,8 September,Málaga to Granada,173.5 km,Road stage",
            "5,9 September,Olula del Rio to Murcia,165.5 km,Road stage",
            "6,10 September,Murcia to Xorret de Catí,201.5 km,Road stage",
            "7,11 September,Alicante to Valencia,185.0 km,Road stage",
            "8,12 September,Palma de Mallorca,181.5 km,Road stage",
            "9,13 September,Alcudia,39.5 km,Individual time trial",
            ",14 September,Rest day",
            "10,15 September,Vic to Estacio de Pal (Andorra),199.3 km,Road stage",
            "11,16 September,Andorra la Vella (Andorra) to Cerler,186.0 km,Road stage",
            "12,17 September,Benasque to Jaca (Canfranc International Station),187.0 km,Road stage",
            "13,18 September,Sabiñánigo,208.5 km,Road stage",
            "14,19 September,Biescas to Zaragoza,145.5 km,Road stage",
            "15,20 September,Zaragoza to Soria,178.7 km,Road stage",
            "16,21 September,Soria to Laguna Negra de Neila,143.7 km,Road stage",
            "17,22 September,Burgos to León,188.5 km,Road stage",
            "18,23 September,León to Salamanca,223.0 km,Road stage",
            "19,24 September,Ávila to Segovia,170.4 km,Road stage",
            "20,25 September,Segovia to Puerto de Navacerrada,206.0 km,Road stage",
            "21,26 September,Fuenlabrada,39.0 km,Individual time trial",
            "22,27 September,Madrid,163.0 km,Road stage"
        ];
        let edition = tour_of_spain_1998();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }

    #[test]
    fn test_tour_of_spain_1999() {
        let route = [
            "P,4 September,Murcia,6.1 km,Individual time trial",
            "1,5 September,Murcia to Benidorm,179.0 km,Road stage",
            "2,6 September,Alicante to Albacete,206.0 km,Road stage",
            "3,7 September,La Roda to Fuenlabrada,229.5 km,Road stage",
            "4,8 September,La Rozas to Salamanca,185.6 km,Road stage",
            "5,9 September,Béjar to Ciudad Rodrigo,160.0 km,Road stage",
            "6,10 September,Salamanca,46.4 km,Individual time trial",
            "7,11 September,Salamanca to León,217.0 km,Road stage",
            "8,12 September,León to Alto de l'Angliru,175.6 km,Road stage",
            "9,13 September,Gijón to Los Corrales de Buelna,185.8 km,Road stage",
            ",14 September,Rest day",
            "10,15 September,Zaragoza,183.2 km,Road stage",
            "11,16 September,Huesca to Val d'Aran/Pla-de-Beret (France),201.0 km,Road stage",
            "12,17 September,Sort to Andorra-Arcalis (Andorra),147.4 km,Road stage",
            "13,18 September,Andorra la Vella (Andorra) to Castellar del Riu (Rasos de Peguera),149.0 km,Road stage",
            "14,19 September,Barcelona,94.4 km,Road stage",
            "15,20 September,La Senia to Valencia,193.4 km,Road stage",
            "16,21 September,Valencia to Teruel,200.4 km,Road stage",
            "17,22 September,Bronchales to Guadalajara,225.0 km,Road stage",
            "18,23 September,Guadalajara to Alto de Abantos,166.3 km,Road stage",
            "19,24 September,San Lorenzo de El Escorial to Ávila,184.6 km,Road stage",
            "20,25 September,El Tiemblo to Ávila,46.5 km,Individual time trial",
            "21,26 September,Madrid,163.0 km,Road stage"
        ];
        let edition = tour_of_spain_1999();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }

    #[test]
    fn test_tour_of_spain_2000() {
        let route = [
            "1,26 August,Málaga,13.3 km,Individual time trial",
            "2,27 August,Málaga to Córdoba,167.5 km,Road stage",
            "3,28 August,Montoro to Valdepeñas,198.4 km,Road stage",
            "4,29 August,Valdepeñas to Albacete,159.0 km,Road stage",
            "5,30 August,Albacete to Xorret de Catí,152.3 km,Road stage",
            "6,31 August,Benidorm to Valencia,155.5 km,Road stage",
            "7,1 September,Valencia to Morella,175.0 km,Road stage",
            "8,2 September,Vinaròs to Port Aventura,168.5 km,Road stage",
            "9,3 September,Tarragona,37.6 km,Individual time trial",
            "10,4 September,Sabadell to Supermolina,165.8 km,Road stage",
            "11,5 September,Alp to Andorra-Arcalis (Andorra),136.5 km,Road stage",
            ",6 September,Rest day",
            "12,7 September,Zaragoza,131.5 km,Road stage",
            ",8 September,Rest day",
            "13,9 September,Santander,143.3 km,Road stage",
            "14,10 September,Santander to Lagos de Covadonga,146.5 km,Road stage",
            "15,11 September,Cangas de Onís to Gijón,164.2 km,Road stage",
            "16,12 September,Oviedo to Alto de l'Angliru,168.0 km,Road stage",
            "17,13 September,Benavente to Salamanca,155.5 km,Road stage",
            "18,14 September,Béjar to Ciudad Rodrigo,159.0 km,Road stage",
            "19,15 September,Salamanca to Ávila,130.0 km,Road stage",
            "20,16 September,Ávila to Alto de Abantos,128.2 km,Road stage",
            "21,17 September,Madrid,38.0 km,Individual time trial"
        ];
        let edition = tour_of_spain_2000();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2001() {
        let route = [
            "1,8 September,Salamanca,12.0 km,Individual time trial",
            "2,9 September,Salamanca to Valladolid,147.2 km,Road stage",
            "3,10 September,Valladolid to León,140.5 km,Road stage",
            "4,11 September,León to Gijón,175.0 km,Road stage",
            "5,12 September,Gijón to Lagos de Covadonga,160.8 km,Road stage",
            "6,13 September,Cangas de Onís to Torrelavega,180.6 km,Road stage",
            "7,14 September,Torrelavega,44.2 km,Individual time trial",
            "8,15 September,Reinosa to Alto de la Cruz de la Demanda (Valdezcaray),195.0 km,Road stage",
            "9,16 September,Logroño to Barcelona,179.2 km,Road stage",
            ",17 September,Rest day",
            "10,18 September,Sabadell to La Molina,168.4 km,Road stage",
            "11,19 September,Alp to Estacio de Pal (Andorra),154.2 km,Road stage",
            "12,20 September,Ordino (Andorra) to Estacio d'Esqui d'Ordino-Alcalis (Andorra),17.1 km,Individual time trial",
            "13,21 September,Andorra la Vella (Andorra) to Universal Studios Port Aventura,206.0 km,Road stage",
            "14,22 September,Tarragona to Vinaròs,170.5 km,Road stage",
            "15,23 September,Valencia to Alto de Aitana,207.2 km,Road stage",
            ",24 September,Rest day",
            "16,25 September,Alcoy to Murcia,153.3 km,Road stage",
            "17,26 September,Murcia to Albacete,159.5 km,Road stage",
            "18,27 September,Albacete to Cuenca,154.2 km,Road stage",
            "19,28 September,Cuenca to Guadalajara,168.0 km,Road stage",
            "20,29 September,Guadalajara to Alto de Abantos,176.3 km,Road stage",
            "21,30 September,Madrid,38.0 km,Individual time trial"
        ];
        let edition = tour_of_spain_2001();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2002() {
        let route = [
            "1,7 September,Valencia,24.6 km,Team time trial",
            "2,8 September,Valencia to Alcoy,144.7 km,Road stage",
            "3,9 September,San Vicente del Raspeig to Murcia,134.2 km,Road stage",
            "4,10 September,Águilas to Roquetas de Mar,149.5 km,Road stage",
            "5,11 September,El Ejido to Sierra Nevada,198.0 km,Road stage",
            "6,12 September,Granada to Sierra de la Pandera,153.1 km,Road stage",
            "7,13 September,Jaén to Málaga,196.8 km,Road stage",
            "8,14 September,Málaga to Ubrique,173.6 km,Road stage",
            "9,15 September,Córdoba,130.2 km,Road stage",
            "10,16 September,Córdoba,36.5 km,Individual time trial",
            ",17 September,Rest day",
            "11,18 September,Alcobendas to Collado Villalba,166.1 km,Road stage",
            "12,19 September,Segovia to Burgos,210.5 km,Road stage",
            "13,20 September,Burgos to Santander,189.8 km,Road stage",
            "14,21 September,Santander to Gijón,190.2 km,Road stage",
            "15,22 September,Gijón to Alto de l'Angliru,176.7 km,Road stage",
            ",23 September,Rest day",
            "16,24 September,Avilés to León,154.7 km,Road stage",
            "17,25 September,Benavente to Salamanca,146.6 km,Road stage",
            "18,26 September,Salamanca to La Covatilla,193.7 km,Road stage",
            "19,27 September,Béjar to Ávila,177.8 km,Road stage",
            "20,28 September,Ávila to Parque Warner Madrid,141.2 km,Road stage",
            "21,29 September,Parque Warner Madrid to Santiago Bernabeu Stadium (Madrid),41.2 km,Individual time trial"
        ];
        let edition = tour_of_spain_2002();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2003() {
        let route = [
            "1,6 September,Gijón,28.0 km,Team time trial",
            "2,7 September,Gijón to Cangas de Onís,148.0 km,Road stage",
            "3,8 September,Cangas de Onís to Santander,154.3 km,Road stage",
            "4,9 September,Santander to Burgos,151.0 km,Road stage",
            "5,10 September,Soria to Zaragoza,166.7 km,Road stage",
            "6,11 September,Zaragoza,43.8 km,Individual time trial",
            "7,12 September,Huesca to Cauterets (France),190.0 km,Road stage",
            "8,13 September,Cauterets (France) to Pla de Beret/Val d'Aran,166.0 km,Road stage",
            "9,14 September,Vielha to Envalira (Andorra),174.8 km,Road stage",
            "10,15 September,Andorra la Vella (Andorra) to Sabadell,194.0 km,Road stage",
            ",16 September,Rest day",
            "11,17 September,Utiel to Cuenca,162.0 km,Road stage",
            "12,18 September,Cuenca to Albacete,168.8 km,Road stage",
            "13,19 September,Albacete,53.3 km,Individual time trial",
            "14,20 September,Albacete to Valdepeñas,167.4 km,Road stage",
            "15,21 September,Valdepeñas to La Pandera,172.1 km,Road stage",
            ",22 September,Rest day",
            "16,23 September,Jaén to Sierra Nevada,162.0 km,Road stage",
            "17,24 September,Granada to Córdoba,188.4 km,Road stage",
            "18,25 September,Las Rozas,143.8 km,Road stage",
            "19,26 September,Alcobendas to Collado Villalba,164.0 km,Road stage",
            "20,27 September,San Lorenzo de El Escorial to Alto de Abantos,11.2 km,Individual time trial",
            "21,28 September,Madrid,148.5 km,Road stage"
        ];
        let edition = tour_of_spain_2003();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2004() {
        let route = [
            "1,4 September,León,28.0 km,Team time trial",
            "2,5 September,León to Burgos,207.0 km,Road stage",
            "3,6 September,Burgos to Soria,156.0 km,Road stage",
            "4,7 September,Soria to Zaragoza,167.0 km,Road stage",
            "5,8 September,Zaragoza to Morella,186.5 km,Road stage",
            "6,9 September,Benicarlo to Castellón de la Plana,157.0 km,Road stage",
            "7,10 September,Castellón de la Plana to Valencia,170.0 km,Road stage",
            "8,11 September,Almussafes,40.1 km,Individual time trial",
            "9,12 September,Xativa to Alto de Aitana,162.0 km,Road stage",
            "10,13 September,Alcoy to Xorret de Catí,174.2 km,Road stage",
            "11,14 September,San Vicente del Raspeig to Caravaca de la Cruz,165.0 km,Road stage",
            ",15 September,Rest day",
            "12,16 September,Almería to Calar Alto Observatory,145.0 km,Road stage",
            "13,17 September,El Ejido to Málaga,172.0 km,Road stage",
            "14,18 September,Málaga to Granada,167.0 km,Road stage",
            "15,19 September,Granada to Sierra Nevada,29.6 km,Individual time trial",
            ",20 September,Rest day",
            "16,21 September,Olivenza to Cáceres,190.1 km,Road stage",
            "17,22 September,Plasencia to La Covatilla,170.0 km,Road stage",
            "18,23 September,Béjar to Ávila,196.0 km,Road stage",
            "19,24 September,Ávila to Collado Villalba,142.0 km,Road stage",
            "20,25 September,Alcobendas to Puerto de Navacerrada,178.0 km,Road stage",
            "21,26 September,Madrid,28.0 km,Individual time trial"
        ];
        let edition = tour_of_spain_2004();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2005() {
        let route = [
            "1,27 August,Granada,7.0 km,Individual time trial",
            "2,28 August,Granada to Córdoba,189.3 km,Road stage",
            "3,29 August,Córdoba to Puertollano,153.3 km,Road stage",
            "4,30 August,Ciudad Real to Argamasilla de Alba,232.3 km,Road stage",
            "5,31 August,Alcázar de San Juan to Cuenca,176.0 km,Road stage",
            "6,1 September,Cuenca to Valdelinares,217.0 km,Road stage",
            "7,2 September,Teruel to Vinaròs,212.5 km,Road stage",
            "8,3 September,Tarragona to Lloret de Mar,189.0 km,Road stage",
            "9,4 September,Lloret de Mar,48.0 km,Individual time trial",
            "10,5 September,La Vall d'en Bas to Estacio d'Esqui d'Ordino-Alcalis (Andorra),206.3 km,Road stage",
            "11,6 September,Andorra la Vella (Andorra) to Cerler,186.6 km,Road stage",
            ",7 September,Rest day",
            "12,8 September,Logroño to Burgos,133.0 km,Road stage",
            "13,9 September,Burgos to Santuario de la Bien Aparecida (Ampuero),196.0 km,Road stage",
            "14,10 September,La Penilla to Lagos de Covadonga,172.3 km,Road stage",
            "15,11 September,Cangas de Onís to Valgrande-Pajares,191.0 km,Road stage",
            ",12 September,Rest day",
            "16,13 September,León to Valladolid,162.5 km,Road stage",
            "17,14 September,El Espinar to La Granja de San Ildefonso,165.6 km,Road stage",
            "18,15 September,Ávila,197.5 km,Road stage",
            "19,16 September,San Martin de Valdeiglesias to Alcobendas,142.9 km,Road stage",
            "20,17 September,Guadalajara to Alcalá de Henares,38.9 km,Individual time trial",
            "21,18 September,Madrid,136.5 km,Road stage"
        ];
        let edition = tour_of_spain_2005();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2006() {
        let route = [
            "1,26 August,Málaga,7.2 km,Team time trial",
            "2,27 August,Málaga to Córdoba,167.0 km,Road stage",
            "3,28 August,Córdoba to Almendralejo,220.0 km,Road stage",
            "4,29 August,Almendralejo to Cáceres,142.0 km,Road stage",
            "5,30 August,Plasencia to La Covatilla (Estacion de Esqui),178.0 km,Road stage",
            "6,31 August,Zamora to León,155.0 km,Road stage",
            "7,1 September,León to Alto del Morredero (Ponferrada),148.0 km,Road stage",
            "8,2 September,Ponferrada to Lugo,173.0 km,Road stage",
            "9,3 September,A Fonsagrada to Alto de la Cobertoria,206.0 km,Road stage",
            ",4 September,Rest day",
            "10,5 September,Avilés to Santillana del Mar (Museo de Altamira),190.0 km,Road stage",
            "11,6 September,Torrelavega to Burgos,165.0 km,Road stage",
            "12,7 September,Aranda de Duero to Guadalajara,162.0 km,Road stage",
            "13,8 September,Guadalajara to Cuenca,170.0 km,Road stage",
            "14,9 September,Cuenca,33.0 km,Individual time trial",
            "15,10 September,Motilla del Palancar to Factoria Ford (Almussafes),175.0 km,Road stage",
            ",11 September,Rest day",
            "16,12 September,Almería to Calar Alto Observatory,145.0 km,Road stage",
            "17,13 September,Adra to Granada,167.0 km,Road stage",
            "18,14 September,Granada to Sierra de la Pandera,153.0 km,Road stage",
            "19,15 September,Jaén to Ciudad Real,195.0 km,Road stage",
            "20,16 September,Rivas Futura to Rivas Vaciamadrid,28.0 km,Individual time trial",
            "21,17 September,Madrid,150.0 km,Road stage"
        ];
        let edition = tour_of_spain_2006();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2007() {
        let route = [
            "1,1 September,Vigo,146.4 km,Road stage",
            "2,2 September,Allariz to Santiago de Compostela,148.7 km,Road stage",
            "3,3 September,Viveiro to Luarca,153.0 km,Road stage",
            "4,4 September,Langreo to Lagos de Covadonga,185.1 km,Road stage",
            "5,5 September,Cangas de Onís to Reinosa,157.4 km,Road stage",
            "6,6 September,Reinosa to Logroño,184.3 km,Road stage",
            "7,7 September,Calahora to Zaragoza,176.3 km,Road stage",
            "8,8 September,Carinena to Zaragoza,52.2 km,Individual time trial",
            "9,9 September,Huesca to Cerler,167.6 km,Road stage",
            "10,10 September,Benasque to Andorra-Arcalis (Andorra),214.0 km,Road stage",
            ",11 September,Rest day",
            "11,12 September,Oropesa del Mar to Algemesi,191.3 km,Road stage",
            "12,13 September,Algemesi to Hellín,176.0 km,Road stage",
            "13,14 September,Hellín to Torre-Pacheco,176.4 km,Road stage",
            "14,15 September,Puerto Lumbreras to Villacarrilo,207.0 km,Road stage",
            "15,16 September,Villacarrilo to Granada,201.4 km,Road stage",
            ",17 September,Rest day",
            "16,18 September,Jaén to Puertollano,161.5 km,Road stage",
            "17,19 September,Ciudad Real to Talavera de la Reina,175.0 km,Road stage",
            "18,20 September,Talavera de la Reina to Ávila,153.5 km,Road stage",
            "19,21 September,Ávila to Alto de Abantos,133.0 km,Road stage",
            "20,22 September,Collado Villalba,20.0 km,Individual time trial",
            "21,23 September,Rivas Vaciamadrid to Madrid,104.2 km,Road stage"
        ];
        let edition = tour_of_spain_2007();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2008() {
        let route = [
            "1,30 August,Granada,7.7 km,Individual time trial",
            "2,31 August,Granada to Jaén,167.3 km,Road stage",
            "3,1 September,Jaén to Córdoba,168.6 km,Road stage",
            "4,2 September,Córdoba to Puertollano,170.3 km,Road stage",
            "5,3 September,Ciudad Real,42.5 km,Individual time trial",
            "6,4 September,Ciudad Real to Toledo,150.1 km,Road stage",
            ",5 September,Rest day",
            "7,6 September,Barbastro to Naturlandia (Andorra),223.2 km,Road stage",
            "8,7 September,Escaldes-Engordany (Andorra) to Pla de Beret,151.0 km,Road stage",
            "9,8 September,Vielha e Mijaran to Sabiñánigo,200.8 km,Road stage",
            "10,9 September,Sabiñánigo to Zaragoza,151.3 km,Road stage",
            "11,10 September,Calahorra to Burgos,178.0 km,Road stage",
            "12,11 September,Burgos to Suances,186.4 km,Road stage",
            ",12 September,Rest day",
            "13,13 September,San Vicente de la Barquera to Alto de l'Angliru,209.5 km,Road stage",
            "14,14 September,Oviedo to Fuentes de Invierno,158.4 km,Road stage",
            "15,15 September,Cudillero to Ponferrada,202.0 km,Road stage",
            "16,16 September,Ponferrada to Zamora,186.3 km,Road stage",
            "17,17 September,Zamora to Valladolid,148.2 km,Road stage",
            "18,18 September,Valladolid to Las Rozas,167.4 km,Road stage",
            "19,19 September,Las Rozas to Segovia,145.5 km,Road stage",
            "20,20 September,La Granja de San Ildefonso to Puerto de Navacerrada,17.1 km,Individual time trial",
            "21,21 September,San Sebastián de los Reyes to Madrid,102.2 km,Road stage"
        ];
        let edition = tour_of_spain_2008();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2009() {
        let route = [
            "1,29 August,Assen (Netherlands),4.8 km,Individual time trial",
            "2,30 August,Assen (Netherlands) to Emmen (Netherlands),203.7 km,Road stage",
            "3,31 August,Zutphen (Netherlands) to Venlo (Netherlands),189.7 km,Road stage",
            "4,1 September,Venlo (Netherlands) to Liège (Belgium),225.5 km,Road stage",
            ",2 September,Rest day",
            "5,3 September,Tarragona to Vinaròs,174.0 km,Road stage",
            "6,4 September,Xativa,176.8 km,Road stage",
            "7,5 September,Valencia,30.0 km,Individual time trial",
            "8,6 September,Alzira to Alto de Aitana,204.7 km,Road stage",
            "9,7 September,Alcoy to Xorret de Catí,188.8 km,Road stage",
            "10,8 September,Alicante to Murcia,171.2 km,Road stage",
            "11,9 September,Murcia to Caravaca de la Cruz,200.0 km,Road stage",
            ",10 September,Rest day",
            "12,11 September,Almería to Alto de Velefique,179.3 km,Road stage",
            "13,12 September,Berja to Sierra Nevada,172.4 km,Road stage",
            "14,13 September,Granada to Sierra de la Pandera,157.0 km,Road stage",
            "15,14 September,Jaén to Córdoba,167.7 km,Road stage",
            "16,15 September,Córdoba to Puertollano,170.3 km,Road stage",
            "17,16 September,Ciudad Real to Talavera de la Reina,193.6 km,Road stage",
            "18,17 September,Talavera de la Reina to Ávila,165.0 km,Road stage",
            "19,18 September,Ávila to La Granja de San Ildefonso,179.8 km,Road stage",
            "20,19 September,Toledo,27.8 km,Individual time trial",
            "21,20 September,Rivas Vaciamadrid to Madrid,110.2 km,Road stage"
        ];
        let edition = tour_of_spain_2009();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2010() {
        let route = [
            "1,28 August,Seville,13.0 km,Team time trial",
            "2,29 August,Alcalá de Guadaira to Marbella,173.0 km,Road stage",
            "3,30 August,Marbella to Málaga,156.0 km,Road stage",
            "4,31 August,Málaga to Valdepeñas de Jaén,177.0 km,Road stage",
            "5,1 September,Guadix to Lorca,194.0 km,Road stage",
            "6,2 September,Caravaca de la Cruz to Murcia,144.0 km,Road stage",
            "7,3 September,Murcia to Orihuela,170.0 km,Road stage",
            "8,4 September,Villena to Xorret de Catí,188.8 km,Road stage",
            "9,5 September,Calp to Alcoi,187.0 km,Road stage",
            ",6 September,Rest day",
            "10,7 September,Tarragona to Vilanova i la Geltru,173.7 km,Road stage",
            "11,8 September,Vilanova i la Geltru to Pal (Andorra),208.0 km,Road stage",
            "12,9 September,Andorra la Vella (Andorra) to Lleida,175.0 km,Road stage",
            "13,10 September,Rincon de Soto to Burgos,193.7 km,Road stage",
            "14,11 September,Burgos to Peña Cabarga,178.8 km,Road stage",
            "15,12 September,Solares to Lagos de Covadonga,170.0 km,Road stage",
            "16,13 September,Gijón to Alto de Cotobello,179.3 km,Road stage",
            ",14 September,Rest day",
            "17,15 September,Peñafiel,46.0 km,Individual time trial",
            "18,16 September,Valladolid to Salamanca,153.0 km,Road stage",
            "19,17 September,Piedrahita to Toledo,200.0 km,Road stage",
            "20,18 September,San Martin de Valdeiglesias to Bola del Mundo,168.8 km,Road stage",
            "21,19 September,San Sebastián de los Reyes to Madrid,85.0 km,Road stage"
        ];
        let edition = tour_of_spain_2010();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2011() {
        let route = [
            "1,20 August,Benidorm,13.5 km,Team time trial",
            "2,21 August,La Nucia to Playas de Orihuela,175.5 km,Road stage",
            "3,22 August,Petrer to Totana,163.0 km,Road stage",
            "4,23 August,Baza to Sierra Nevada,170.2 km,Road stage",
            "5,24 August,Sierra Nevada to Valdepeñas de Jaén,187.0 km,Road stage",
            "6,25 August,Ubeda to Córdoba,196.8 km,Road stage",
            "7,26 August,Almadén to Talavera de la Reina,187.6 km,Road stage",
            "8,27 August,Talavera de la Reina to San Lorenzo de El Escorial,177.3 km,Road stage",
            "9,28 August,Villacastin to La Covatilla,183.0 km,Road stage",
            "10,29 August,Salamanca,47.0 km,Individual time trial",
            ",30 August,Rest day",
            "11,31 August,Verín to Estacion de Esqui Manzaneda,167.0 km,Road stage",
            "12,1 September,Ponteareas to Pontevedra,167.3 km,Road stage",
            "13,2 September,Sarria to Ponferrada,158.2 km,Road stage",
            "14,3 September,Astorga to La Farrapona (Lagos de Somiedo),172.8 km,Road stage",
            "15,4 September,Avilés to Alto de l'Angliru,142.2 km,Road stage",
            ",5 September,Rest day",
            "16,6 September,Villa Romana la Olmeda (Palancia) to Haro,188.1 km,Road stage",
            "17,7 September,Faustino V (Oyon) to Peña Cabarga,211.0 km,Road stage",
            "18,8 September,Solares to Noja,174.6 km,Road stage",
            "19,9 September,Noja to Bilbao,158.5 km,Road stage",
            "20,10 September,Bilbao to Vitoria-Gasteiz,185.0 km,Road stage",
            "21,11 September,Circuito del Jarama to Madrid,94.2 km,Road stage"
        ];
        let edition = tour_of_spain_2011();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2012() {
        let route = [
            "1,18 August,Pamplona,16.5 km,Team time trial",
            "2,19 August,Pamplona to Viana,181.4 km,Road stage",
            "3,20 August,Oion to Arrate (Eibar),155.3 km,Road stage",
            "4,21 August,Barakaldo to Valdezcaray,160.6 km,Road stage",
            "5,22 August,Logroño,168.0 km,Road stage",
            "6,23 August,Tarazona to El Fuerte del Rapitan (Jaca),175.4 km,Road stage",
            "7,24 August,Huesca to Motorland Aragon (Alcaniz),164.2 km,Road stage",
            "8,25 August,Lleida to Coll de la Gallina (Andorra),174.7 km,Road stage",
            "9,26 August,Andorra la Vella (Andorra) to Barcelona,196.3 km,Road stage",
            ",27 August,Rest day",
            "10,28 August,Ponteareas to Sanxenxo,190.0 km,Road stage",
            "11,29 August,Cambados to Pontevedra,39.4 km,Individual time trial",
            "12,30 August,Vilagarcia to Mirador de Ézaro,190.5 km,Road stage",
            "13,31 August,Santiago de Compostela to Ferrol,172.8 km,Road stage",
            "14,1 September,Palas de Rei to Los Ancares,149.2 km,Road stage",
            "15,2 September,La Robla to Lagos de Covadonga,186.5 km,Road stage",
            "16,3 September,Gijón to Cuitu Negro,183.5 km,Road stage",
            ",4 September,Rest day",
            "17,5 September,Santander to Fuente De,187.3 km,Road stage",
            "18,6 September,Aguilar de Campoo to Valladolid,204.5 km,Road stage",
            "19,7 September,Peñafiel to La Lastrilla,178.4 km,Road stage",
            "20,8 September,Palazuelos de Eresma to Bola del Mundo,170.7 km,Road stage",
            "21,9 September,Cercedilla to Madrid,115.0 km,Road stage"
        ];
        let edition = tour_of_spain_2012();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2013() {
        let route = [
            "1,24 August,Vilanova de Arousa to Sanxenxo,27.4 km,Team time trial",
            "2,25 August,Pontevedra to Monte da Groba,177.7 km,Road stage",
            "3,26 August,Vigo to Mirador de Lobeira,184.8 km,Road stage",
            "4,27 August,Lalin to Finisterra,189.0 km,Road stage",
            "5,28 August,Sober to Lago de Sanabria,174.3 km,Road stage",
            "6,29 August,Guijuelo to Cáceres,175.0 km,Road stage",
            "7,30 August,Almendralejo to Mairena del Aljarafe,205.9 km,Road stage",
            "8,31 August,Jerez de la Frontera to Alto de Peñas Blancas,166.6 km,Road stage",
            "9,1 September,Antequera to Valdepeñas de Jaén,163.7 km,Road stage",
            "10,2 September,Torredelcampo to Alto de Haza Llana,186.8 km,Road stage",
            ",3 September,Rest day",
            "11,4 September,Tarrazona,38.8 km,Individual time trial",
            "12,5 September,Maella to Tarragona,164.2 km,Road stage",
            "13,6 September,Valls to Castelldefels,169.0 km,Road stage",
            "14,7 September,Baga to Coll de la Gallina (Andorra),155.7 km,Road stage",
            "15,8 September,Andorra la Vella (Andorra) to Peyragudes (France),224.9 km,Road stage",
            "16,9 September,Graus to Formigal,146.8 km,Road stage",
            ",10 September,Rest day",
            "17,11 September,Calahorra to Burgos,189.0 km,Road stage",
            "18,12 September,Burgos to Peña Cabarga,186.5 km,Road stage",
            "19,13 September,San Vicente de la Barquera to Alto del Naranco,181.0 km,Road stage",
            "20,14 September,Avilés to Alto de l'Angliru,142.2 km,Road stage",
            "21,15 September,Leganés to Madrid,109.6 km,Road stage"
        ];
        let edition = tour_of_spain_2013();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2014() {
        let route = [
            "1,23 August,Jerez de la Frontera,12.6 km,Team time trial",
            "2,24 August,Algerciras to San Fernando,174.4 km,Road stage",
            "3,25 August,Cádiz to Arcos de la Frontera,197.8 km,Road stage",
            "4,26 August,Mairena del Alcor to Córdoba,164.7 km,Road stage",
            "5,27 August,Priego de Córdoba to Ronda,180.0 km,Road stage",
            "6,28 August,Benalmádena to Cumbres Verdes (La Zubia),167.1 km,Road stage",
            "7,29 August,Alhendin to Alcaudete,169.0 km,Road stage",
            "8,30 August,Baeza to Albacete,207.0 km,Road stage",
            "9,31 August,Carboneras de Guadazaon to Aramon Valdelinares,185.0 km,Road stage",
            ",1 September,Rest day",
            "10,2 September,Monasterio de Santa Maria de Veruela to Borja,36.7 km,Individual time trial",
            "11,3 September,Pamplona to Santuario de San Miguel de Aralar,153.4 km,Road stage",
            "12,4 September,Logroño,166.4 km,Road stage",
            "13,5 September,Belorado to Parque de Cabarceno (Obregon),188.7 km,Road stage",
            "14,6 September,Santander to La Camperona (Valle de Sabero),200.8 km,Road stage",
            "15,7 September,Oviedo to Lagos de Covadonga,152.2 km,Road stage",
            "16,8 September,San Martin del Rey Aurelio to La Farrapona (Lagos de Somiedo),160.5 km,Road stage",
            ",9 September,Rest day",
            "17,10 September,Ortigueira to A Coruña,190.7 km,Road stage",
            "18,11 September,A Estrada to Mont Castrove (Meis),157.0 km,Road stage",
            "19,12 September,Salvaterra de Mino to Cangas do Morrazo,180.5 km,Road stage",
            "20,13 September,Santo Estevo de Ribas de Sil to Puerto de Ancares,185.7 km,Road stage",
            "21,14 September,Santiago de Compostela,9.7 km,Individual time trial"
        ];
        let edition = tour_of_spain_2014();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2015() {
        let route = [
            "1,22 August,Puerto Banus to Marbella,7.4 km,Team time trial",
            "2,23 August,Alhaurín de la Torre to Caminito del Ray,158.7 km,Road stage",
            "3,24 August,Mijas to Málaga,158.4 km,Road stage",
            "4,25 August,Estepona to Vejer de la Frontera,209.6 km,Road stage",
            "5,26 August,Rota to Alcalá de Guadaira,167.3 km,Road stage",
            "6,27 August,Córdoba to Sierra de Cazorla,200.3 km,Road stage",
            "7,28 August,Jodar to La Alpujarra,191.1 km,Road stage",
            "8,29 August,Puebla de Don Fadrique to Murcia,182.5 km,Road stage",
            "9,30 August,Torrevieja to Cumbre del Sol (Benitachell),168.3 km,Road stage",
            "10,31 August,Valencia to Castellón de la Plana,146.6 km,Road stage",
            ",1 September,Rest day,Andorra la Vella (Andorra)",
            "11,2 September,Andorra la Vella (Andorra) to Cortals d'Encamp,138.0 km,Road stage",
            "12,3 September,Escaldes-Engordany (Andorra) to Lleida,173.0 km,Road stage",
            "13,4 September,Calatayud to Tarazona,178.0 km,Road stage",
            "14,5 September,Vitoria-Gasteiz to Alto Campoo,215.0 km,Road stage",
            "15,6 September,Comillas to Sotres,175.8 km,Road stage",
            "16,7 September,Luarca to Ermita del Alba (Quiros),185.0 km,Road stage",
            ",8 September,Rest day,Burgos",
            "17,9 September,Burgos,38.7 km,Individual time trial",
            "18,10 September,Roa de Duero to Riaza,204.0 km,Road stage",
            "19,11 September,Medina del Campo to Ávila,185.8 km,Road stage",
            "20,12 September,San Lorenzo de El Escorial to Cercedilla,175.8 km,Road stage",
            "21,13 September,Alcalá de Henares to Madrid,98.8 km,Road stage"
        ];
        let edition = tour_of_spain_2015();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2016() {
        let route = [
            "1,20 August,Laias to Parque Náutico Castrelo de Miño,7.4 km,Team time trial",
            "2,21 August,Ourense to Baiona,160.8 km,Road stage",
            "3,22 August,Marin to Mirador de Ézaro (Dumbria),176.4 km,Road stage",
            "4,23 August,Betanzos to San Andrés de Teixido,163.5 km,Road stage",
            "5,24 August,Viveiro to Lugo,171.3 km,Road stage",
            "6,25 August,Monforte de Lemos to Ribeira Sacra,163.2 km,Road stage",
            "7,26 August,Maceda to Puebla de Sanabria,158.5 km,Road stage",
            "8,27 August,Villalpando to La Camperona (Valle de Sabero),181.5 km,Road stage",
            "9,28 August,Cistierna to Alto del Naranco,164.5 km,Road stage",
            "10,29 August,Lugones to Lagos de Covadonga,188.7 km,Road stage",
            ",30 August,Rest day,Oviedo",
            "11,31 August,Jurassic Museum of Asturias (Colunga) to Peña Cabarga,168.6 km,Road stage",
            "12,1 September,Los Corrales de Buelna to Bilbao,193.2 km,Road stage",
            "13,2 September,Bilbao to Urdax-Dantxarinea,213.4 km,Road stage",
            "14,3 September,Urdax-Dantxarinea to Col d'Aubisque (France),196.0 km,Road stage",
            "15,4 September,Sabiñánigo to Aramon Formigal (Sallent de Gallego),118.5 km,Road stage",
            "16,5 September,Alcaniz to Peniscola,156.4 km,Road stage",
            ",6 September,Rest day,Castellón de la Plana",
            "17,7 September,Castellón de la Plana to Camins del Penyagolosa (Llucena),177.5 km,Road stage",
            "18,8 September,Requena to Gandia,200.6 km,Road stage",
            "19,9 September,Xabia to Calp,37.0 km,Individual time trial",
            "20,10 September,Benidorm to Alto de Aitana,193.2 km,Road stage",
            "21,11 September,Las Rozas to Madrid,104.8 km,Road stage"
        ];
        let edition = tour_of_spain_2016();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2017() {
        let route = [
            "1,19 August,Nîmes (France),13.7 km,Team time trial",
            "2,20 August,Nîmes (France) to Gruissan (France),203.4 km,Road stage",
            "3,21 August,Prades (France) to Andorra la Vella (Andorra),158.5 km,Road stage",
            "4,22 August,Escaldes-Engordany (Andorra) to Tarragona,198.2 km,Road stage",
            "5,23 August,Benicàssim to Alcossebre,175.7 km,Road stage",
            "6,24 August,Villareal to Sagunto,204.4 km,Road stage",
            "7,25 August,Llíria to Cuenca,207.0 km,Road stage",
            "8,26 August,Hellín to Xorret de Catí,199.5 km,Road stage",
            "9,27 August,Orihuela to Benitachell,174.0 km,Road stage",
            ",28 August,Rest day,Alicante",
            "10,29 August,Caravaca de la Cruz to El Pozo,164.8 km,Road stage",
            "11,30 August,Lorca to Calar Alto Observatory,187.5 km,Road stage",
            "12,31 August,Motril to Antequera,160.1 km,Road stage",
            "13,1 September,Coín to Tomares,198.4 km,Road stage",
            "14,2 September,Écija to Sierra de la Pandera,175.0 km,Road stage",
            "15,3 September,Alcalá la Real to Alto de la Hoya,129.4 km,Road stage",
            ",4 September,Rest day,Logroño",
            "16,5 September,Circuito de Navarra to Logroño,40.2 km,Individual time trial",
            "17,6 September,Villadiego to Alto de Los Machucos,180.5 km,Road stage",
            "18,7 September,Suances to Santo Toribo de Liébana,169.0 km,Road stage",
            "19,8 September,Caso to Gijón,149.7 km,Road stage",
            "20,9 September,Corvera de Asturias to Alto de l'Angliru,117.5 km,Road stage",
            "21,10 September,Arroyomolinos to Madrid,117.7 km,Road stage"
        ];
        let edition = tour_of_spain_2017();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_of_spain_2018() {
        let route = [
            "1,25 August,Málaga,8.0 km,Individual time trial",
            "2,26 August,Marbella to Caminito del Ray,163.9 km,Road stage",
            "3,27 August,Mijas to Alhaurín de la Torre,182.5 km,Road stage",
            "4,28 August,Vélez-Málaga to Alfacar,162.0 km,Road stage",
            "5,29 August,Granada to Roquetas de Mar,188.0 km,Road stage",
            "6,30 August,Huércal-Overa to San Javier,153.0 km,Road stage",
            "7,31 August,Puerto Lumbreras to Pozo Alcón,182.0 km,Road stage",
            "8,1 September,Linares to Almadén,195.5 km,Road stage",
            "9,2 September,Talavera de la Reina to La Covatilla,195.0 km,Road stage",
            ",3 September,Rest day,Salamanca",
            "10,4 September,Salamanca to Fermosella,172.5 km,Road stage",
            "11,5 September,Mombuey to Ribeira Sacra,208.8 km,Road stage",
            "12,6 September,Mondoñedo to Punta de Estaca de Bars,177.5 km,Road stage",
            "13,7 September,Candás to La Camperona,175.5 km,Road stage",
            "14,8 September,Cistierna to Les Praeres de Nava,167.0 km,Road stage",
            "15,9 September,Ribera de Arriba to Lagos de Covadonga,185.5 km,Road stage",
            ",10 September,Rest day,Santander",
            "16,11 September,Santillana del Mar to Torrelavega,32.7 km,Individual time trial",
            "17,12 September,Getxo to Oiz,166.4 km,Road stage",
            "18,13 September,Ejea de los Caballeros to Lleida,180.5 km,Road stage",
            "19,14 September,Lleida to Naturlandia (Andorra),157.0 km,Road stage",
            "20,15 September,Escaldes-Engordany (Andorra) to Coll de la Gallina (Andorra),105.8 km,Road stage",
            "21,16 September,Alcorcón to Madrid,112.3 km,Road stage"
        ];
        let edition = tour_of_spain_2018();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
}
