use crate::location::Location;
use chrono::{Date,Utc};

pub trait Classic {
    fn date(&self) -> &Date<Utc>;
    fn distance(&self) -> &f64;
    fn name(&self) -> String;
    fn start(&self) -> Location;
    fn finish(&self) -> Location;
}

pub trait ClassicBuilder<C: Classic> {
    fn build(&self) -> C;
}

pub trait BergClassic {
    fn bergs(&self) -> Vec<String>;
}

pub trait CobbledClassic {
    fn cobbles(&self) -> Vec<String>;
}

pub trait HillyClassic {
    fn cotes(&self) -> Vec<String>;
}

pub trait GravelClassic {
    fn gravel(&self) -> Vec<String>;
}
