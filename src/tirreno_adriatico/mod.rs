use crate::col::ColCategory;
use crate::distance::Distance;
use crate::italy::*;
use crate::mountains::appennines::*;
use crate::stage_race::{StageRace,StageRaceBuilder};
use chrono::{TimeZone,Utc};

pub fn tirreno_adriatico_2013() -> StageRace {
    StageRaceBuilder::tirreno(Utc.ymd(2013,3,6))
        .prologue(&san_vincenzo(), &donoratico(), 16.9)
        .road_stage(&san_vincenzo(), &indicatore(), 232.0)
        .road_stage(&indicatore(), &narni_scalo(), 190.0)
        .road_stage(&narni_scalo(), &prati_di_tivo(), 173.0)
        .road_stage(&ortona(), &chieti(), 230.0)
        .criterium(&porto_sant_elpidio(), 209.0)
        .out_and_back_itt(&san_benedetto_del_tronto(), 9.2)
        .build()
}

pub fn tirreno_adriatico_2014() -> StageRace {
    StageRaceBuilder::tirreno(Utc.ymd(2014,3,12))
        .prologue(&donoratico(), &san_vincenzo(), 18.5)
        .road_stage(&san_vincenzo(), &cascina(), 166.0)
        .road_stage(&cascina(), &arezzo(), 212.0)
        .road_stage(&indicatore(), &cittareale(), 244.0)
        .road_stage(&amatrice(), &guardiagrele(), 192.0)
        .road_stage(&bucchianico(), &porto_sant_elpidio(), 193.0)
        .out_and_back_itt(&san_benedetto_del_tronto(), 9.1)
        .build()
}

pub fn tirreno_adriatico_2015() -> StageRace {
    StageRaceBuilder::tirreno(Utc.ymd(2015,3,11))
        .out_and_back_itt(&lido_di_camaiore(), 5.4)
        .road_stage(&camaiore(), &cascina(), 153.0)
        .road_stage(&cascina(), &arezzo(), 203.0)
        .road_stage(&indicatore(), &castelraimondo(), 226.0)
        .mountain_stage(&esanatoglia())
        .summit_finish(&monte_terminillo(), ColCategory::UC, Distance::new(196,9))
        .road_stage(&rieti(), &porto_sant_elpidio(), 210.0)
        .out_and_back_itt(&san_benedetto_del_tronto(), 10.1)
        .build()
}

pub fn tirreno_adriatico_2016() -> StageRace {
    StageRaceBuilder::tirreno(Utc.ymd(2016,3,9))
        .out_and_back_ttt(&lido_di_camaiore(), 22.7)
        .road_stage(&camaiore(), &pomarance(), 207.0)
        .road_stage(&castelnuouvo_di_val_di_cecina(), &montalto_di_castro(), 176.0)
        .road_stage(&montalto_di_castro(), &foligno(), 216.0)
        .mountain_stage(&foligno())
        .summit_finish(&monte_san_vicino(), ColCategory::UC, Distance::new(178,0))
        .road_stage(&castelraimondo(), &cepagatti(), 210.0)
        .out_and_back_itt(&san_benedetto_del_tronto(), 10.1)
        .build()
}
