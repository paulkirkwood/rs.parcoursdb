use crate::classic::{Classic,ClassicBuilder,HillyClassic};
use crate::cote::Cote;
use crate::distance::Distance;
use crate::location::Location;
use crate::belgium::{ans,liege};
use chrono::{Date, Datelike, TimeZone, Utc};
use std::collections::BTreeMap;

/*
 * Liège-Bastogne-Liège - "La Doyenne"
 */
pub struct LiegeBastogneLiege {
    date: Date<Utc>,
    distance: f64,
    cotes: Vec<(f64,Cote)>
}

impl LiegeBastogneLiege {
    fn new(date: Date<Utc>, distance: f64, cotes: Vec<(f64,Cote)>) -> LiegeBastogneLiege {
        LiegeBastogneLiege { date: date, distance: distance, cotes: cotes }
    }
}

impl Classic for LiegeBastogneLiege {

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        "Liège-Bastogne-Liège".to_string()
    }

    fn start(&self) -> Location {
        liege()
    }

    fn finish(&self) -> Location {
        let yr = self.date().year();
        if (1992 .. 2019).contains(&yr) {
            ans()
        } else {
           liege()
        }
    }
}

impl HillyClassic for LiegeBastogneLiege {

    fn cotes(&self) -> Vec<String> {
        let mut hills:Vec<String> = Vec::new();
        for (start,cote) in self.cotes.iter() {
            hills.push(format!("{:.1} km,{},{}m", start, cote.name(), cote.height()));
        }
        hills
    }
}

pub struct LiegeBastogneLiegeBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64,
    cotes: BTreeMap<Distance,Cote>,
}
    
impl LiegeBastogneLiegeBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> LiegeBastogneLiegeBuilder {
        LiegeBastogneLiegeBuilder { year: year
                                  , month: month
                                  , day: day
                                  , distance: distance
                                  , cotes: BTreeMap::new()
                                  }
    }   

    pub fn cote(mut self, name: String, height: i32, km: Distance) -> LiegeBastogneLiegeBuilder {
        self.cotes.insert(km, Cote::new(name, height));
        self
    }
}

impl ClassicBuilder<LiegeBastogneLiege> for LiegeBastogneLiegeBuilder {
    fn build(&self) -> LiegeBastogneLiege {
        let mut hills:Vec<(f64,Cote)> = Vec::new();
        for (km, cote) in self.cotes.iter() {
            let start: f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            hills.push((self.distance - start, Cote::new(cote.name().clone(), cote.height())));
        }

        LiegeBastogneLiege::new(Utc.ymd(self.year, self.month, self.day), self.distance, hills)
    }
}
