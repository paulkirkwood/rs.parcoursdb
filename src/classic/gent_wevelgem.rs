use crate::classic::{Classic,ClassicBuilder};
use crate::location::Location;
use crate::belgium::{deinze,ghent,wevelgem};
use chrono::{Date, Datelike, TimeZone, Utc};

pub struct GentWevelgem {
    date: Date<Utc>,
    distance: f64
}

impl GentWevelgem {
    fn new(date: Date<Utc>, distance: f64) -> GentWevelgem {
        GentWevelgem { date: date, distance: distance }
    }
}

impl Classic for GentWevelgem {

    fn date(&self) -> &Date<Utc> {
        &self.date
    }

    fn distance(&self) -> &f64 {
        &self.distance
    }

    fn name(&self) -> String {
        "Gent-Wevelgem".to_string()
    }

    fn start(&self) -> Location {
        if self.date().year() > 2013 {
            deinze()
        } else {
            ghent()
        }
    }

    fn finish(&self) -> Location {
        wevelgem()
    }
}

pub struct GentWevelgemBuilder {
    year: i32,
    month: u32,
    day: u32,
    distance: f64
}

impl GentWevelgemBuilder {
    pub fn new(year: i32, month: u32, day: u32, distance: f64) -> GentWevelgemBuilder {
        GentWevelgemBuilder { year: year, month: month, day: day, distance: distance }
    }
}

impl ClassicBuilder<GentWevelgem> for GentWevelgemBuilder {
    fn build(&self) -> GentWevelgem {
        GentWevelgem::new(Utc.ymd(self.year, self.month, self.day), self.distance)
    }
}
