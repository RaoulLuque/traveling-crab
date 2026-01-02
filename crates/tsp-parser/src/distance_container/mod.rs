use tsp_core::instance::{InstanceMetadata, distance::Distance};

use crate::data_section::Point2D;

mod matrix_sym;

pub trait ParseFromTSPLib {
    fn from_node_coord_section(
        node_data: &Vec<Point2D>,
        metadata: &InstanceMetadata,
        distance_function: impl Fn(&Point2D, &Point2D) -> Distance + Sync + Send + Copy,
    ) -> Self;
}
