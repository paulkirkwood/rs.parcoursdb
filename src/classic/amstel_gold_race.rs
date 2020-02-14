use crate::classic::{Classic,ClassicBuilder};
use crate::location::Location;
use crate::netherlands::{breda, berg_en_terblijt, elsloo, helmond, heerlen, maastricht, meerssen, valkenburg};
use chrono::{Date, Datelike, TimeZone, Utc};

/*
 * Amstel Gold Race: The only Dutch classic
 */
pub struct AmstelGoldRace {
    date: Date<Utc>,
    distance: f64
}

impl AmstelGoldRace {
    fn new(date: Date<Utc>, distance: f64) -> AmstelGoldRace {
        AmstelGoldRace { date: date, distance: distance }
    }
}

impl Classic for AmstelGoldRace {

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        "Amstel Gold Race".to_string()
    }

    fn start(&self) -> Location {
        let yr = self.date().year();
        if yr == 1966 {
            breda()
        } else if (1967 .. 1970 + 1).contains(&yr) {
            helmond()
        } else if (1971 .. 1997 + 1).contains(&yr) {
            heerlen()
        } else {
            maastricht()
        }
    }

    fn finish(&self) -> Location {
        let yr = self.date().year();
        if yr == 1966 || (1969 .. 1990 + 1).contains(&yr) {
            meerssen()
        } else if yr == 1968 {
            elsloo()
        } else if (1991 .. 2002 + 1).contains(&yr) {
            maastricht()
        } else if (2003 .. 2012 + 1).contains(&yr) {
            valkenburg()
        } else {
            berg_en_terblijt()
        }
    }
}

pub struct AmstelGoldRaceBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64
}

impl AmstelGoldRaceBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> AmstelGoldRaceBuilder {
        AmstelGoldRaceBuilder { year: year, month: month, day: day, distance: distance }
    }
}

impl ClassicBuilder<AmstelGoldRace> for AmstelGoldRaceBuilder {
    fn build(&self) -> AmstelGoldRace {
        AmstelGoldRace::new(Utc.ymd(self.year, self.month, self.day), self.distance)
    }
}
