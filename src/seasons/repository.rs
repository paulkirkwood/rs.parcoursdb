use crate::amstel_gold_race::repository::all     as amstel_editions;
use crate::e3_harelbeke::repository::all         as e3_editions;
use crate::gent_wevelgem::repository::all        as gw_editions;
use crate::la_fleche_wallonne::repository::all   as fleche_editions;
use crate::liege_bastogne_liege::repository::all as lbl_editions;
use crate::omloop_het_volk::repository::all      as omloop_editions;
use crate::paris_roubaix::repository::all        as paris_roubaix_editions;
use crate::paris_tours::repository::all          as paris_tours_editions;
use crate::tour_of_flanders::repository::all     as flanders_editions;
use crate::tour_of_lombardy::repository::all     as lombardy_editions;
use crate::tirreno_adriatico::repository::all    as tirreno_editions;
use crate::tour_de_france::repository::all       as tdf_editions;
use crate::tour_of_italy::repository::all        as giro_editions;
use crate::tour_of_spain::repository::all        as vuelta_editions;
use crate::classic::Classic;
use std::collections::BTreeSet;
use chrono::Datelike;

pub fn all() -> Vec<i32> {
    let mut years:BTreeSet<i32> = BTreeSet::new();

    for edition in amstel_editions() {
        years.insert(edition.date().year());
    }

    for edition in e3_editions() {
        years.insert(edition.date().year());
    }

    for edition in gw_editions() {
        years.insert(edition.date().year());
    }

    for edition in fleche_editions() {
        years.insert(edition.date().year());
    }

    for edition in lbl_editions() {
        years.insert(edition.date().year());
    }

    for edition in omloop_editions() {
        years.insert(edition.date().year());
    }

    for edition in paris_roubaix_editions() {
        years.insert(edition.date().year());
    }

    for edition in paris_tours_editions() {
        years.insert(edition.date().year());
    }

    for edition in flanders_editions() {
        years.insert(edition.date().year());
    }

    for edition in lombardy_editions() {
        years.insert(edition.date().year());
    }

    for edition in tdf_editions() {
        years.insert(edition.start_date().year());
    }

    for edition in giro_editions() {
        years.insert(edition.start_date().year());
    }

    for edition in vuelta_editions() {
        years.insert(edition.start_date().year());
    }

    for edition in tirreno_editions() {
        years.insert(edition.start_date().year());
    }

    let seasons:Vec<i32> = years.into_iter().collect();
    seasons.to_vec()
}
