use crate::instance::edge::data::{EdgeDataMatrixSym, RestrictedDataMatrixSym};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Distance(pub i32);

pub type DistanceMatrixSym = EdgeDataMatrixSym<Distance>;
pub type RestrictedDistanceMatrixSym<'a> = RestrictedDataMatrixSym<'a, Distance>;
