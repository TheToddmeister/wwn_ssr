use serde::{Deserialize, Serialize};
use strum::Display;
use crate::{DataTable, Error, Filter};
use crate::Error::InvalidFilterFError;
use crate::mapping::{Nation, Origin};
use crate::station::StationId;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocationFilter {
    pub origin: Vec<Origin>,
    pub nation: Vec<Nation>,
    pub loc_type: Vec<LocationType>,
}

impl Filter for LocationFilter {
}

#[derive(Debug, Clone, Hash, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct LocationId(pub Origin, pub String);

impl LocationId {
    pub fn from(origin: Origin, id: String) -> Self {
        Self { 0: origin, 1: id.to_ascii_lowercase().replace("-", "_").replace(".", "_") }
    }
}

#[derive(Debug, Clone, Hash, Deserialize, Serialize, Display, Eq, PartialEq, Ord, PartialOrd)]
pub enum LocationType {
    MeasuringStation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub id: LocationId,
    pub name: String,
    pub description: String,
    pub coordinates: (f64, f64),
    pub location_type: LocationType,
    pub origin: Origin,
    pub source: String,
    pub source_id: String,
    pub nation: Nation,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct LocationMetaData {}

impl DataTable for Location {
    type DataType = Location;
    type PrimaryId = LocationId;
    type FilterType = LocationFilter;
    type MetaDataType = LocationMetaData;
    const ENDPOINT: &'static str = "/location";
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Position {
    pub name: String,
    pub description: String,
    pub coordinate: Coordinates,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Coordinates {
    pub longitude: f64,
    pub latitude: f64,
}