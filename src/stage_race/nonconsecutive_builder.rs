use crate::col::Col;
use crate::location::Location;
use crate::stage_race::{Stage,StageRace};
use chrono::{TimeZone,Utc};

pub struct NonConsecutiveStageRaceBuilder {
    edition: StageRace,
    stages: Vec<Stage>,
    year: i32,
    stage_number: i32,
}

impl NonConsecutiveStageRaceBuilder {
    fn new(edition: StageRace, year: i32) -> NonConsecutiveStageRaceBuilder {
        NonConsecutiveStageRaceBuilder {
            edition: edition,
            stages: Vec::new(),
            year: year,
            stage_number: 1,
        }
    }

    pub fn giro(year: i32) -> NonConsecutiveStageRaceBuilder {
        NonConsecutiveStageRaceBuilder::new(StageRace::TourOfItaly { stages: Vec::new() }, year)
    }

    pub fn tdf(year: i32) -> NonConsecutiveStageRaceBuilder {
        NonConsecutiveStageRaceBuilder::new(StageRace::TourDeFrance { stages: Vec::new() }, year)
    }
    
    pub fn road_stage(mut self, month: u32, day: u32, start: &Location, finish: &Location, distance: f64) -> NonConsecutiveStageRaceBuilder {
        let cols: Vec<(f64,Col)> = Vec::new();
        let stage = Stage::road(Utc.ymd(self.year, month, day), format!("{}", self.stage_number), start.clone(),
            Some(finish.clone()), distance, cols);
        self.stages.push(stage);        
        self.stage_number += 1;
        self
    }

    pub fn team_time_trial(self, month: u32, day: u32, start: &Location, finish: &Location, distance: f64) -> NonConsecutiveStageRaceBuilder {
        self.ttt(month, day, start, finish, distance)
    }

    pub fn ttt(mut self, month: u32, day: u32, start: &Location, finish: &Location, distance: f64) -> NonConsecutiveStageRaceBuilder {
        let stage = Stage::ttt(Utc.ymd(self.year, month, day), format!("{}", self.stage_number),
            start.clone(), finish.clone(), distance);
        self.stages.push(stage);
        self.stage_number += 1;
        self
    }

    pub fn build(self) -> StageRace {
        match self.edition {
            StageRace::TourDeFrance { .. } => StageRace::TourDeFrance { stages: self.stages },
            StageRace::TourOfItaly  { .. } => StageRace::TourOfItaly  { stages: self.stages },
            _ => panic!("Only the Tour de France and the Giro had non-consecutive stages")
        }
    }
}
