use crate::col::{Col,ColCategory};
use crate::distance::Distance;
use crate::location::Location;
use crate::stage_race::Stage;
use chrono::{TimeZone,Utc};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, Default)]
pub struct Yes;
#[derive(Debug, Default)]
pub struct No;

pub trait ToAssign: Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

enum StageType {
    ITT,
    Road
}

pub struct StageBuilder<StageStartSet, StageFinishSet, StageLengthSet, StageTypeSet>
where
    StageStartSet: ToAssign,
    StageFinishSet: ToAssign,
    StageLengthSet: ToAssign,
    StageTypeSet: ToAssign
{
    start_set: PhantomData<StageStartSet>,
    finish_set: PhantomData<StageFinishSet>,
    length_set: PhantomData<StageLengthSet>,
    type_set: PhantomData<StageTypeSet>,

    start: Location,
    finish: Option<Location>,
    length: f64,
    stage_type: StageType,
    cols: BTreeMap<Distance,Col>
}

impl<StageStartSet, StageFinishSet, StageLengthSet, StageTypeSet>
    StageBuilder<StageStartSet, StageFinishSet, StageLengthSet, StageTypeSet>
where
    StageStartSet: ToAssign,
    StageFinishSet: ToAssign,
    StageLengthSet: ToAssign,
    StageTypeSet: ToAssign
{
    pub fn c1_col(self, km: Distance, loc: Location) -> StageBuilder<StageStartSet, StageFinishSet, StageLengthSet, StageTypeSet> {
        self.col(km, ColCategory::C1, loc)
    }

    pub fn c2_col(self, km: Distance, loc: Location) -> StageBuilder<StageStartSet, StageFinishSet, StageLengthSet, StageTypeSet> {
        self.col(km, ColCategory::C2, loc)
    }

    pub fn c3_col(self, km: Distance, loc: Location) -> StageBuilder<StageStartSet, StageFinishSet, StageLengthSet, StageTypeSet> {
        self.col(km, ColCategory::C3, loc)
    }

    pub fn c4_col(self, km: Distance, loc: Location) -> StageBuilder<StageStartSet, StageFinishSet, StageLengthSet, StageTypeSet> {
        self.col(km, ColCategory::C4, loc)
    }

    pub fn col(self, km: Distance, category: ColCategory, loc: Location) -> StageBuilder<StageStartSet, StageFinishSet, StageLengthSet, StageTypeSet> {
        let c = Col::new(loc.clone(), category, None, None, None);
        let mut stage_cols = self.cols;
        stage_cols.insert(km, c);
        StageBuilder {
            start_set: PhantomData {},
            finish_set: PhantomData {},
            length_set: PhantomData {},
            type_set: PhantomData {},
            start: self.start,
            finish: self.finish,
            length: self.length,
            stage_type: self.stage_type,
            cols: stage_cols
        }
    }

    pub fn finish(self, finish: Location) -> StageBuilder<StageStartSet, Yes, StageLengthSet, StageTypeSet> {
        StageBuilder {
            start_set: PhantomData {},
            finish_set: PhantomData {},
            length_set: PhantomData {},
            type_set: PhantomData {},
            start: self.start,
            finish: Some(finish),
            length: self.length,
            stage_type: self.stage_type,
            cols: self.cols
        }
    }
}

impl StageBuilder<Yes, Yes, Yes, Yes> {
    pub fn build(&self) -> Stage {
        let finish = self.finish.as_ref().unwrap().clone();
        let date = Utc.ymd(1972,1,1);
        let id = String::from("1");
        match self.stage_type {
            StageType::ITT  => {
                let cols = self.btreemap_cols_to_vec_cols();
                Stage::itt(date, id, self.start.clone(), Some(finish), self.length, cols)
            },
            StageType::Road => {
                let cols = self.btreemap_cols_to_vec_cols();
                Stage::road(date, id, self.start.clone(), Some(finish), self.length, cols)
            }
        }
    }

    fn btreemap_cols_to_vec_cols(&self) -> Vec<(f64,Col)> {
        let mut cols:Vec<(f64,Col)> = Vec::new();
        for (km, col) in self.cols.iter() {
            let km_f64:f64 = format!("{}.{}", km.integral(), km.fractional()).parse().unwrap();
            let c = Col::new(col.location().clone(), col.category(), col.length(),
                col.average_gradient(), col.maximum_gradient());
            cols.push((km_f64,c));
        }
        cols
    }
}

impl StageBuilder<No, No, No, No> {
    pub fn individual_time_trial(start: Location, dist: f64) -> StageBuilder<Yes, No, Yes, Yes> {
        StageBuilder {
            start_set: PhantomData {},
            finish_set: PhantomData {},
            length_set: PhantomData {},
            type_set: PhantomData {},
            start: start,
            finish: None,
            length: dist,
            stage_type: StageType::ITT,
            cols: BTreeMap::new(),
        }
    }

    pub fn road_stage(start: Location, dist: f64) -> StageBuilder<Yes, No, Yes, Yes> {
        StageBuilder {
            start_set: PhantomData {},
            finish_set: PhantomData {},
            length_set: PhantomData {},
            type_set: PhantomData {},
            start: start,
            finish: None,
            length: dist,
            stage_type: StageType::Road,
            cols: BTreeMap::new()
        }
    }
}
