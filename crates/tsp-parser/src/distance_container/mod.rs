use tsp_core::instance::{InstanceMetadata, distance::Distance};

use crate::data_section::{GeoPoint, Point2D};

mod matrix_sym;

pub trait ParseFromTSPLib {
    fn from_node_coord_section<PointType: Sync + Send>(
        node_data: &Vec<PointType>,
        metadata: &InstanceMetadata,
        distance_function: impl Fn(&PointType, &PointType) -> Distance + Sync + Send + Copy,
    ) -> Self;
}
