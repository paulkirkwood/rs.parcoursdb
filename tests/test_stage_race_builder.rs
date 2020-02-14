extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::belgium::*;
    use parcoursdb::france::*;
    use parcoursdb::west_germany::*;
    use parcoursdb::stage_race::StageRaceBuilder;
    use chrono::{TimeZone,Utc};

    #[test]
    fn test_name() {

        let dauphine_libere = StageRaceBuilder::dauphine(Utc.ymd(2009,6,7))
            .out_and_back_itt(&nancy(), 12.1)
            .road_stage(&nancy(), &dijon(), 228.0)
            .road_stage(&tournus(), &saint_etienne(), 182.0)
            .itt(&bourg_les_valence(), &valence(), 42.4)
            .road_stage(&valence(), &mont_ventoux(), 154.0)
            .road_stage(&gap(), &briancon(), 106.0)
            .road_stage(&briancon(), &saint_francois_longchamp(), 157.0)
            .road_stage(&faverges(), &grenoble(), 146.0)
            .build();

        let dauphine_libere_route = [
            "1,7 June,Nancy,12.1 km,Individual time trial",
            "2,8 June,Nancy to Dijon,228.0 km,Road stage",
            "3,9 June,Tournus to Saint-Étienne,182.0 km,Road stage",
            "4,10 June,Bourg-lès-Valence to Valence,42.4 km,Individual time trial",
            "5,11 June,Valence to Mont Ventoux,154.0 km,Road stage",
            "6,12 June,Gap to Briançon,106.0 km,Road stage",
            "7,13 June,Briançon to Saint-François-Longchamp,157.0 km,Road stage",
            "8,14 June,Faverges to Grenoble,146.0 km,Road stage",
        ];

        assert_eq!(dauphine_libere.name(), String::from("Critérium du Dauphiné Libéré"));
        assert_eq!(dauphine_libere.route(), dauphine_libere_route);
        assert_eq!(dauphine_libere.distance(), 1027.5);
        assert_eq!(dauphine_libere.has_prologue(), false);
        assert_eq!(dauphine_libere.number_of_road_stages(), 6);
        assert_eq!(dauphine_libere.number_of_individual_time_trials(), 2);
        assert_eq!(dauphine_libere.number_of_team_time_trials(), 0);
        assert_eq!(dauphine_libere.number_of_rest_days(), 0);

        let dauphine = StageRaceBuilder::dauphine(Utc.ymd(2018,6,3))
            .out_and_back_prologue(&valence(), 6.6)
            .road_stage(&valence(), &saint_just_saint_rambert() ,179.0)
            .road_stage(&montbrison(), &belleville(), 181.0)
            .ttt(&pont_de_vaux(), &louhans_chateaurenaud(), 35.0)
            .road_stage(&chazey_sur_ain(), &lans_en_vercors(), 181.0)
            .road_stage(&grenoble(), &valmorel(), 130.0)
            .road_stage(&frontenex(), &la_rosiere(), 110.0)
            .road_stage(&moutiers(), &saint_gervais_mont_blanc(), 136.0)
            .build();

        let dauphine_route = [
            "P,3 June,Valence,6.6 km,Individual time trial",
            "1,4 June,Valence to Saint-Just-Saint-Rambert,179.0 km,Road stage",
            "2,5 June,Montbrison to Belleville,181.0 km,Road stage",
            "3,6 June,Pont-de-Vaux to Louhans-Chateaurenaud,35.0 km,Team time trial",
            "4,7 June,Chazey-sur-Ain to Lans-en-Vercors,181.0 km,Road stage",
            "5,8 June,Grenoble to Valmorel,130.0 km,Road stage",
            "6,9 June,Frontenex to La Rosiere,110.0 km,Road stage",
            "7,10 June,Moûtiers to Saint-Gervais Mont-Blanc,136.0 km,Road stage"
        ];

        assert_eq!(dauphine.name(), String::from("Critérium du Dauphiné"));
        assert_eq!(dauphine.route(), dauphine_route);
        assert_eq!(dauphine.distance(), 958.6);
        assert_eq!(dauphine.has_prologue(), true);
        assert_eq!(dauphine.number_of_road_stages(), 6);
        assert_eq!(dauphine.number_of_individual_time_trials(), 0);
        assert_eq!(dauphine.number_of_team_time_trials(), 1);
        assert_eq!(dauphine.number_of_rest_days(), 0);
    }

    //#[test]
    //fn test_route() {

    //    let paris_nice = StageRaceBuilder::paris_nice(Utc.ymd(2018,3,4))
    //       .road_stage(String::from("Chatou"), String::from("Meudon"), 134.5)
    //       .road_stage(String::from("Orsonville"), String::from("Vierzon"), 187.5)
    //       .road_stage(String::from("Bourges"), String::from("Chatel-Guyon"), 210.0)
    //       .itt(String::from("La Fouillouse"), String::from("Saint-Étienne"), 18.4)
    //       .road_stage(String::from("Salon-de-Provence"), String::from("Sisteron"), 165.0)
    //       .road_stage(String::from("Sisteron"), String::from("Vence"), 198.0)
    //       .road_stage(String::from("Nice"), String::from("Valdeblore La Colmiane"), 175.0)
    //       .criterium(String::from("Nice"), 110.0)
    //       .build();

    //    let route = [
    //        "1,4 March,Chatou to Meudon,134.5 km,Road stage",
    //        "2,5 March,Orsonville to Vierzon,187.5 km,Road stage",
    //        "3,6 March,Bourges to Chatel-Guyon,210.0 km,Road stage",
    //        "4,7 March,La Fouillouse to Saint-Étienne,18.4 km,Individual time trial",
    //        "5,8 March,Salon-de-Provence to Sisteron,165.0 km,Road stage",
    //        "6,9 March,Sisteron to Vence,198.0 km,Road stage",
    //        "7,10 March,Nice to Valdeblore La Colmiane,175.0 km,Road stage",
    //        "8,11 March,Nice,110.0 km,Road stage"
    //    ];

    //    assert_eq!(paris_nice.route(), route);
    //}

    #[test]
    fn test_split_stages() {

        let tdf_1970 = StageRaceBuilder::tdf(Utc.ymd(1970,6,26))
            .out_and_back_prologue(&limoges(), 7.4)
            .road_stage(&limoges(), &la_rochelle(), 224.0)
            .road_stage(&la_rochelle(), &angers(), 200.0)
            .enable_split_stages()
            .out_and_back_ttt(&angers(), 10.7)
            .road_stage(&angers(), &rennes(), 140.0)
            .disable_split_stages()
            .road_stage(&rennes(), &lisieux(), 229.0)
            .enable_split_stages()
            .road_stage(&lisieux(), &rouen(), 94.5)
            .road_stage(&rouen(), &amiens(), 223.0)
            .disable_split_stages()
            .road_stage(&amiens(), &valenciennes(), 135.5)
            .enable_split_stages()
            .road_stage(&valenciennes(), &forest(), 120.0)
            .out_and_back_itt(&forest(), 7.2)
            .disable_split_stages()
            .road_stage(&ciney(), &felsberg(), 232.5)
            .road_stage(&saarlouis(), &mulhouse(), 269.5)
            .road_stage(&belfort(), &divonne_les_bains(), 241.0)
            .enable_split_stages()
            .out_and_back_itt(&divonne_les_bains(), 8.8)
            .road_stage(&divonne_les_bains(), &thonon_les_bains(), 139.5)
            .disable_split_stages()
            .road_stage(&divonne_les_bains(), &grenoble(), 194.0)
            .road_stage(&grenoble(), &gap(), 194.5)
            .road_stage(&gap(), &mont_ventoux(), 170.0)
            .road_stage(&carpentras(), &montpellier(), 140.5)
            .road_stage(&montpellier(), &toulouse(), 160.0)
            .road_stage(&toulouse(), &saint_gaudens(), 190.0)
            .road_stage(&saint_gaudens(), &la_mongie(), 135.5)
            .road_stage(&bagneres_de_bigorre(), &mourenx(), 185.5)
            .enable_split_stages()
            .road_stage(&mourenx(), &bordeaux(), 223.5)
            .out_and_back_itt(&bordeaux(), 8.2)
            .disable_split_stages()
            .road_stage(&ruffex(), &tours(), 191.5)
            .road_stage(&tours(), &versailles(), 238.5)
            .itt(&versailles(), &paris(), 54.0)
            .build();

        let route = [
            "P,26 June,Limoges,7.4 km,Individual time trial",
            "1,27 June,Limoges to La Rochelle,224.0 km,Road stage",
            "2,28 June,La Rochelle to Angers,200.0 km,Road stage",
            "3a,29 June,Angers,10.7 km,Team time trial",
            "3b,29 June,Angers to Rennes,140.0 km,Road stage",
            "4,30 June,Rennes to Lisieux,229.0 km,Road stage",
            "5a,1 July,Lisieux to Rouen,94.5 km,Road stage",
            "5b,1 July,Rouen to Amiens,223.0 km,Road stage",
            "6,2 July,Amiens to Valenciennes,135.5 km,Road stage",
            "7a,3 July,Valenciennes to Forest (Belgium),120.0 km,Road stage",
            "7b,3 July,Forest (Belgium),7.2 km,Individual time trial",
            "8,4 July,Ciney to Felsberg (West Germany),232.5 km,Road stage",
            "9,5 July,Saarlouis (West Germany) to Mulhouse,269.5 km,Road stage",
            "10,6 July,Belfort to Divonne-les-Bains,241.0 km,Road stage",
            "11a,7 July,Divonne-les-Bains,8.8 km,Individual time trial",
            "11b,7 July,Divonne-les-Bains to Thonon-les-Bains,139.5 km,Road stage",
            "12,8 July,Divonne-les-Bains to Grenoble,194.0 km,Road stage",
            "13,9 July,Grenoble to Gap,194.5 km,Road stage",
            "14,10 July,Gap to Mont Ventoux,170.0 km,Road stage",
            "15,11 July,Carpentras to Montpellier,140.5 km,Road stage",
            "16,12 July,Montpellier to Toulouse,160.0 km,Road stage",
            "17,13 July,Toulouse to Saint-Gaudens,190.0 km,Road stage",
            "18,14 July,Saint-Gaudens to La Mongie,135.5 km,Road stage",
            "19,15 July,Bagnères-de-Bigorre to Mourenx,185.5 km,Road stage",
            "20a,16 July,Mourenx to Bordeaux,223.5 km,Road stage",
            "20b,16 July,Bordeaux,8.2 km,Individual time trial",
            "21,17 July,Ruffex to Tours,191.5 km,Road stage",
            "22,18 July,Tours to Versailles,238.5 km,Road stage",
            "23,19 July,Versailles to Paris,54.0 km,Individual time trial"
        ];

        assert_eq!(tdf_1970.route(), route);
        assert_eq!(tdf_1970.distance(), 4368.3);
        assert_eq!(tdf_1970.has_prologue(), true);
        assert_eq!(tdf_1970.number_of_road_stages(), 23);
        assert_eq!(tdf_1970.number_of_individual_time_trials(), 4);
        assert_eq!(tdf_1970.number_of_team_time_trials(), 1);
        assert_eq!(tdf_1970.number_of_rest_days(), 0);
    }
}
