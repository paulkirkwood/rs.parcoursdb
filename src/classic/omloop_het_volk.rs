use crate::berg::{Berg,Pavement};
use crate::classic::{BergClassic,Classic,ClassicBuilder,CobbledClassic};
use crate::distance::Distance;
use crate::location::Location;
use crate::belgium::{ghent,lokeren};
use crate::pave::{Pave};
use chrono::{Date, Datelike, TimeZone, Utc};
use std::collections::BTreeMap;

pub struct OmloopHetVolk {
    date: Date<Utc>,
    distance: f64,
    bergs: Vec<(f64,Berg)>,
    pave: Vec<(f64,Pave)>
}

impl OmloopHetVolk {
    fn new(date: Date<Utc>, distance: f64, bergs: Vec<(f64,Berg)>, pave: Vec<(f64,Pave)>) -> OmloopHetVolk {
        OmloopHetVolk { date: date, distance: distance, bergs: bergs, pave: pave }
    }
}

impl Classic for OmloopHetVolk {

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        let yr = self.date().year();
        if yr == 1945 || yr == 1946 {
            "Omloop van Vlaanderen".to_string()
        } else if ( 1947 .. 2008 + 1).contains(&yr) {
            "Omloop Het Volk".to_string()
        } else {
            "Omloop Het Nieuwsblad".to_string()
        }
    }

    fn start(&self) -> Location {
        ghent()
    }

    fn finish(&self) -> Location {
        let yr = self.date().year();
        if (1996 .. 2008).contains(&yr) {
            lokeren()
        } else {
            ghent()
        }
    }
}

impl BergClassic for OmloopHetVolk {

    fn bergs(&self) -> Vec<String> {
        let mut bergs:Vec<String> = Vec::new();
        for (i,(start,b)) in self.bergs.iter().enumerate() {
            bergs.push(format!("{},{:.1} km,{},{:.1} km,{}", i + 1, start, b.name(), b.length(), b.pavement()));
        }
        bergs
    }
}

impl CobbledClassic for OmloopHetVolk {

    fn cobbles(&self) -> Vec<String> {
        let mut cobbles:Vec<String> = Vec::new();
        for (i,(start,p)) in self.pave.iter().enumerate() {
            cobbles.push(format!("{},{:.1} km,{},{} km", i + 1, start, p.name(), p.length()));
        }
        cobbles
    }
}

pub struct OmloopHetVolkBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64,
    bergs: BTreeMap<Distance,Berg>,
    pave: BTreeMap<Distance,Pave>,
}

impl OmloopHetVolkBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> OmloopHetVolkBuilder {
        OmloopHetVolkBuilder { year: year, month: month, day: day, distance: distance, bergs: BTreeMap::new(), pave: BTreeMap::new() }
    }

    pub fn asphalt_berg(mut self, name: String, length: f64, km: Distance) -> OmloopHetVolkBuilder {
        self.bergs.insert(km, Berg::new(name, length, Pavement::Asphalt));
        self
    }

    pub fn cobbled_berg(mut self, name: String, length: f64, km: Distance) -> OmloopHetVolkBuilder {
        self.bergs.insert(km, Berg::new(name, length, Pavement::Cobbles));
        self
    }

    pub fn pave(mut self, name: String, length: f64, km: Distance) -> OmloopHetVolkBuilder {
        self.pave.insert(km, Pave::new(name, length));
        self
    }
}

impl ClassicBuilder<OmloopHetVolk> for OmloopHetVolkBuilder {
    fn build(&self) -> OmloopHetVolk {

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

        OmloopHetVolk::new(Utc.ymd(self.year, self.month, self.day), self.distance, bergs, pave)
    }
}
