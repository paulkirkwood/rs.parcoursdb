use crate::classic::{Classic,ClassicBuilder,HillyClassic};
use crate::cote::Cote;
use crate::distance::Distance;
use crate::location::Location;
use crate::belgium::{ans,charleroi,esneux,huy,liege,marcinelle,mons,rocourt,spa,tournai,verviers};
use chrono::{Date, Datelike, TimeZone, Utc};
use std::collections::BTreeMap;

pub struct LaFlecheWallonne {
    date: Date<Utc>,
    distance: f64,
    cotes: Vec<(f64,Cote)>
}

impl LaFlecheWallonne {
    fn new(date: Date<Utc>, distance: f64, cotes: Vec<(f64,Cote)>) -> LaFlecheWallonne {
        LaFlecheWallonne { date: date, distance: distance, cotes: cotes }
    }
}

impl Classic for LaFlecheWallonne { 

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        "La Flèche Wallonne".to_string()
    }

    fn start(&self) -> Location {
        let yr = self.date().year();
        if (1936 .. 1938 + 1).contains(&yr) {
            tournai()
        } else if (1939 .. 1947 + 1).contains(&yr) || yr == 1980 {
            mons()
        } else if (1948 .. 1959 + 1).contains(&yr) || (1982 .. 1984 + 1).contains(&yr) {
            charleroi()
        } else if (1960 .. 1971 + 1).contains(&yr) {
            liege()
        } else if (1972 .. 1978 + 1).contains(&yr) {
            verviers()
        } else if yr == 1979 {
            esneux()    
        } else if yr == 1985 {
            huy()
        } else { 
            spa()
        }
    }

    fn finish(&self) -> Location {
        let yr = self.date().year();
        if yr == 1936 || (1946 .. 1959 + 1).contains(&yr) {
            liege()
        } else if yr == 1937 {
            ans()
        } else if (1938 .. 1941 + 1).contains(&yr) {
            rocourt()
        } else if yr == 1942 || (1965 .. 1973 + 1).contains(&yr) || yr == 1979 {
            marcinelle()
        } else if (1943 .. 1954 + 1).contains(&yr) || (1960 .. 1964 + 1).contains(&yr) {
            charleroi()
        } else if (1974 .. 1978 + 1).contains(&yr) {
            verviers()
        } else if yr == 1980 || yr == 1982 {
            spa()
        } else if yr == 1981 {
            mons()
        } else { 
            huy()
        }
    }
}

impl HillyClassic for LaFlecheWallonne {

    fn cotes(&self) -> Vec<String> {
        let mut hills:Vec<String> = Vec::new();
        for (start,cote) in self.cotes.iter() {
            hills.push(format!("{:.1} km,{},{}m", start, cote.name(), cote.height()));
        }
        hills
    }
}

pub struct LaFlecheWallonneBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64,
    cotes: BTreeMap<Distance,Cote>,
}

impl LaFlecheWallonneBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> LaFlecheWallonneBuilder {
        LaFlecheWallonneBuilder { year: year
                                  , month: month
                                  , day: day
                                  , distance: distance
                                  , cotes: BTreeMap::new()
                                  }
    }

    pub fn cote(mut self, name: String, height: i32, km: Distance) -> LaFlecheWallonneBuilder {
        self.cotes.insert(km, Cote::new(name, height));
        self
    }
}

impl ClassicBuilder<LaFlecheWallonne> for LaFlecheWallonneBuilder {
    fn build(&self) -> LaFlecheWallonne {
        let mut hills:Vec<(f64,Cote)> = Vec::new();
        for (km, cote) in self.cotes.iter() {
            let start: f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            hills.push((self.distance - start, Cote::new(cote.name().clone(), cote.height())));
        }

        LaFlecheWallonne::new(Utc.ymd(self.year, self.month, self.day), self.distance, hills)
    }
}
