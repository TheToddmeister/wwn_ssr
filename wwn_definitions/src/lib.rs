use std::fmt::Debug;
use std::hash::Hash;
use chrono::{DateTime, Duration, Utc};
use serde::{de, Deserialize, Serialize};
use serde::de::DeserializeOwned;
use strum::Display;
use crate::mapping::{Nation, Origin, ParameterDefinitions, Regulation};
use thiserror::Error;

pub mod timeseries;
pub mod mapping;
pub mod location;
pub mod station;
pub mod timeseries_metadata;
pub mod utils;
pub mod defaults;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Display)]
pub enum ServiceStatus {
    Good,
    Compromised(String),
    Unavailable,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Status {
    version: String,
    service: ServiceStatus,
}

pub struct Data<T> where
    T: DataTable{
    service: Status,
    data: T,
    metadata: T::MetaDataType,
}

pub trait DataTable: Serialize + DeserializeOwned + Send + Sync + Clone + Debug {
    type DataType: DataTable + Serialize + DeserializeOwned + Send + Sync + Clone + Debug;
    type PrimaryId: Serialize + DeserializeOwned + Send + Sync + Clone + Debug;
    type FilterType: Filter + Serialize + DeserializeOwned + Send + Sync + Default + Clone + Eq + PartialEq + Debug + Hash;
    type MetaDataType: Serialize + DeserializeOwned + Send + Sync;
    const ENDPOINT: &'static str;
}

pub trait Filter {
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("The data observation type provided by the data source doesnt exist internally for the provided data origin -> {0} + {1}")]
    InternalMappingError(Origin, String),
    #[error("The provided data observation type doesnt provide an external defintion for the data origin -> {0} + {1}")]
    ExternalMappingError(Origin, ParameterDefinitions),
    #[error("An empty filter is not valid. For all data, input the all parameters")]
    InvalidFilterFError,

}

