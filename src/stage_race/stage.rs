use crate::col::Col;
use crate::country::Country;
use crate::location::Location;
extern crate chrono;
use chrono::{Date, Utc};

const DATE_FORMAT: &'static str = "%-e %B";

pub enum Stage {
    IndividualTimeTrial         { date: Date<Utc>
                                , id: String
                                , start: Location
                                , finish: Option<Location>
                                , distance: f64
                                , cols: Vec<(f64,Col)>
                                },
    Prologue                    { date: Date<Utc>
                                , id: String
                                , start: Location
                                , finish: Location
                                , distance: f64
                                , cols: Vec<(f64,Col)>
                                },
    PrologueTeamTimeTrial       { date: Date<Utc>
                                , id: String
                                , start: Location
                                , finish: Location
                                , distance: f64
                                , cols: Vec<(f64,Col)>
                                },
    PrologueTwoManTeamTimeTrial { date: Date<Utc>
                                , id: String
                                , start: Location
                                , finish: Location
                                , distance: f64
                                , cols: Vec<(f64,Col)>
                                },
    RestDay                     { date: Date<Utc>
                                , location: Option<Location>
                                },
    Road                        { date: Date<Utc>
                                , id: String
                                , start: Location
                                , finish: Option<Location>
                                , distance: f64
                                , cols: Vec<(f64,Col)>
                                },
    TeamTimeTrial               { date: Date<Utc>
                                , id: String
                                , start: Location
                                , finish: Location
                                , distance: f64
                                , cols: Vec<(f64,Col)>
                                },
    ThreeManTeamTimeTrial       { date: Date<Utc>
                                , id: String
                                , start: Location
                                , finish: Location
                                , distance: f64
                                , cols: Vec<(f64,Col)>
                                }
}

impl Stage {
 
    pub fn itt(date: Date<Utc>, id: String, start: Location, finish: Option<Location>, distance: f64, cols: Vec<(f64,Col)>) -> Stage {
        if distance < 0.0 {
            panic!("ITT distance must be greater than zero");
        }
        Stage::IndividualTimeTrial{ date: date, id: id, start: start, finish: finish, distance: distance, cols: cols }
    }

    pub fn prologue(date: Date<Utc>, start: Location, finish: Location, distance: f64) -> Stage {
        if distance < 0.0 {
            panic!("Prologue distance must be greater than zero");
        }
        Stage::Prologue{ date: date, id: "P".to_string(), start: start, finish: finish, distance: distance, cols: Vec::new() }
    }

    pub fn prologue_ttt(date: Date<Utc>, start: Location, finish: Location, distance: f64) -> Stage {
        if distance < 0.0 {
            panic!("Prologue distance must be greater than zero");
        }
        Stage::PrologueTeamTimeTrial{ date: date, id: "P".to_string(), start: start, finish: finish, distance: distance, cols: Vec::new() }
    }

    pub fn prologue_two_man_ttt(date: Date<Utc>, start: Location, finish: Location, distance: f64) -> Stage {
        if distance < 0.0 {
            panic!("Prologue distance must be greater than zero");
        }
        Stage::PrologueTwoManTeamTimeTrial{ date: date, id: "P".to_string(), start: start, finish: finish, distance: distance,
            cols: Vec::new() }
    }

    pub fn rest_day(date: Date<Utc>, location: Location) -> Stage {
        Stage::RestDay{ date: date, location: Some(location) }
    }

    pub fn transfer_day(date: Date<Utc>) -> Stage {
        Stage::RestDay{ date: date, location: None }
    }
        
    pub fn road(date: Date<Utc>, id: String, start: Location, finish: Option<Location>, distance: f64, cols: Vec<(f64,Col)>) -> Stage {
        if distance < 0.0 {
            panic!("Road stage distance must be greater than zero");
        }
        Stage::Road{ date: date, id: id, start: start, finish: finish, distance: distance, cols: cols }
    }

    pub fn ttt(date: Date<Utc>, id: String, start: Location, finish: Location, distance: f64) -> Stage {
        if distance < 0.0 {
            panic!("TTT distance must be greater than zero");
        }
        Stage::TeamTimeTrial{ date: date, id: id, start: start, finish: finish, distance: distance, cols: Vec::new() }
    }

    pub fn three_man_ttt(date: Date<Utc>, id: String, start: Location, finish: Location, distance: f64) -> Stage {
        if distance < 0.0 {
            panic!("TTT distance must be greater than zero");
        }
        Stage::ThreeManTeamTimeTrial{ date: date, id: id, start: start, finish: finish, distance: distance, cols: Vec::new() }
    }

    pub fn description(&self) -> String {
        match self {
            Stage::PrologueTwoManTeamTimeTrial { .. } => "Two man team time trial".to_string(),
            Stage::RestDay { .. }                     => "Rest day".to_string(),
            Stage::Road { .. }                        => "Road stage".to_string(),
            Stage::PrologueTeamTimeTrial {.. } | Stage::TeamTimeTrial { .. } => "Team time trial".to_string(),
            Stage::ThreeManTeamTimeTrial { .. }       => "Three man team time trial".to_string(),
            _                                         => "Individual time trial".to_string()
        }
    }

    pub fn date(&self) -> &Date<Utc> {
        match self {
            Stage::IndividualTimeTrial         { date, ..  } => date,
            Stage::Prologue                    { date, ..  } => date,
            Stage::PrologueTeamTimeTrial       { date, ..  } => date,
            Stage::PrologueTwoManTeamTimeTrial { date, ..  } => date,
            Stage::RestDay                     { date, ..  } => date,
            Stage::Road                        { date, ..  } => date,
            Stage::TeamTimeTrial               { date, ..  } => date,
            Stage::ThreeManTeamTimeTrial       { date, ..  } => date
        }
    }

    pub fn start(&self) -> &Location {
        match self {
            Stage::IndividualTimeTrial         { date: _d, id: _id, start: s, ..  } => s,
            Stage::Prologue                    { date: _d, id: _id, start: s, ..  } => s,
            Stage::PrologueTeamTimeTrial       { date: _d, id: _id, start: s, ..  } => s,
            Stage::PrologueTwoManTeamTimeTrial { date: _d, id: _id, start: s, ..  } => s,
            Stage::Road                        { date: _d, id: _id, start: s, ..  } => s,
            Stage::TeamTimeTrial               { date: _d, id: _id, start: s, ..  } => s,
            Stage::ThreeManTeamTimeTrial       { date: _d, id: _id, start: s, ..  } => s,
            Stage::RestDay { .. } => panic!("A rest day does not have a 'start'")
        }
    }

    pub fn finish(&self) -> &Location {
        match self {
            Stage::IndividualTimeTrial { date: _d, id: _id, start: _s, finish: f, distance: _di, cols } => {
                match f {
                    Some(fin) => fin,
                    None      => {
                       let (_km, col) = &cols[cols.len() - 1];
                       col.location()
                    }
                }
            },
            Stage::Prologue                    { date: _d, id: _id, start: _s, finish: f, ..  } => f,
            Stage::PrologueTeamTimeTrial       { date: _d, id: _id, start: _s, finish: f, ..  } => f,
            Stage::PrologueTwoManTeamTimeTrial { date: _d, id: _id, start: _s, finish: f, ..  } => f,
            Stage::Road { date: _d, id: _id, start: _s, finish: f, distance: _di, cols } => {
                match f {
                    Some(fin) => fin,
                    None      => {
                       let (_km, col) = &cols[cols.len() - 1];
                       col.location()
                    }
                }
            },
            Stage::TeamTimeTrial         { date: _d, id: _id, start: _s, finish: f, ..  } => f,
            Stage::ThreeManTeamTimeTrial { date: _d, id: _id, start: _s, finish: f, ..  } => f,
            Stage::RestDay { .. } => panic!("A rest day does not have a 'finish'")
        }
    }

    pub fn id(&self) -> &String {
        match self {
            Stage::IndividualTimeTrial         { date: _d, id, ..  } => id,
            Stage::Prologue                    { date: _d, id, ..  } => id,
            Stage::PrologueTeamTimeTrial       { date: _d, id, ..  } => id,
            Stage::PrologueTwoManTeamTimeTrial { date: _d, id, ..  } => id,
            Stage::Road                        { date: _d, id, ..  } => id,
            Stage::TeamTimeTrial               { date: _d, id, ..  } => id,
            Stage::ThreeManTeamTimeTrial       { date: _d, id, ..  } => id,
            Stage::RestDay { .. } => panic!("A rest day does not have an 'id'")
        }
    }

    pub fn distance(&self) -> &f64 {
        match self {
            Stage::IndividualTimeTrial         { date: _d, id: _id, start: _s, finish: _f, distance: d, ..  } => d,
            Stage::Prologue                    { date: _d, id: _id, start: _s, finish: _f, distance: d, ..  } => d,
            Stage::PrologueTeamTimeTrial       { date: _d, id: _id, start: _s, finish: _f, distance: d, ..  } => d,
            Stage::PrologueTwoManTeamTimeTrial { date: _d, id: _id, start: _s, finish: _f, distance: d, ..  } => d,
            Stage::Road                        { date: _d, id: _id, start: _s, finish: _f, distance: d, ..  } => d,
            Stage::TeamTimeTrial               { date: _d, id: _id, start: _s, finish: _f, distance: d, ..  } => d,
            Stage::ThreeManTeamTimeTrial       { date: _d, id: _id, start: _s, finish: _f, distance: d, ..  } => d,
            Stage::RestDay { .. } => panic!("A rest day does not have a 'distance'")
        }
    }

    fn from_to(&self, country: Country) -> String {
        let start = self.start();
        let finish = self.finish();
        if start.name() == finish.name() {
            format!("{}", start.format(country))
        } else {
            format!("{} to {}", start.format(country), finish.format(country))
        }
    }

    pub fn route(&self, country: Country) -> String {
        match self {
            Stage::RestDay { date: d, location } => {
                match location {
                    Some(loc) => format!(",{},{},{}", d.format(DATE_FORMAT), self.description(), loc.format(country)),
                    None      => format!(",{},{}", d.format(DATE_FORMAT), self.description())
                }
            },
            _ => format!("{},{},{},{:?} km,{}", self.id(), self.date().format(DATE_FORMAT),
                self.from_to(country), self.distance(), self.description())
        }
    }

    pub fn cols(&self) -> &Vec<(f64,Col)> {
        match self {
            Stage::IndividualTimeTrial { date: _d, id: _id, start: _s, finish: _f, distance: _di, cols } => &cols,
            Stage::Prologue            { date: _d, id: _id, start: _s, finish: _f, distance: _di, cols } => &cols,
            Stage::Road                { date: _d, id: _id, start: _s, finish: _f, distance: _di, cols } => &cols,
            Stage::PrologueTeamTimeTrial       { .. } |
            Stage::PrologueTwoManTeamTimeTrial { .. } |
            Stage::TeamTimeTrial               { .. } |
            Stage::ThreeManTeamTimeTrial       { .. } |
            Stage::RestDay { .. } => panic!("A rest day does not have 'cols'")
        }
    }

    pub fn profile(&self) -> Vec<String> {
        let mut profile: Vec<String> = Vec::new();
        
        for (km,col) in self.cols().iter() {
            profile.push(format!("{:.1} km,{},{},{}m", km, col.location().name(), col.category(), col.location().elevation().unwrap()));
        }

        profile
    }

    pub fn summit_finish(&self) -> bool {
        match self {
            Stage::IndividualTimeTrial { date: _d, id: _id, start: _s, finish: f, .. } => {
                match f {
                    Some(_fin) => false,
                    None      => true,
                    }
                },
            Stage::Road { date: _d, id: _id, start: _s, finish: f, .. } => {
                match f {
                    Some(_fin) => false,
                    None      => true,
                    }
                },
            _ => false,
        }
    }

    pub fn is_racing_stage(&self) -> bool {
        match self {
            Stage::RestDay { .. } => false,
            _                     => true
        }
    }

    pub fn is_flat_stage(&self) -> bool {
        match self {
            Stage::RestDay { .. } => panic!("A rest day does not have any cols"),
            _ => {
                if self.cols().len() > 0 {
                    false
                } else {
                    true
                }
            }
        }
    }
}
