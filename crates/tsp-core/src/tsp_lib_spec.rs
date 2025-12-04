#![allow(non_camel_case_types)]

/// Enumeration of all possible data section keywords in a .tsp file.
///
/// The Keywords are according to the TSPLIB 95 specification.
#[derive(Debug, Clone)]
pub enum TSPDataKeyword {
    NODE_COORD_SECTION,
    DEPOT_SECTION,
    DEMAND_SECTION,
    EDGE_DATA_SECTION,
    FIXED_EDGES_SECTION,
    DISPLAY_DATA_SECTION,
    TOUR_SECTION,
    EDGE_WEIGHT_SECTION,
}

/// Enumeration of all possible keywords in the specification part
/// of a .tsp file.
///
/// The Keywords are according to the TSPLIB 95 specification.
#[derive(Debug, Clone)]
pub enum TSPSpecificationKeyword {
    NAME(String),
    TYPE(ProblemType),
    COMMENT(String),
    DIMENSION(u32),
    CAPACITY(u32),
    EDGE_WEIGHT_TYPE(EdgeWeightType),
    EDGE_WEIGHT_FORMAT(EdgeWeightFormat),
    EDGE_DATA_FORMAT(EdgeDataFormat),
    NODE_COORD_TYPE(NodeCoordType),
    DISPLAY_DATA_TYPE(DisplayDataType),
    EOF,
}

#[derive(Debug, Clone)]
pub enum ProblemType {
    TSP,
    ATSP,
    SOP,
    HCP,
    CVRP,
    TOUR,
}

#[derive(Debug, Clone)]
pub enum EdgeWeightType {
    EXPLICIT,
    EUC_2D,
    EUC_3D,
    MAX_2D,
    MAX_3D,
    MAN_2D,
    MAN_3D,
    CEIL_2D,
    GEO,
    ATT,
    XRAY1,
    XRAY2,
    SPECIAL,
}

#[derive(Debug, Clone)]
pub enum EdgeWeightFormat {
    FUNCTION,
    FULL_MATRIX,
    UPPER_ROW,
    LOWER_ROW,
    UPPER_DIAG_ROW,
    LOWER_DIAG_ROW,
    UPPER_COL,
    LOWER_COL,
    UPPER_DIAG_COL,
    LOWER_DIAG_COL,
}

#[derive(Debug, Clone)]
pub enum EdgeDataFormat {
    EDGE_LIST,
    ADJ_LIST,
}

#[derive(Debug, Clone)]
pub enum NodeCoordType {
    TWOD_COORDS,
    THREED_COORDS,
    NO_COORDS,
}

#[derive(Debug, Clone)]
pub enum DisplayDataType {
    COORD_DISPLAY,
    TWOD_DISPLAY,
    NO_DISPLAY,
}
