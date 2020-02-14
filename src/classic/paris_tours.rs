use crate::classic::{Classic,ClassicBuilder,GravelClassic,HillyClassic};
use crate::cote::Cote;
use crate::distance::Distance;
use crate::gravel::Gravel;
use crate::location::Location;
use crate::france::{blois,brou,chartres,chaville,creteil,la_loupe,montlhery,paris,saint_arnoult_en_yvelines,tours,versailles};
use chrono::{Date, Datelike, TimeZone, Utc};
use std::collections::BTreeMap;

/*
 * Paris - Tours: "The Sprinter's classic"
 */
pub struct ParisTours {
    date: Date<Utc>,
    distance: f64,
    cotes: Vec<(f64,Cote)>,
    gravel: Vec<(i32,f64,Gravel)>
}

impl ParisTours {
    fn new(date: Date<Utc>, distance: f64, cotes: Vec<(f64,Cote)>, gravel: Vec<(i32,f64,Gravel)>) -> ParisTours {
        ParisTours { date: date, distance: distance, cotes: cotes, gravel: gravel }
    }
}

impl Classic for ParisTours {

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        let yr = self.date().year();
        if (1976 .. 1988).contains(&yr) {
            "Grand Prix d'Automne".to_string()
        } else {
            "Paris-Tours".to_string()
        }
    }

    fn start(&self) -> Location {
        let yr = self.date().year();
        if (1974 .. 1977 + 1).contains(&yr) {
            tours()
        } else if (1978 .. 1984 + 1).contains(&yr) {
            blois()
        } else if (1985 .. 1987 + 1).contains(&yr) {
            creteil()
        } else if (1988 .. 1990 + 1).contains(&yr) {
            chaville()
        } else if (1993 .. 2008 + 1).contains(&yr) || yr == 2014 {
            saint_arnoult_en_yvelines()
        } else if yr == 2009 {
            chartres()
        } else if yr == 2010 {
            la_loupe()
        } else if yr == 2017 {
            brou()
        } else {
            paris()
        }
    }

    fn finish(&self) -> Location {
        let yr = self.date().year();
        if (1974 .. 1977 + 1).contains(&yr) {
            versailles()
        } else if yr == 1978 {
            montlhery()
        } else if (1979 .. 1987 + 1).contains(&yr) {
            chaville()
        } else { 
            tours()
        }
    }
}

impl GravelClassic for ParisTours {

    fn gravel(&self) -> Vec<String> {
        let mut gravel:Vec<String> = Vec::new();
        for (id, start,g) in self.gravel.iter() {
            gravel.push(format!("{},{:.1} km,{},{:.1} km", id, start, g.name(), g.length()));
        }
        gravel
    }
}

impl HillyClassic for ParisTours {

    fn cotes(&self) -> Vec<String> {
        let mut hills:Vec<String> = Vec::new();
        for (start,cote) in self.cotes.iter() {
            hills.push(format!("{:.1} km,{},{}m", start, cote.name(), cote.height()));
        }
        hills
    }
}

pub struct ParisToursBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64,
    sector_id: i32,
    total_sectors: i32,
    gravel: BTreeMap<Distance,(i32,Gravel)>,
    cotes: BTreeMap<Distance,Cote>,
}

impl ParisToursBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> ParisToursBuilder {
        ParisToursBuilder { year: year
                          , month: month
                          , day: day
                          , distance: distance
                          , sector_id: -1
                          , total_sectors: 0
                          , gravel: BTreeMap::new()
                          , cotes: BTreeMap::new()
                          }
    }

    pub fn gravel(mut self, name: String, length: f64, km: Distance) -> ParisToursBuilder {
        self.gravel.insert(km, (self.sector_id, Gravel::new(name, length)));
        self.sector_id -= 1;
        self.total_sectors += 1;
        self
    }

    pub fn cote(mut self, name: String, height: i32, km: Distance) -> ParisToursBuilder {
        self.cotes.insert(km, Cote::new(name, height));
        self
    }
}

impl ClassicBuilder<ParisTours> for ParisToursBuilder {
    fn build(&self) -> ParisTours {
        let mut gravel:Vec<(i32,f64,Gravel)> = Vec::new();
        let total = self.total_sectors + 1;
        for (km,(id,g)) in self.gravel.iter() {
            let start: f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            gravel.push((*id + total,self.distance - start, Gravel::new(g.name().clone(), *g.length())));
        }

        let mut hills:Vec<(f64,Cote)> = Vec::new();
        for (km, cote) in self.cotes.iter() {
            let start: f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            hills.push((self.distance - start, Cote::new(cote.name().clone(), cote.height())));
        }

        ParisTours::new(Utc.ymd(self.year, self.month, self.day), self.distance, hills, gravel)
    }
}
