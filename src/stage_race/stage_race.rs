use crate::country::Country;
use crate::stage_race::Stage;
use chrono::{Date, Datelike, Utc};
extern crate regex;
use regex::Regex;

pub enum StageRace {
    Dauphine         { stages: Vec<Stage> },
    ParisNice        { stages: Vec<Stage> },
    TirrenoAdriatico { stages: Vec<Stage> },
    TourDeFrance     { stages: Vec<Stage> },
    TourOfItaly      { stages: Vec<Stage> },
    TourOfSpain      { stages: Vec<Stage> }
}

impl StageRace {

    pub fn dauphine(stages: Vec<Stage>) -> StageRace {
        StageRace::Dauphine { stages: stages }
    }

    pub fn paris_nice(stages: Vec<Stage>) -> StageRace {
        StageRace::ParisNice { stages: stages }
    }

    pub fn tirreno(stages: Vec<Stage>) -> StageRace {
        StageRace::TirrenoAdriatico { stages: stages }
    }

    pub fn tdf(stages: Vec<Stage>) -> StageRace {
        StageRace::TourDeFrance { stages: stages }
    }

    pub fn giro(stages: Vec<Stage>) -> StageRace {
        StageRace::TourOfItaly { stages: stages }
    }

    pub fn vuelta(stages: Vec<Stage>) -> StageRace {
        StageRace::TourOfSpain { stages: stages }
    }

    pub fn name(&self) -> String {
        match self {
            StageRace::Dauphine { stages: _ } => {
                if self.start_date().year() < 2010 {
                    "Critérium du Dauphiné Libéré".to_string()
                }
                else {
                    "Critérium du Dauphiné".to_string()
                }
            },
            StageRace::ParisNice        { stages: _ } => "Paris-Nice".to_string(),
            StageRace::TirrenoAdriatico { stages: _ } => "Tirreno Adriatico".to_string(),
            StageRace::TourDeFrance     { stages: _ } => "Tour de France".to_string(),
            StageRace::TourOfItaly      { stages: _ } => "Giro d'Italia".to_string(),
            StageRace::TourOfSpain      { stages: _ } => "Vuelta a España".to_string()
        }
    }

    pub fn route(&self) -> Vec<String> {
        let mut rte: Vec<String> = Vec::new();

        for stage in self.stages().iter() {
            rte.push(stage.route(self.country()));
        }
        rte
    }

    pub fn start_date(&self) -> &Date<Utc> {
        let stages = self.stages();
        stages[0].date()
    } 

    pub fn distance(&self) -> f64 {
        self.itt_kms() + self.prologue_kms() + self.road_kms() + self.ttt_kms()
    }

    pub fn itt_kms(&self) -> f64 {
        self.stages().iter().fold(0.0, |acc, stage| match stage {
            Stage::IndividualTimeTrial { date: _d, id: _i, start: _s, finish: _f, distance: d, .. } => acc + d,
            _ => acc + 0.0
        })
    }

    pub fn number_of_individual_time_trials(&self) -> i32 {
        self.stages().iter().fold(0, |acc, stage| match stage {
            Stage::IndividualTimeTrial { .. } => acc + 1,
            _ => acc + 0
        })
    }

    pub fn prologue_kms(&self) -> f64 {
        self.stages().iter().fold(0.0, |acc, stage| match stage {
            Stage::Prologue                    { date: _d, id: _i, start: _s, finish: _f, distance: d, cols: _ } => acc + d,
            Stage::PrologueTeamTimeTrial       { date: _d, id: _i, start: _s, finish: _f, distance: d, cols: _ } => acc + d,
            Stage::PrologueTwoManTeamTimeTrial { date: _d, id: _i, start: _s, finish: _f, distance: d, cols: _ } => acc + d,
            _ => acc + 0.0
        })
    }

    pub fn has_prologue(&self) -> bool {
        if self.prologue_kms() > 0.0 {
            true
        } else {
            false
        }
    }

    pub fn road_kms(&self) -> f64 {
        self.stages().iter().fold(0.0, |acc, stage| match stage {
            Stage::Road { date: _d, id: _i, start: _s, finish: _f, distance: d, .. } => acc + d,
            _ => acc + 0.0
        })
    }

    pub fn number_of_road_stages(&self) -> i32 {
        self.stages().iter().fold(0, |acc, stage| match stage {
            Stage::Road { .. } => acc + 1,
            _ => acc + 0
        })
    }

    pub fn ttt_kms(&self) -> f64 {
        self.stages().iter().fold(0.0, |acc, stage| match stage {
            Stage::TeamTimeTrial { date: _d, id: _i, start: _s, finish: _f, distance: d, cols: _ } => acc + d,
            _ => acc + 0.0
        })
    }

    pub fn number_of_team_time_trials(&self) -> i32 {
        self.stages().iter().fold(0, |acc, stage| match stage {
            Stage::TeamTimeTrial { .. } => acc + 1,
            Stage::ThreeManTeamTimeTrial { .. } => acc + 1,
            _ => acc + 0
        })
    }

    pub fn number_of_rest_days(&self) -> i32 {
        self.stages().iter().fold(0, |acc, stage| match stage {
            Stage::RestDay { .. } => acc + 1,
            _ => acc + 0
        })
    }

    pub fn summit_finishes(&self) -> i32 {
        self.stages().iter().fold(0, |acc, stage| match stage.summit_finish() {
            true  => acc + 1,
            false => acc + 0,
        })
    }

    pub fn country(&self) -> Country {
        match self {
            StageRace::TirrenoAdriatico { .. } => Country::Italy,
            StageRace::TourOfItaly      { .. } => Country::Italy,
            StageRace::TourOfSpain      { .. } => Country::Spain,
            _                                  => Country::France,
        }
    }

    pub fn stages(&self) -> &Vec<Stage> {
        match self {
            StageRace::Dauphine         { stages } => stages,
            StageRace::ParisNice        { stages } => stages,
            StageRace::TirrenoAdriatico { stages } => stages,
            StageRace::TourDeFrance     { stages } => stages,
            StageRace::TourOfItaly      { stages } => stages,
            StageRace::TourOfSpain      { stages } => stages
        }
    }

    pub fn stage_summary(&self) -> String {
        let non_split_re = Regex::new(r"(^\d+[AB]?$)").unwrap();
        let non_split = self.stages().iter().fold(0, |acc, stage| match stage {
            Stage::RestDay { .. } => acc + 0,
            Stage::Prologue { .. } | Stage::PrologueTeamTimeTrial { .. } | Stage::PrologueTwoManTeamTimeTrial { .. } => acc + 0,
            Stage::IndividualTimeTrial { date: _d, id, .. } | Stage::Road { date: _d, id, .. } |
            Stage::TeamTimeTrial { date: _d, id, .. } | Stage::ThreeManTeamTimeTrial { date: _d, id, .. } => {
                if non_split_re.is_match(id) {
                    acc + 1
                } else {
                    acc + 0
                }
            },
        });

        let split_re = Regex::new(r"(^\d+a$)").unwrap();
        let split = self.stages().iter().fold(0, |acc, stage| match stage { 
            Stage::RestDay { .. } => acc + 0,
            Stage::Prologue { .. } => acc + 0,
            Stage::PrologueTeamTimeTrial { .. } => acc + 0,
            Stage::PrologueTwoManTeamTimeTrial { .. } => acc + 0,
            Stage::IndividualTimeTrial { date: _d, id, .. } | Stage::Road { date: _d, id, .. } |
            Stage::TeamTimeTrial { date: _d, id, .. } | Stage::ThreeManTeamTimeTrial { date: _d, id, .. } => {
                if split_re.is_match(id) {
                    acc + 1
                } else {
                    acc + 0
                }
            },
        });

        let total = non_split + split;

        let mut summary: Vec<String> = Vec::new();
        summary.push(format!("{} stages", total));

        if self.has_prologue() {
            summary.push("+ Prologue".to_string());
        }

        if split == 1 {
            summary.push("including 1 split stage".to_string());
        } else if split > 1 {
            summary.push(format!("including {} split stages", split));
        }

        summary.join(" ")
    }

    pub fn racing_stages(&self) -> Vec<&Stage> {
        self.stages().iter().filter(|&s| s.is_racing_stage()).collect()
    }

    pub fn get_stage(&self, id: &String) -> &Stage {
        let stages = self.racing_stages();
        let stage = stages.iter().find(|&s| s.id() == id);
        match stage {
            Some(s) => s,
            None    => panic!("Stage does not exist")
        }
    }
}

