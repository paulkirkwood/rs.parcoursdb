use crate::col::{Col,ColCategory};
use crate::france::alpe_d_huez as other_alpe_d_huez;

pub fn alpe_d_huez() -> Col {
    Col::new(other_alpe_d_huez().clone(), ColCategory::HC, Some(13.8), Some(8.1), None)
}
