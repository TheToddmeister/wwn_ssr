use itertools::Itertools;
use crate::location::{LocationFilter, LocationId};
use crate::mapping::*;
use crate::mapping::Nation::Norway;
use crate::mapping::Origin::{CANADA, NVE, UKGOV};
use crate::mapping::ParameterDefinitions::{FLOW, WATERLEVEL};
use crate::station::{StationFilter, StationId};
use crate::timeseries::TimeSeriesFilter;

pub static NVE_TEST_STATION_ID: [&str; 5]= [   "2_13_0", "2_39_0", "2_595_0", "2_661_0", "2_284_0"];
pub static  UK_TEST_STATION_ID: [&str; 5] = [    "052d0819_2a32_47df_9b99_c243c9c8235b", "0a8b3311_f905_40e3_9516_823b7eb8a946", "0b2ad98d_d131_4233_9edc_f57be0b4ae31", "127b5107_82fb_4e10_a15d_77ac7766422d","15eae722_7f1f_41cb_a93e_2acdeca10a0a",];
pub static CANADA_TEST_STATION_ID: [&str; 5] = ["02gh003","02ge009","02gh002","02gh016","02gh011"];
pub static RIVER_TEST_NAMES: [&str; 5] = ["Sjoa", "Etna", "Lysakerelva", "Lomma", "River Thames"];

pub fn test_station_strings() ->Vec<String>{
    let mut ids = vec![];
    ids.extend(UK_TEST_STATION_ID.clone());
    ids.extend(NVE_TEST_STATION_ID.clone());
    ids.into_iter().map(|a| a.to_string()).collect_vec()
}
pub fn test_station_ids()->Vec<StationId>{
    let mut ids = vec![];
    ids.extend(UK_TEST_STATION_ID.clone().into_iter().map(|a| StationId(UKGOV, a.to_string())).collect_vec());
    ids.extend(UK_TEST_STATION_ID.clone().into_iter().map(|a| StationId(NVE, a.to_string())).collect_vec());
    ids
}
pub fn test_location_ids()->Vec<LocationId>{
    CANADA_TEST_STATION_ID.into_iter().map(|a| LocationId(CANADA, a.to_string())).collect_vec()
}
pub fn test_river_names()->Vec<String>{
    RIVER_TEST_NAMES.into_iter().map(|a| a.to_string()).collect_vec()
}


impl StationFilter {
    pub fn test_default()->Self{

        Self{
            nations: Norway,
            include_activity_status_inactive: true,
        }
    }
}


impl Default for StationFilter {
    fn default() -> Self {
        Self {
            nations:Norway,
            include_activity_status_inactive: true,
        }
    }
}

impl LocationFilter {
    pub fn test_default()->Self{
        Self{
            origin: vec![CANADA],
            nation: vec![],
            loc_type: vec![],
        }
    }
}

impl Default for LocationFilter {
    fn default() -> Self {
        Self {
            origin: vec![],
            nation: vec![Norway],
            loc_type: vec![],
        }
    }
}

impl TimeSeriesFilter{
    pub fn test_default()->Self{

        Self{
            station_id: test_station_ids(),
            origin: Default::default(),
            resolution: None,
            parameters: vec![],
        }
    }
}

impl Default for TimeSeriesFilter {
    fn default() -> Self {
        Self {
            station_id: vec![],
            origin: Default::default(),
            resolution: None,
            parameters: vec![FLOW],
        }
    }
}

impl Default for Origin {
    fn default() -> Self {
        Self::NVE
    }
}

impl Default for Nation {
    fn default() -> Self {
        Self::Norway
    }
}

impl Default for Regulation {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

impl Default for ParameterDefinitions {
    fn default() -> Self {
        Self::FLOW
    }
}

impl Default for ParameterResolutionDefinitions {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

impl Default for Quality {
    fn default() -> Self {
        Self::UNKNOWN
    }
}