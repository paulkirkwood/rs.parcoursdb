use crate::classic::{Classic,ClassicBuilder,HillyClassic};
use crate::cote::Cote;
use crate::distance::Distance;
use crate::location::Location;
use crate::italy::{bergamo,cantu,como,lecco,milano,monza,varese};
use crate::switzerland::mendrisio;
use chrono::{Date, Datelike, TimeZone, Utc};
use std::collections::BTreeMap;

pub struct TourOfLombardy {
    date: Date<Utc>,
    distance: f64,
    cotes: Vec<(f64,Cote)>
}

impl TourOfLombardy {
    fn new(date: Date<Utc>, distance: f64, cotes: Vec<(f64,Cote)>) -> TourOfLombardy {
        TourOfLombardy { date: date, distance: distance, cotes: cotes }
    }
}

impl Classic for TourOfLombardy { 

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        "Il Lombardia".to_string()
    }

    fn start(&self) -> Location {
        let yr = self.date().year();
        if (1905 .. 1984 + 1).contains(&yr) || (1990 .. 1994 + 1).contains(&yr) || [2010, 2011].contains(&yr) {
            milano()
        } else if (1985 .. 1989 + 1).contains(&yr) || [2003, 2014, 2016].contains(&yr) {
            como()
        } else if (1995 .. 2001 + 1).contains(&yr) || [2001, 2007, 2008, 2009].contains(&yr) {
            varese()
        } else if [2004, 2005, 2006].contains(&yr) {
            mendrisio()
        } else if yr == 2002 {
            cantu()
        } else {
            bergamo()
        }
    }

    fn finish(&self) -> Location {
        let yr = self.date().year();
        if [1990, 1991, 1992, 1993, 1994].contains(&yr) {
            monza()
        } else if [2011, 2012, 2013].contains(&yr) {
            lecco()
        } else if (1905 .. 1960 + 1).contains(&yr) || [1985, 1986, 1987, 1988, 1989].contains(&yr) {
            milano()
        } else if (1995 .. 2003 + 1).contains(&yr) || [2014, 2016].contains(&yr) {
            bergamo()
        } else {
            como()
        }
    }
}

impl HillyClassic for TourOfLombardy {

    fn cotes(&self) -> Vec<String> {
        let mut hills:Vec<String> = Vec::new();
        for (start,cote) in self.cotes.iter() {
            hills.push(format!("{:.1} km,{},{}m", start, cote.name(), cote.height()));
        }
        hills
    }
}

pub struct TourOfLombardyBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64,
    cotes: BTreeMap<Distance,Cote>,
}

impl TourOfLombardyBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> TourOfLombardyBuilder {
        TourOfLombardyBuilder { year: year
                                  , month: month
                                  , day: day
                                  , distance: distance
                                  , cotes: BTreeMap::new()
                                  }
    }

    pub fn cote(mut self, name: String, height: i32, km: Distance) -> TourOfLombardyBuilder {
        self.cotes.insert(km, Cote::new(name, height));
        self
    }
}

impl ClassicBuilder<TourOfLombardy> for TourOfLombardyBuilder {
    fn build(&self) -> TourOfLombardy {
        let mut hills:Vec<(f64,Cote)> = Vec::new();
        for (km, cote) in self.cotes.iter() {
            let start: f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            hills.push((self.distance - start, Cote::new(cote.name().clone(), cote.height())));
        }

        TourOfLombardy::new(Utc.ymd(self.year, self.month, self.day), self.distance, hills)
    }
}
