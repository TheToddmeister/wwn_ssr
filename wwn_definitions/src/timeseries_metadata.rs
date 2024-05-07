use std::collections::BTreeMap;

use chrono::{Datelike, DateTime, Duration, NaiveDate, Timelike, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::timeseries::{Observation, TimeSeriesId, ValidatedObservation};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct TimeSeriesData {
    pub timeseries_id: TimeSeriesId,
    pub delta: Option<TimeSeriesDelta>,
    pub metadata: Option<TimeseriesMetaData>,
    pub data: Vec<Observation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct TimeSeriesDelta {
    pub change_min24h: Option<(f64, DateTime<Utc>)>,
    pub change_min6h: Option<(f64, DateTime<Utc>)>,
    pub change_min1h: Option<(f64, DateTime<Utc>)>,
}

pub fn calculate_delta(a: &ValidatedObservation, b: &ValidatedObservation, target_resolution_in_minutes: u32) -> f64 {
    let time_delta = a.datetime - b.datetime;
    (a.value - a.value) / time_delta.num_minutes() as f64 * target_resolution_in_minutes as f64
}

impl TimeSeriesDelta {
    pub fn from_observations(observation: &[Observation]) -> Option<Self> {
        let now = Utc::now();
        let most_recent_observation = observation.iter()
            .find(|p| p.datetime <= now - Duration::try_hours(4).unwrap() && p.value.is_some() && p.is_good_quality())
            .and_then(|v| ValidatedObservation::from_observation(v));

        if let Some(recent) = most_recent_observation {
            let h1 = now - Duration::try_hours(1).unwrap();
            let change_min1h = observation.iter()
                .find(|a| h1 <= a.datetime && a.datetime <= h1 - Duration::try_minutes(30).unwrap())
                .and_then(|b| ValidatedObservation::from_observation(b))
                .and_then(|c|
                    Some((calculate_delta(&recent, &c, 60), c.datetime))
                );

            let h6 = now - Duration::try_hours(6).unwrap();
            let change_min6h = observation.iter()
                .find(|a| h6 <= a.datetime && a.datetime <= h6 - Duration::try_minutes(30).unwrap())
                .and_then(|b| ValidatedObservation::from_observation(b))
                .and_then(|c|
                    Some((calculate_delta(&recent, &c, 60), c.datetime))
                );

            let h24 = now - Duration::try_hours(24).unwrap();
            let change_min24h = observation.iter()
                .find(|a| h24 <= a.datetime && a.datetime <= h24 - Duration::try_minutes(30).unwrap())
                .and_then(|b| ValidatedObservation::from_observation(b))
                .and_then(|c|
                    Some((calculate_delta(&recent, &c, 60), c.datetime))
                );
            let meta = Self {
                change_min24h,
                change_min6h,
                change_min1h,
            };
            return Some(meta);
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct TimeseriesMetaData {
    pub timeseries_id: TimeSeriesId,
    pub ordinate_calculations: Vec<OrdinateToMetaData>,
    pub last_year_observations: Vec<OrdinateToValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct OrdinateToMetaData {
    pub day: u32,
    pub hour: u32,
    pub metdata: OrdinateTimeSeriesMetadata,
}

impl OrdinateToMetaData {
    pub fn from(map: BTreeMap<(u32, u32), OrdinateTimeSeriesMetadata>) -> Vec<OrdinateToMetaData> {
        map.into_iter().map(|record| {
            let (ordinal, ordinate) = record.0;
            Self {
                day: ordinal,
                hour: ordinate,
                metdata: record.1,
            }
        }).collect_vec()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct OrdinateToValue {
    pub day: u32,
    pub hour: u32,
    pub value: f64,
}

impl OrdinateToValue {
    pub fn from(map: BTreeMap<(u32, u32, u32), f64>) -> Vec<OrdinateToValue> {
        map.into_iter().map(|record| {
            let (_year,ordinal, ordinate) = record.0;
            Self {
                day: ordinal,
                hour: ordinate,
                value: record.1,
            }
        }).collect_vec()
    }
}




#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct OrdinateTimeSeriesMetadata {
    pub count: u32,
    pub mean: f64,
    pub max: Option<f64>,
    pub min: Option<f64>,
}

impl TimeseriesMetaData {
    pub fn calculate_metadata_from_observations(observations: &[Observation],
                                                timeseries_id: TimeSeriesId) -> TimeseriesMetaData {
        let hourly_dataset = create_a_hourly_dataset(observations);
        let last_year_map = extract_last_year_observations(&hourly_dataset);
        let last_year_observations = OrdinateToValue::from(last_year_map);
        let hourly_means = calculate_hourly_mean(hourly_dataset);
        let ordinate_calculation_map = ordinate_calulations(hourly_means);
        let ordinate_calculations = OrdinateToMetaData::from(ordinate_calculation_map);
        Self {
            timeseries_id,
            ordinate_calculations,
            last_year_observations,

        }
    }
}

pub fn extract_last_year_observations(hourly_dataset: &BTreeMap<(u32, u32, u32), Vec<ValidatedObservation>>)
                                      -> BTreeMap<(u32, u32, u32), f64> {
    let last_year = Utc::now().year() as u32 - 1;
    let mut ordinate_to_mean_data = BTreeMap::new();

    for key in hourly_dataset.keys() {
        let k = key.to_owned();
        if k.0 == last_year {
            let data = hourly_dataset.get(key);
            if let Some(d) = data {
                let s = d.iter().map(|v| v.value).sum::<f64>();
                let v = d.iter().count() as f64;
                ordinate_to_mean_data.insert(key.to_owned(), s / v);
            }
        }
    }
    ordinate_to_mean_data
}

/// SPLIT INTO ORDINATES 1..365 * 24
/// REMOVE OBSERVATION RECORDS WITH "BAD" ASSESSMENTS FROM SOURCE.
/// REMOVE OBSERVATION RECORDS WITHOUT VALUE
pub fn create_a_hourly_dataset(observations: &[Observation]) -> BTreeMap<(u32, u32, u32), Vec<ValidatedObservation>> {
    let mut ordinal_observations = BTreeMap::new();
    for obs in observations {
        if obs.is_good_quality() {
            if let Some(o) = ValidatedObservation::from_observation(obs) {
                let year = obs.datetime.year() as u32;
                let ordinal = obs.datetime.ordinal();
                let hour = obs.datetime.hour();
                ordinal_observations.entry((year, ordinal, hour)).or_insert_with(Vec::new).push(o)
            }
        }
    }
    ordinal_observations
}

/// THERE MAY BE MORE THAN ONE OBSERVATION PER HOUR DEPENDING ON THE DATESET
/// CALCULATE THE MEAN OF EACH INDICIDUAL HOUR
/// ADD EACH CALCULATION TO A VEC OF VALUES FROM THE SAME ORDINATE HOUR
pub fn calculate_hourly_mean(ordinal_dataset: BTreeMap<(u32, u32, u32), Vec<ValidatedObservation>>) -> BTreeMap<(u32, u32), Vec<f64>> {
    let mut means = BTreeMap::new();
    for time_to_data in ordinal_dataset {
        let hourly_data = time_to_data.1.iter().map(|f| f.value).collect_vec();
        let hourly_mean = hourly_data.iter().sum::<f64>() / time_to_data.1.len() as f64;
        means.entry((time_to_data.0.1, time_to_data.0.2)).or_insert_with(Vec::new).push(hourly_mean)
    }
    means
}

/// CALCULATE ON THE ORDINATE HOURS
pub fn ordinate_calulations(ordinate_dataset: BTreeMap<(u32, u32), Vec<f64>>) -> BTreeMap<(u32, u32), OrdinateTimeSeriesMetadata> {
    let mut daily_calculation = BTreeMap::new();
    for time_to_data in ordinate_dataset {
        let data = time_to_data.1;
        let count = data.len() as u32;
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let max = data.iter().max_by(|a, b| f64::total_cmp(a, b)).map(|a| a.to_owned());
        let min = data.iter().min_by(|a, b| f64::total_cmp(a, b)).map(|a| a.to_owned());
        let calc = OrdinateTimeSeriesMetadata {
            count,
            mean,
            max,
            min,
        };
        daily_calculation.insert(time_to_data.0, calc);
    }
    daily_calculation
}

pub fn provide_a_negative_span_from_start_of_the_year(days: i64) -> (DateTime<Utc>, DateTime<Utc>) {
    let y = Utc::now().year();
    let this_year_start = NaiveDate::from_ymd_opt(y, 1, 1)
        .expect("Failed to create an appropriate date from the provided year")
        .and_hms_opt(0, 0, 1)
        .unwrap().and_utc();
    (
        this_year_start,
        this_year_start - Duration::try_days(days).expect("Failed to substract a year from the proided date")
    )
}