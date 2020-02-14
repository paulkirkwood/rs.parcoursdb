extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::country::Country;
    use parcoursdb::france::{annecy,boulogne_billancourt,cholet,paris,tours,versailles};
    use parcoursdb::stage_race::Stage;
    use chrono::{TimeZone,Utc};

    #[test]
    fn test_itt() {
        let date   = Utc.ymd(2018, 7, 2);
        let id     = String::from("1");
        let itt    = Stage::itt(date, id, versailles(), Some(paris()), 49.5, Vec::new());
        assert_eq!(itt.start().name(), "Versailles");
        assert_eq!(itt.finish().name(), "Paris");
        assert_eq!(itt.id(), "1");
        assert_eq!(itt.description(), "Individual time trial");
    }

    #[test]
    fn test_prologue() {
        let date     = Utc.ymd(2018, 7, 1);
        let prologue = Stage::prologue(date, boulogne_billancourt(), paris(), 7.6);
        assert_eq!(prologue.start().name(), "Boulogne-Billancourt");
        assert_eq!(prologue.finish().name(), "Paris");
        assert_eq!(prologue.id(), "P");
        assert_eq!(prologue.description(), "Individual time trial");
    }

    #[test]
    fn test_road_stage() {
        let date       = Utc.ymd(2018, 7, 2);
        let id         = String::from("1");
        let road_stage = Stage::road(date, id, paris(), Some(tours()), 256.5, Vec::new());
        assert_eq!(road_stage.start().name(), "Paris");
        assert_eq!(road_stage.finish().name(), "Tours");
        assert_eq!(road_stage.id(), "1");
        assert_eq!(road_stage.description(), "Road stage");
    }

    #[test]
    fn test_ttt() {
        let date  = Utc.ymd(2018, 7, 2);
        let id    = String::from("1");
        let ttt    = Stage::ttt(date, id, cholet(), cholet(), 35.8);
        assert_eq!(ttt.start().name(), "Cholet");
        assert_eq!(ttt.finish().name(), "Cholet");
        assert_eq!(ttt.id(), "1");
        assert_eq!(ttt.description(), "Team time trial");
    }

    #[test]
    fn test_route() {
        let date = Utc.ymd(2018, 7, 16);
        let rest_day = Stage::rest_day(date, annecy());
        assert_eq!(rest_day.route(Country::France), ",16 July,Rest day,Annecy");

        assert_eq!(Stage::transfer_day(Utc.ymd(2018, 7, 16)).route(Country::France), ",16 July,Rest day");
    }
}
