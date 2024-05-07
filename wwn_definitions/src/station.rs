use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::{DataTable, Error, Filter};
use crate::Error::InvalidFilterFError;
use crate::location::Location;
use crate::mapping::{Nation, Origin, ParameterDefinitions, ParameterResolutionDefinitions, Regulation};
use crate::mapping::ParameterDefinitions::FLOW;
use crate::timeseries_metadata::TimeseriesMetaData;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct StationFilter {
    pub nations: Nation,
    pub include_activity_status_inactive: bool,
}
impl Filter for StationFilter{
}

#[derive(Debug, Clone,Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StationId(pub Origin, pub String);
impl StationId{
    pub fn from(origin: Origin, id: String)->Self{
        Self{ 0: origin, 1: id.to_ascii_lowercase().replace("-", "_").replace(".", "_")}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Station {
    pub id: StationId,
    pub source_id: String,
    pub status: bool,
    pub status_description: Vec<String>,
    pub river_name: Vec<String>,
    pub parental_hierarchy: Vec<String>,
    pub last_update: DateTime<Utc>,
    pub origin: Origin,
    pub measuring_authority_id: Option<String>,
    pub station_type: Option<String>,
    pub regulation_status: Regulation,
    pub station_parameters: Vec<StationParameterMetaData>,
    pub location: Location,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct StationMetaData{
    max_resolution_with_more_than_one_year: Vec<(ParameterDefinitions, ParameterResolutionDefinitions)>
}
impl DataTable for Station{
    type DataType = Station;
    type PrimaryId = StationId;
    type FilterType = StationFilter;
    type MetaDataType = StationMetaData;
    const ENDPOINT: &'static str = "/station";
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PartialOrd)]
pub struct StationParameterMetaData {
    pub parameter: ParameterDefinitions,
    pub resolutions: Vec<StationParameterResolution>,
    pub metadata: Option<TimeseriesMetaData>,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq,  PartialOrd)]
pub struct StationParameterResolution {
    pub source_resolution: String,
    pub internal_resolution: Option<i64>,
    pub resolution: ParameterResolutionDefinitions,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub method: String,
}