use crate::classic::{Classic,ClassicBuilder,CobbledClassic};
use crate::distance::Distance;
use crate::location::Location;
use crate::france::{argenteuil,chantilly,chatou,compiegne,porte_maillot,saint_denis,saint_germain,suresnes,roubaix};
use crate::pave::{PaveRating,ParisRoubaixPave};
use chrono::{Date, Datelike, TimeZone, Utc};
use std::collections::BTreeMap;

/*
 * Paris-Roubaix: The Queen of the Classics
 */
pub struct ParisRoubaix {
    date: Date<Utc>,
    distance: f64,
    pave: Vec<(i32,f64,ParisRoubaixPave)>
}

impl ParisRoubaix {
    fn new(date: Date<Utc>, distance: f64, pave: Vec<(i32,f64,ParisRoubaixPave)>) -> ParisRoubaix {
        ParisRoubaix { date: date, distance: distance, pave: pave }
    }
}

impl Classic for ParisRoubaix {

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        "Paris-Roubaix".to_string()
    }

    fn start(&self) -> Location {
        let yr = self.date().year();
        if yr == 1938 {
            argenteuil()
        } else if (1966 .. 1977).contains(&yr) {
            chantilly()
        } else if (1898 .. 1900).contains(&yr) || (1902 .. 1914).contains(&yr) {
            chatou()
        } else if (1896 .. 1898).contains(&yr) || (1929 .. 1938).contains(&yr) || yr == 1901 || yr == 1939 {
            porte_maillot()
        } else if (1943 .. 1966).contains(&yr) {
            saint_denis()
        } else if yr == 1900 {
            saint_germain()
        } else if yr == 1914 || (1919 .. 1929).contains(&yr) {
            suresnes()
        } else {
            compiegne()
        }
    }

    fn finish(&self) -> Location {
        roubaix()
    }
}

impl CobbledClassic for ParisRoubaix {

    fn cobbles(&self) -> Vec<String> {
        let mut cobbles:Vec<String> = Vec::new();
        for (id,start,p) in self.pave.iter() {
            cobbles.push(format!("{},{:.1} km,{},{:.1} km,{}", id, start, p.name(), p.length(), p.rating()));
        }
        cobbles
    }
}

pub struct ParisRoubaixBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64,
    sector_id: i32,
    total_sectors: i32,
    pave: BTreeMap<Distance,(i32,ParisRoubaixPave)>,
}

impl ParisRoubaixBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> ParisRoubaixBuilder {
        ParisRoubaixBuilder { year: year
                            , month: month
                            , day: day
                            , distance: distance
                            , sector_id: -1
                            , total_sectors: 0
                            , pave: BTreeMap::new() }
    }

    pub fn five_star_pave(self, name: String, length: f64, km: Distance) -> ParisRoubaixBuilder {
        self.pave(name, length, km, PaveRating::FiveStar)
    }

    pub fn four_star_pave(self, name: String, length: f64, km: Distance) -> ParisRoubaixBuilder {
        self.pave(name, length, km, PaveRating::FourStar)
    }

    pub fn three_star_pave(self, name: String, length: f64, km: Distance) -> ParisRoubaixBuilder {
        self.pave(name, length, km, PaveRating::ThreeStar)
    }

    pub fn two_star_pave(self, name: String, length: f64, km: Distance) -> ParisRoubaixBuilder {
        self.pave(name, length, km, PaveRating::TwoStar)
    }

    pub fn one_star_pave(self, name: String, length: f64, km: Distance) -> ParisRoubaixBuilder {
        self.pave(name, length, km, PaveRating::OneStar)
    }

    pub fn pave(mut self, name: String, length: f64, km: Distance, rating: PaveRating) -> ParisRoubaixBuilder {
        self.pave.insert(km, (self.sector_id, ParisRoubaixPave::new(name, length, rating)));
        self.sector_id -= 1;
        self.total_sectors += 1;
        self
    }

    pub fn multi_sector_pave(mut self, sectors: Vec<(String,f64,PaveRating,Distance)>) -> ParisRoubaixBuilder {
        for (name,length,rating,km) in sectors.iter() {
            self.pave.insert(Distance::new(km.integral(),km.fractional()),
                (self.sector_id, ParisRoubaixPave::new(name.to_string(), *length, *rating)));
        }
        self.sector_id -= 1;
        self.total_sectors += 1;
        self
    }
}

impl ClassicBuilder<ParisRoubaix> for ParisRoubaixBuilder {
    fn build(&self) -> ParisRoubaix {
        let mut sectors:Vec<(i32,f64,ParisRoubaixPave)> = Vec::new();
        let total = self.total_sectors + 1;
        for (km, (id,pave)) in self.pave.iter() {
            let start: f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            sectors.push((*id + total,self.distance - start, ParisRoubaixPave::new(pave.name().clone(), *pave.length(), *(pave.rating()))));
        }
        ParisRoubaix::new(Utc.ymd(self.year, self.month, self.day), self.distance, sectors)
    }
}
