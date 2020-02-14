use crate::classic::{Classic,ClassicBuilder};
use crate::location::Location;
use crate::belgium::harelbeke;
use chrono::{Date, Datelike, TimeZone, Utc};

pub struct E3Harelbeke {
    date: Date<Utc>,
    distance: f64
}

impl E3Harelbeke {
    fn new(date: Date<Utc>, distance: f64) -> E3Harelbeke {
        E3Harelbeke { date: date, distance: distance }
    }
}

impl Classic for E3Harelbeke {

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        let yr = self.date().year();
        if (1958 .. 1961 + 1).contains(&yr) {
            return "Harelbeke-Antwerp-Harelbeke".to_string();
        } else if (1962 .. 2013 + 1).contains(&yr) {
            return "E3 Prijs Vlaanderen".to_string();
        } else if (2014 .. 2018 + 1).contains(&yr) {
            return "E3 Harelbeke".to_string();
        }

        return "E3 BinckBank Classic".to_string();
    }

    fn start(&self) -> Location {
        harelbeke()
    }

    fn finish(&self) -> Location {
        self.start()
    }
}

pub struct E3HarelbekeBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64
}

impl E3HarelbekeBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> E3HarelbekeBuilder {
        E3HarelbekeBuilder { year: year, month: month, day: day, distance: distance }
    }
}

impl ClassicBuilder<E3Harelbeke> for E3HarelbekeBuilder {
    fn build(&self) -> E3Harelbeke {
        E3Harelbeke::new(Utc.ymd(self.year, self.month, self.day), self.distance)
    }
}
