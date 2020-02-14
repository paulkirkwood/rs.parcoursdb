use crate::cote::Cote;
use crate::classic::{Classic,ClassicBuilder,HillyClassic};
use crate::distance::Distance;
use crate::location::Location;
use crate::italy::{milano,san_remo};
use chrono::{Date, TimeZone, Utc};
use std::collections::BTreeMap;

/*
 * Milano - San Remo: La Primavera. 1st "Monument" of the season
 */
pub struct MilanoSanRemo {
    date: Date<Utc>,
    distance: f64,
    cotes: Vec<(f64,Cote)>
}

impl MilanoSanRemo {
    fn new(date: Date<Utc>, distance: f64, cotes: Vec<(f64,Cote)>) -> MilanoSanRemo {
        MilanoSanRemo { date: date, distance: distance, cotes: cotes }
    }
}

impl Classic for MilanoSanRemo {

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        "Milano-San Remo".to_string()
    }

    fn start(&self) -> Location {
        milano()
    }

    fn finish(&self) -> Location {
        san_remo()
    }
}

impl HillyClassic for MilanoSanRemo {

    fn cotes(&self) -> Vec<String> {
        let mut hills:Vec<String> = Vec::new();
        for (start,cote) in self.cotes.iter() {
            hills.push(format!("{:.1} km,{},{}m", start, cote.name(), cote.height()));
        }
        hills
    }
}

pub struct MilanoSanRemoBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64,
    cotes: BTreeMap<Distance,Cote>,
}

impl MilanoSanRemoBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> MilanoSanRemoBuilder {
        MilanoSanRemoBuilder { year: year
                            , month: month
                            , day: day
                            , distance: distance
                            , cotes: BTreeMap::new()
                            }
    }

    pub fn passo_del_turchino(self, km: Distance) -> MilanoSanRemoBuilder {
        self.cote("Passo del Turchino".to_string(), 532, km )
    }

    pub fn capo_mele(self, km: Distance) -> MilanoSanRemoBuilder {
        self.cote("Capo Mele".to_string(), 67, km )
    }

    pub fn capo_cervo(self, km: Distance) -> MilanoSanRemoBuilder {
        self.cote("Capo Cervo".to_string(), 61, km )
    }

    pub fn capo_berta(self, km: Distance) -> MilanoSanRemoBuilder {
        self.cote("Capo Berta".to_string(), 130, km )
    }

    pub fn cipressa(self, km: Distance) -> MilanoSanRemoBuilder {
        self.cote("Cipressa".to_string(), 239, km )
    }

    pub fn poggio(self, km: Distance) -> MilanoSanRemoBuilder {
        self.cote("Poggio".to_string(), 160, km )
    }

    fn cote(mut self, name: String, height: i32, km: Distance) -> MilanoSanRemoBuilder {
        self.cotes.insert(km, Cote::new(name, height));
        self
    }
}

impl ClassicBuilder<MilanoSanRemo> for MilanoSanRemoBuilder {
    fn build(&self) -> MilanoSanRemo {
        let mut hills:Vec<(f64,Cote)> = Vec::new();
        for (km, cote) in self.cotes.iter() {
            let start: f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            hills.push((self.distance - start, Cote::new(cote.name().clone(), cote.height())));
        }

        MilanoSanRemo::new(Utc.ymd(self.year, self.month, self.day), self.distance, hills)
    }
}
