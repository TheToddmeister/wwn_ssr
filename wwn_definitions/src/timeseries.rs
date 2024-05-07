use std::collections::BTreeMap;

use chrono::{Datelike, DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};

use crate::{DataTable, Error, Filter};
use crate::Error::InvalidFilterFError;
use crate::mapping::{Nation, Origin, ParameterDefinitions, ParameterResolutionDefinitions, Quality};
use crate::mapping::Origin::NVE;
use crate::mapping::ParameterDefinitions::FLOW;
use crate::mapping::Quality::*;
use crate::station::StationId;
use crate::timeseries_metadata::{OrdinateTimeSeriesMetadata};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Observation {
    pub datetime: DateTime<Utc>,
    pub value: Option<f64>,
    pub quality: Quality,
}

impl Observation {
    pub fn is_good_quality(&self) -> bool {
        match self.quality {
            UNKNOWN => false,
            UNCONTROLLED => false,
            UNAVAILABLE => false,
            ESTIMATED => true,
            GOOD => true,
            CONTROLLED => true,
            SUSPECT => false,
            NOTPROVIDED => true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ValidatedObservation {
    pub datetime: DateTime<Utc>,
    pub value: f64,
    pub quality: Quality,
}

impl ValidatedObservation {
    pub fn from_observation(observation: &Observation) -> Option<Self> {
        if let Some(value) = observation.value {
            let obs = Self {
                datetime: observation.datetime.to_owned(),
                value,
                quality: observation.quality.to_owned(),
            };
            return Some(obs);
        }
        None
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct TimeSeriesId(pub StationId, pub ParameterDefinitions, pub ParameterResolutionDefinitions);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct TimeSeries {
    pub timseries_id: TimeSeriesId,
    pub observations: Vec<Observation>,
    pub last_update_request: DateTime<Utc>,
}




impl DataTable for TimeSeries {
    type DataType = TimeSeries;
    type PrimaryId = TimeSeriesId;
    type FilterType = TimeSeriesFilter;
    type MetaDataType = OrdinateTimeSeriesMetadata;
    const ENDPOINT: &'static str = "/timeseries";
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesFilter {
    pub station_id: Vec<StationId>,
    pub origin: Origin,
    pub resolution: Option<ParameterResolutionDefinitions>,
    pub parameters: Vec<ParameterDefinitions>,
}

impl Filter for TimeSeriesFilter {
}
impl TimeSeriesFilter{
    fn is_valid(&self) -> Result<(), Error> {
        match self.station_id.is_empty(){
            true => Ok(()),
            false => Err(InvalidFilterFError)
        }
    }
}
