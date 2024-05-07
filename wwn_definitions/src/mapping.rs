use std::str::FromStr;
use phf::phf_map;

use serde::{Deserialize, Serialize};
use strum::Display;
use log::info;
use crate::Error;
use crate::Error::{ExternalMappingError, InternalMappingError};
use crate::mapping::Nation::*;
use crate::mapping::Origin::*;

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, Display, Eq, PartialEq, Ord, PartialOrd)]
pub enum ParameterResolutionDefinitions {
    INSTANT,
    QUARTERLY,
    HOURLY,
    DAILY,
    UNKNOWN,
}

impl ParameterResolutionDefinitions {
    pub fn from_nve(nve_resolution_unit: &i64) -> Self {
        match nve_resolution_unit {
            0 => Self::INSTANT,
            60 => Self::HOURLY,
            1440 => Self::DAILY,
            _ => Self::UNKNOWN,
        }
    }
    pub fn to_nve(&self) -> Option<i64> {
        match self {
            Self::INSTANT => Some(0),
            Self::QUARTERLY => None,
            Self::HOURLY => Some(60),
            Self::DAILY => Some(1440),
            Self::UNKNOWN => None
        }
    }
    pub fn from_uk_to_internal(uk_resolution_unit: &Option<i64>) -> Option<i64> {
        match uk_resolution_unit {
            Some(86400)  => Some(0),
            Some(900) => Some(15),
            Some(1) => Some(1440),
            Some(_) => {info!("Found an unmapped UKGOV resolution {:?}", uk_resolution_unit ); None}
            _ => None,
        }
    }
    pub fn from_uk(uk_resolution_unit: &Option<i64>) -> Self {
        match uk_resolution_unit {
            Some(86400)  => Self::INSTANT,
            Some(900) => Self::QUARTERLY,
            Some(1) => Self::DAILY,
            _ => Self::UNKNOWN,
        }
    }
    pub fn to_uk(&self) -> Option<i64> {
        match self {
            Self::INSTANT => Some(0),
            Self::QUARTERLY => Some(15),
            Self::HOURLY => Some(60),
            Self::DAILY => Some(1440),
            Self::UNKNOWN => None
        }
    }
    pub fn to_uk_external_period_definition(&self) -> Option<i64> {
        match self {
            Self::INSTANT => Some(0),
            Self::QUARTERLY => Some(900),
            Self::HOURLY => None,
            Self::DAILY => Some(86400),
            Self::UNKNOWN => None
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Display, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ParameterDefinitions {
    FLOW,
    WATERLEVEL,
    TEMPERATURE,
    RAINFALL,
}

impl ParameterDefinitions {
    pub fn from_nve(s: &i64) -> Result<Self, crate::Error> {
        match s {
            1001 => Ok(Self::FLOW),
            1000 => Ok(Self::WATERLEVEL),
            1003 => Ok(Self::TEMPERATURE),
            0 => Ok(Self::RAINFALL),
            _ => Err(Error::from(InternalMappingError(NVE, s.to_string()))),
        }
    }
    pub fn to_nve(&self) -> &'static str {
        match self {
            Self::FLOW => "1001",
            Self::WATERLEVEL => "1000",
            Self::TEMPERATURE => "1003",
            Self::RAINFALL => "0",
        }
    }
    pub fn from_uk(s: &str) -> Result<Self, Error> {
        match s {
            "flow" => Ok(Self::FLOW),
            "level" => Ok(Self::WATERLEVEL),
            "temperature" => Ok(Self::TEMPERATURE),
            "rainfall" => Ok(Self::RAINFALL),
            _ => Err(Error::from(InternalMappingError(UKGOV, s.to_string()))),
        }
    }
    pub fn to_uk(&self) -> &'static str {
        match self {
            Self::FLOW => "flow",
            Self::WATERLEVEL => "level",
            Self::TEMPERATURE => "temperature",
            Self::RAINFALL => "rainfall",
        }
    }
    pub fn from_smih_name(s: &str) -> Result<Self, Error> {
        match s {
            "Vattenföring (15 min)" => Ok(Self::FLOW),
            "Vattenstånd" => Ok(Self::WATERLEVEL),
            "Vattendragstemperatur" => Ok(Self::TEMPERATURE),
            _ => Err(InternalMappingError(SMIH, s.to_string())),
        }
    }
    pub fn to_smih_id(&self) -> Result<i32, Error> {
        match self {
            Self::FLOW => Ok(2),
            Self::WATERLEVEL => Ok(3),
            Self::TEMPERATURE => Ok(4),
            Self::RAINFALL => Err(Error::from(ExternalMappingError(SMIH, Self::RAINFALL))),
        }
    }
    pub fn to_canada_id(&self) -> Result<&'static str, crate::Error> {
        match self {
            Self::FLOW => Ok("flow"),
            Self::WATERLEVEL => Ok("primaryLevel"),
            Self::TEMPERATURE => Ok("temperature"),
            Self::RAINFALL => Err(Error::from(ExternalMappingError(CANADA, Self::RAINFALL))),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, Display, PartialEq, Eq, Ord, PartialOrd)]
pub enum Origin {
    NVE,
    SMIH,
    UKGOV,
    CANADA,
}

impl Origin {
    pub fn to_nation(&self) -> Nation {
        match self {
            NVE => Norway,
            SMIH => Sweden,
            UKGOV => Uk,
            CANADA => Canada,
        }
    }
}


#[derive(Debug, Copy, Clone, Serialize, Deserialize, Display, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Regulation {
    REGULATED,
    UNKNOWN,
    UNREGULATED,
    NOTDOWNLOADED,
}

pub static EXTERNAL_TO_INTERNAL_REGULATION: phf::Map<&'static str, Regulation> = phf_map! {
    "regulert m/magasinregulering" => Regulation::REGULATED,
    "Regulert m/magasinregulering og overføringer" => Regulation::REGULATED,
    "regulert m/magasin og uregulert restfelt" => Regulation::REGULATED,
    "" => Regulation::UNKNOWN,
    "Uregulert" => Regulation::UNREGULATED,
    "Regulated" => Regulation::REGULATED,
    "Unregulated" => Regulation::UNREGULATED,
};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Display, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Nation {
    Norway,
    Sweden,
    Uk,
    Canada,
}

impl Nation {
    pub fn to_origin(&self) -> &'static [Origin] {
        match self {
            Norway => &[NVE],
            Sweden => &[SMIH],
            Uk => &[UKGOV],
            Canada => &[CANADA],
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Display, Eq, PartialEq, Ord, PartialOrd)]
pub enum Quality {
    UNKNOWN,
    UNCONTROLLED,
    UNAVAILABLE,
    ESTIMATED,
    GOOD,
    CONTROLLED,
    SUSPECT,
    NOTPROVIDED
}

impl Quality {
    pub fn from_nve(quality: &i64) -> Self {
        match quality {
            0 => Self::UNKNOWN,
            1 => Self::UNCONTROLLED,
            2 => Self::CONTROLLED,
            3 => Self::CONTROLLED,
            _ => Self::UNKNOWN,
        }
    }
    pub fn from_uk(quality: &str) -> Self {
        match quality {
            "Good" => Self::GOOD,
            "Estimated" => Self::ESTIMATED,
            "Suspect" => Self::SUSPECT,
            "Unchecked" => Self::UNCONTROLLED,
            "Missing" => Self::UNAVAILABLE,
            _ => Self::UNKNOWN,
        }
    }
}

pub enum Method{
    Qualified,
    Instantaneous,
    Minimum,
    Maximum,
}