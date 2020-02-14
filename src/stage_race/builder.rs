use crate::col::{Col,ColCategory};
use crate::distance::Distance;
use crate::location::Location;
use crate::stage_race::{Stage,StageRace};
use crate::stage_race::util::*;
use chrono::{Date,Utc};
use std::collections::BTreeMap;

pub struct StageRaceBuilder {
    edition: StageRace,
    stages: Vec<Stage>,
    date: Date<Utc>,
    stage_number: i32,
    split_stages: bool,
    suffix: char,
    cols: BTreeMap<Distance,Col>,
    mountain_itt: bool,
    a_b_stage: bool,
    morning_stage: bool
}

impl StageRaceBuilder {
    fn new(edition: StageRace, date: Date<Utc>) -> StageRaceBuilder {
        StageRaceBuilder {
            edition: edition,
            stages: Vec::new(),
            date: date,
            stage_number: 1,
            split_stages: false,
            suffix: 'a',
            cols: BTreeMap::new(),
            mountain_itt: false,
            a_b_stage: false,
            morning_stage: false
        }
    }

    pub fn dauphine(date: Date<Utc>) -> StageRaceBuilder {
        StageRaceBuilder::new(StageRace::Dauphine { stages: Vec::new() }, date)
    }

    pub fn giro(date: Date<Utc>) -> StageRaceBuilder {
        StageRaceBuilder::new(StageRace::TourOfItaly { stages: Vec::new() }, date)
    }

    pub fn paris_nice(date: Date<Utc>) -> StageRaceBuilder {
        StageRaceBuilder::new(StageRace::ParisNice { stages: Vec::new() }, date)
    }

    pub fn tdf(date: Date<Utc>) -> StageRaceBuilder {
        StageRaceBuilder::new(StageRace::TourDeFrance { stages: Vec::new() }, date)
    }
    
    pub fn tirreno(date: Date<Utc>) -> StageRaceBuilder {
        StageRaceBuilder::new(StageRace::TirrenoAdriatico { stages: Vec::new() }, date)
    }

    pub fn vuelta(date: Date<Utc>) -> StageRaceBuilder {
        StageRaceBuilder::new(StageRace::TourOfSpain { stages: Vec::new() }, date)
    }
    
    pub fn prologue(mut self, start: &Location, finish: &Location, distance: f64) -> StageRaceBuilder {
        let stage = Stage::prologue(self.date, start.clone(), finish.clone(), distance);
        self.stages.push(stage);        
        if self.split_stages == false {
            self.date = self.date.succ();
        }
        self
    }

    pub fn out_and_back_prologue(self, start: &Location, distance: f64) -> StageRaceBuilder {
        self.prologue(start, start, distance)
    }

    pub fn prologue_ttt(mut self, start: &Location, finish: &Location, distance: f64) -> StageRaceBuilder {
        let stage = Stage::prologue_ttt(self.date, start.clone(), finish.clone(), distance);
        self.stages.push(stage);        
        if self.split_stages == false {
            self.date = self.date.succ();
        }
        self
    }

    pub fn prologue_two_man_ttt(mut self, start: &Location, distance: f64) -> StageRaceBuilder {
        let stage = Stage::prologue_two_man_ttt(self.date, start.clone(), start.clone(), distance);
        self.stages.push(stage);
        if self.split_stages == false {
            self.date = self.date.succ();
        }
        self
    }

    pub fn road_stage(mut self, start: &Location, finish: &Location, distance: f64) -> StageRaceBuilder {

        /*
         * Copy the cols into the stage
         */
        let mut cols:Vec<(f64,Col)> = Vec::new();
        for (km, col) in self.cols.iter() {
            let km_f64:f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            cols.push((km_f64,
                Col::new(col.location().clone(), col.category(), col.length(), col.average_gradient(), col.maximum_gradient())));
        }

        /*
         * Create the stage
         */
        let id = stage_id(self.stage_number, self.suffix, self.split_stages || self.a_b_stage);
        let stage = Stage::road(self.date, id, start.clone(), Some(finish.clone()), distance, cols);
        self.stages.push(stage);        

        /*
         * Do the housekeeping
         */
        if self.split_stages == true || self.a_b_stage == true {
            let suffix = increment_suffix(self.suffix);
            self.suffix = suffix;
            if self.a_b_stage == true {
                self.date = self.date.succ();
            }
        } else {
            self.stage_number += 1;
            self.date = self.date.succ();
        }

        self
    }

    pub fn criterium(self, start: &Location, distance: f64) -> StageRaceBuilder {
        self.road_stage(start, start, distance)
    }

    pub fn ttt(mut self, start: &Location, finish: &Location, distance: f64) -> StageRaceBuilder {
        let id = stage_id(self.stage_number, self.suffix, self.split_stages);
        let stage = Stage::ttt(self.date, id, start.clone(), finish.clone(), distance);
        self.stages.push(stage);
        if self.split_stages == true {
            let suffix = increment_suffix(self.suffix);
            self.suffix = suffix;
        }
        else {
            self.stage_number += 1;
            self.date = self.date.succ();
        }         
        self
    }

    pub fn out_and_back_ttt(self, start: &Location, distance: f64) -> StageRaceBuilder {
        self.ttt(start, start, distance)
    }

    pub fn three_man_ttt(mut self, start: &Location, distance: f64) -> StageRaceBuilder {
        let id = stage_id(self.stage_number, self.suffix, self.split_stages);
        let stage = Stage::three_man_ttt(self.date, id, start.clone(), start.clone(), distance);
        self.stages.push(stage);
        if self.split_stages == true {
            let suffix = increment_suffix(self.suffix);
            self.suffix = suffix;
        }
        else {
            self.stage_number += 1;
            self.date = self.date.succ();
        }
        self
    }

    pub fn itt(mut self, start: &Location, finish: &Location, distance: f64) -> StageRaceBuilder {
        let cols:Vec<(f64,Col)> = Vec::new();
        let id = stage_id(self.stage_number, self.suffix, self.split_stages || self.a_b_stage);
        let stage = Stage::itt(self.date, id, start.clone(), Some(finish.clone()), distance, cols);
        self.stages.push(stage);        

        if self.split_stages == true || self.a_b_stage == true {
            let suffix = increment_suffix(self.suffix);
            self.suffix = suffix;
            if self.a_b_stage == true {
                self.date = self.date.succ();
            }
        } else {
            self.stage_number += 1;
            self.date = self.date.succ();
        }

        self
    }

    pub fn out_and_back_itt(self, start: &Location, distance: f64) -> StageRaceBuilder {
        self.itt(start, start, distance)
    }

    pub fn rest_day(mut self, location: &Location) -> StageRaceBuilder {
        let rest_day = Stage::rest_day(self.date, location.clone());
        self.stages.push(rest_day);
        self.date = self.date.succ();
        self
    }

    pub fn transfer_day(mut self) -> StageRaceBuilder {
        let transfer_day = Stage::transfer_day(self.date);
        self.stages.push(transfer_day);
        self.date = self.date.succ();
        self
    }

    pub fn build(self) -> StageRace {
        match self.edition {
            StageRace::Dauphine         { .. } => StageRace::Dauphine         { stages: self.stages },
            StageRace::ParisNice        { .. } => StageRace::ParisNice        { stages: self.stages },
            StageRace::TirrenoAdriatico { .. } => StageRace::TirrenoAdriatico { stages: self.stages },
            StageRace::TourDeFrance     { .. } => StageRace::TourDeFrance     { stages: self.stages },
            StageRace::TourOfItaly      { .. } => StageRace::TourOfItaly      { stages: self.stages },
            StageRace::TourOfSpain      { .. } => StageRace::TourOfSpain      { stages: self.stages },
        }
    }

    pub fn enable_split_stages(mut self) -> StageRaceBuilder {
        self.split_stages = true;
        self.suffix = 'a';
        self
    }

    pub fn disable_split_stages(mut self) -> StageRaceBuilder {
        self.split_stages = false;
        self.date = self.date.succ();
        self.stage_number += 1;
        self
    }

    pub fn enable_a_b_stage(mut self) -> StageRaceBuilder {
        self.a_b_stage = true;
        self.suffix = 'A';
        self
    }

    pub fn disable_a_b_stage(mut self) -> StageRaceBuilder {
        self.a_b_stage = false;
        self.stage_number += 1;
        self
    }

    pub fn enable_morning_stage(mut self) -> StageRaceBuilder {
        self.morning_stage = true;
        self
    }

    pub fn mountain_stage(mut self, start: &Location) -> StageRaceBuilder {
        let id = stage_id(self.stage_number, self.suffix, self.split_stages);
        let stage = Stage::road(self.date, id, start.clone(), None, 0.0, Vec::new());
        self.stages.push(stage);

        if self.split_stages == true {
            let suffix = increment_suffix(self.suffix);
            self.suffix = suffix;
        }
        else {
            self.stage_number += 1;
            self.date = self.date.succ();
        }

        self.mountain_itt = false;
        self
    }

    pub fn mountain_time_trial(mut self, start: &Location) -> StageRaceBuilder {
        let id = stage_id(self.stage_number, self.suffix, self.split_stages);
        let stage = Stage::itt(self.date, id, start.clone(), None, 0.0, Vec::new());
        self.stages.push(stage);

        if self.split_stages == true {
            let suffix = increment_suffix(self.suffix);
            self.suffix = suffix;
        }
        else {
            self.stage_number += 1;
            self.date = self.date.succ();
        }

        self.mountain_itt = true;
        self
    }

    pub fn hc_col(self, location: &Location, km: Distance) -> StageRaceBuilder {
        self.col(location, ColCategory::HC, km)
    }

    pub fn c1_col(self, location: &Location, km: Distance) -> StageRaceBuilder {
        self.col(location, ColCategory::C1, km)
    }

    pub fn c2_col(self, location: &Location, km: Distance) -> StageRaceBuilder {
        self.col(location, ColCategory::C2, km)
    }

    pub fn c3_col(self, location: &Location, km: Distance) -> StageRaceBuilder {
        self.col(location, ColCategory::C3, km)
    }

    pub fn c4_col(self, location: &Location, km: Distance) -> StageRaceBuilder {
        self.col(location, ColCategory::C4, km)
    }

    fn col(mut self, location: &Location, category: ColCategory, km: Distance) -> StageRaceBuilder {
        let c = Col::new(location.clone(), category, None, None, None);
        self.cols.insert(km, c);
        self
    }

    pub fn summit_finish(mut self, location: &Location, category: ColCategory, km: Distance) -> StageRaceBuilder {

        let c = Col::new(location.clone(), category, None, None, None);
        self.cols.insert(km, c);

        let mut cols:Vec<(f64,Col)> = Vec::new();
        for (km, col) in self.cols.iter() {
            cols.push((km.to_f64(), col.clone()));
        }

        if self.mountain_itt == true {
            let stage = self.stages.pop().unwrap();
            let s = Stage::itt(*stage.date(), stage.id().to_string(), stage.start().clone(), None, km.to_f64(), cols);
            self.stages.push(s);
        } else {
            let stage = self.stages.pop().unwrap();
            let s = Stage::road(*stage.date(), stage.id().to_string(), stage.start().clone(), None, km.to_f64(), cols);
            self.stages.push(s);
        }

        self.cols.clear();

        self
    }

    pub fn morning_road_stage(mut self, start: &Location, finish: &Location, distance: f64) -> StageRaceBuilder {

        /*
         * Copy the cols into the stage
         */
        let mut cols:Vec<(f64,Col)> = Vec::new();
        for (km, col) in self.cols.iter() {
            let km_f64:f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            cols.push((km_f64,
                Col::new(col.location().clone(), col.category(), col.length(), col.average_gradient(), col.maximum_gradient())));
        }

        /*
         * Create the stage
         */
        let id = stage_id(self.stage_number, self.suffix, self.split_stages || self.a_b_stage);
        let stage = Stage::road(self.date, id, start.clone(), Some(finish.clone()), distance, cols);
        self.stages.push(stage);

        self.stage_number += 1;

        self
    }

    pub fn morning_criterium(self, start: &Location, distance: f64) -> StageRaceBuilder {
        self.morning_road_stage(start, start, distance)
    }

    pub fn morning_out_and_back_prologue(mut self, start: &Location, distance: f64) -> StageRaceBuilder {
        let stage = Stage::prologue(self.date, start.clone(), start.clone(), distance);
        self.stages.push(stage);
        self
    }

    pub fn push(mut self, stg: Stage) -> StageRaceBuilder {
        let id = stage_id(self.stage_number, self.suffix, self.split_stages);
        let stage = match stg {
            Stage::IndividualTimeTrial { .. } => {
                let mut cols:Vec<(f64,Col)> = Vec::new();
                for (km, col) in stg.cols().iter() {
                    cols.push((*km, col.clone()));
                }
                Stage::itt(self.date, id, stg.start().clone(), Some(stg.finish().clone()), *stg.distance(), cols)
            },
            Stage::Road { .. } => {
                let mut cols:Vec<(f64,Col)> = Vec::new();
                for (km, col) in stg.cols().iter() {
                    cols.push((*km, col.clone()));
                }
                Stage::road(self.date, id, stg.start().clone(), Some(stg.finish().clone()), *stg.distance(), cols)
            },
            _ => panic!("Unexpected stage type"),
        };
        self.stages.push(stage);
        self.stage_number += 1;
        self.date = self.date.succ();

        self
    }
}
