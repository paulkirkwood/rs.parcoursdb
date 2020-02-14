use crate::berg::{Berg,Pavement};
use crate::classic::{BergClassic,Classic,ClassicBuilder,CobbledClassic};
use crate::distance::Distance;
use crate::location::Location;
use crate::belgium::{antwerp,bruges,evergem,gentbrugge,ghent,mariakerke,meerbeke,oudenaarde,sint_niklaas,wetteren};
use crate::pave::{Pave};
use chrono::{Date, Datelike, TimeZone, Utc};
use std::collections::BTreeMap;

pub struct TourOfFlanders {
    date: Date<Utc>,
    distance: f64,
    bergs: Vec<(f64,Berg)>,
    pave: Vec<(f64,Pave)>
}

impl TourOfFlanders {
    fn new(date: Date<Utc>, distance: f64, bergs: Vec<(f64,Berg)>, pave: Vec<(f64,Pave)>) -> TourOfFlanders {
        TourOfFlanders { date: date, distance: distance, bergs: bergs, pave: pave }
    }
}

impl Classic for TourOfFlanders {

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        "Tour of Flanders".to_string()
    }

    fn start(&self) -> Location {
        let yr = self.date().year();
        if (1977 .. 1997 + 1).contains(&yr) {
            sint_niklaas()
        } else if (1998 .. 2016 + 1).contains(&yr) {
            bruges()
        } else if yr >= 2017 {
            antwerp()
        } else {
            ghent()
        }
    }

    fn finish(&self) -> Location {
        let yr = self.date().year();
        if yr == 1913 {
            mariakerke()
        } else if yr == 1914 {
            evergem()
        } else if (1919 .. 1923 + 1).contains(&yr) || (1962 .. 1972 + 1).contains(&yr) {
            gentbrugge()
        } else if (1928 .. 1941 + 1).contains(&yr) || (1945 .. 1961 + 1).contains(&yr) {
            wetteren()
        } else if (1973 .. 2011 + 1).contains(&yr) {
            meerbeke()
        } else if yr >= 2012 {
            oudenaarde()
        } else {
            ghent()
        }
    }
}

impl BergClassic for TourOfFlanders {

    fn bergs(&self) -> Vec<String> {
        let mut bergs:Vec<String> = Vec::new();
        for (i,(start,b)) in self.bergs.iter().enumerate() {
            bergs.push(format!("{},{:.1} km,{},{} km,{}", i + 1, start, b.name(), b.length(), b.pavement()));
        }
        bergs
    }
}

impl CobbledClassic for TourOfFlanders {

    fn cobbles(&self) -> Vec<String> {
        let mut cobbles:Vec<String> = Vec::new();
        for (i,(start,p)) in self.pave.iter().enumerate() {
            cobbles.push(format!("{},{:.1} km,{},{} km", i + 1, start, p.name(), p.length()));
        }
        cobbles
    }
}

pub struct TourOfFlandersBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64,
    bergs: BTreeMap<Distance,Berg>,
    pave: BTreeMap<Distance,Pave>,
}

impl TourOfFlandersBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> TourOfFlandersBuilder {
        TourOfFlandersBuilder { year: year, month: month, day: day, distance: distance, bergs: BTreeMap::new(), pave: BTreeMap::new() }
    }

    pub fn asphalt_berg(mut self, name: String, length: f64, km: Distance) -> TourOfFlandersBuilder {
        self.bergs.insert(km, Berg::new(name, length, Pavement::Asphalt));
        self
    }

    pub fn cobbled_berg(mut self, name: String, length: f64, km: Distance) -> TourOfFlandersBuilder {
        self.bergs.insert(km, Berg::new(name, length, Pavement::Cobbles));
        self
    }

    pub fn pave(mut self, name: String, length: f64, km: Distance) -> TourOfFlandersBuilder {
        self.pave.insert(km, Pave::new(name, length));
        self
    }
}

impl ClassicBuilder<TourOfFlanders> for TourOfFlandersBuilder {
    fn build(&self) -> TourOfFlanders {
        let mut bergs:Vec<(f64,Berg)> = Vec::new();
        for (km,berg) in self.bergs.iter() {
            let start: f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            bergs.push((self.distance - start,Berg::new(berg.name().clone(), *berg.length(), *berg.pavement())));
        }

        let mut pave:Vec<(f64,Pave)> = Vec::new();
        for (km,p) in self.pave.iter() {
            let start: f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            pave.push((self.distance - start,Pave::new(p.name().clone(), *p.length())));
        }

        TourOfFlanders::new(Utc.ymd(self.year, self.month, self.day), self.distance, bergs, pave)
    }
}
